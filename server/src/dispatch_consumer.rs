use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::time::Duration;

use actix_web::web::Data;
use futures_lite::StreamExt;
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use lapin::Channel;
use log::{debug, error, info};

use crate::amqp::{Publisher, Serializer, SENT_MESSAGE_QUEUE};
use crate::circuit_breaker::{CircuitBreaker, Error};
use crate::cmd::AsyncMessage;
use crate::configuration::domain::Endpoint;
use crate::retry::RetryPolicyBuilder;
use crate::storage::Storage;

pub async fn consume(channel: Channel, consumer_tag: &str, storage: Data<Storage>) {
    let retry_policy = RetryPolicyBuilder::new()
        .max_retries(5)
        .exponential(2, Duration::from_secs(2))
        .randomize(0.5)
        .build()
        .unwrap();

    let mut circuit_breaker = CircuitBreaker::new();

    let mut consumer = channel
        .basic_consume(
            SENT_MESSAGE_QUEUE,
            consumer_tag,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    let publisher = Publisher::new(channel);

    info!("consumer is ready");

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("error in consumer");
        let async_msg: AsyncMessage = Serializer::deserialize(&delivery.data);

        let AsyncMessage::SentMessage(cmd) = async_msg;

        info!("message consumed: {:?}", cmd);

        // todo allow to revive endpoint, check endpoint status and reset circuit breaker

        let msg = storage.messages.get(cmd.msg_id());
        if msg.is_err() {
            error!(
                "Message {} doesn't not exists and cannot be dispatched",
                cmd.msg_id()
            );

            delivery.ack(BasicAckOptions::default()).await.expect("ack");

            continue;
        }

        let endpoint_id = cmd.endpoint_id();
        let endpoint = storage.endpoints.get(&endpoint_id);
        if endpoint.is_err() {
            error!(
                "Endpoint {} doesn't not exists and message {} cannot be dispatched",
                endpoint_id,
                cmd.msg_id()
            );

            delivery.ack(BasicAckOptions::default()).await.expect("ack");

            continue;
        }

        let msg = msg.unwrap();
        let endpoint = Rc::new(RefCell::new(endpoint.unwrap()));
        let endpoint_borrowed = endpoint.borrow().to_owned();

        debug!(
            "Message {} for endpoint {} is being prepared to send",
            msg.id.to_string(),
            endpoint_borrowed.id.to_string()
        );

        let func = || {
            reqwest::Client::new()
                .post(endpoint_borrowed.url)
                .json(msg.payload.to_string().as_str())
                .send()
        };

        let key = endpoint_id.to_string();

        match circuit_breaker.call(&key, func).await {
            Ok(res) => {
                debug!("Success! {}", res.status())
            }
            Err(err) => match err {
                Error::Closed(res) => {
                    log_error_response(res);

                    disable_endpoint(endpoint.borrow_mut(), &storage);
                }
                Error::Open(res) => {
                    log_error_response(res);

                    if retry_policy.is_retryable(cmd.attempt) {
                        let cmd_to_retry = cmd.with_increased_attempt();
                        let duration = retry_policy.get_waiting_time(cmd.attempt);

                        publisher
                            .publish_delayed(
                                AsyncMessage::SentMessage(cmd_to_retry.clone()),
                                duration,
                            )
                            .await;

                        debug!(
                            "Message queued again. Attempt: {}. Delay: {:?}",
                            cmd_to_retry.attempt, duration
                        );
                    }
                }
                Error::Rejected => {
                    debug!(
                        "Endpoint {} is closed. Message {} rejected.",
                        key,
                        cmd.msg_id()
                    );

                    // todo do something with message? add to some "not delivered" bucket?
                }
            },
        }

        delivery.ack(BasicAckOptions::default()).await.expect("ack");
    }
}

fn log_error_response(res: reqwest::Error) {
    let status: String = res.status().map_or(String::from("-"), |s| s.to_string());

    debug!("Error response! Status: {}, Error: {}", status, res);
}

fn disable_endpoint(mut endpoint: RefMut<Endpoint>, storage: &Data<Storage>) {
    endpoint.disable_failing();
    storage.endpoints.save(endpoint.to_owned());

    debug!("Endpoint {} has been disabled", endpoint.id);
}
