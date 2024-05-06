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
use crate::sender::Sender;
use crate::storage::Storage;
use crate::time::Clock;

pub async fn consume(channel: Channel, consumer_tag: &str, storage: Data<Storage>) {
    let retry_policy = RetryPolicyBuilder::new()
        .max_retries(5)
        .exponential(2, Duration::from_secs(2))
        .randomize(0.5)
        .build()
        .unwrap();

    let mut circuit_breaker = CircuitBreaker::default();

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
    let clock = Clock::chrono();

    info!("consumer is ready");

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("error in consumer");
        let async_msg: AsyncMessage = Serializer::deserialize(&delivery.data);

        let AsyncMessage::SentMessage(cmd) = async_msg;

        info!("message consumed: {:?}", cmd);

        let routed_msg = storage.routed_messages.get(cmd.routed_msg_id());
        if routed_msg.is_err() {
            error!(
                "Routed message {} doesn't exist and cannot be dispatched",
                cmd.routed_msg_id()
            );

            delivery.ack(BasicAckOptions::default()).await.expect("ack");

            continue;
        }

        let mut routed_msg = routed_msg.unwrap();

        let msg = storage.messages.get(routed_msg.msg_id);
        if msg.is_err() {
            error!(
                "Message {} doesn't exist and cannot be dispatched",
                routed_msg.msg_id
            );

            delivery.ack(BasicAckOptions::default()).await.expect("ack");

            continue;
        }

        let endpoint_id = routed_msg.endpoint_id;
        let endpoint = storage.endpoints.get(&endpoint_id);
        if endpoint.is_err() {
            error!(
                "Endpoint {} doesn't not exists and message {} cannot be dispatched",
                endpoint_id, routed_msg.msg_id
            );

            delivery.ack(BasicAckOptions::default()).await.expect("ack");

            continue;
        }

        let msg = msg.unwrap();
        let endpoint = Rc::new(RefCell::new(endpoint.unwrap()));
        let endpoint_borrowed = endpoint.borrow().to_owned();

        let sender = Sender::new(msg.payload.clone(), endpoint_borrowed.url);
        let key = endpoint_id.to_string();

        let endpoint_borrowed = endpoint.borrow().to_owned();
        if endpoint_borrowed.is_active() && circuit_breaker.revive(&key).is_some() {
            debug!("Endpoint {} has been reopened", key);
        }

        let processing_time = msg.calculate_processing_time(&clock);

        debug!(
            "Message {} for endpoint {} is being prepared to send. Processing time: {:?}",
            msg.id.to_string(),
            endpoint_borrowed.id.to_string(),
            processing_time,
        );

        match circuit_breaker.call(&key, || sender.send()).await {
            Ok(res) => {
                let log = routed_msg.record_attempt(res, processing_time);
                storage.routed_messages.save(routed_msg);
                storage.attempt_log.save(log);
            }
            Err(err) => match err {
                Error::Closed(res) => {
                    let log = routed_msg.record_attempt(res, processing_time);
                    storage.routed_messages.save(routed_msg);
                    storage.attempt_log.save(log);

                    disable_endpoint(endpoint.borrow_mut(), &storage);
                }
                Error::Open(res) => {
                    let log = routed_msg.record_attempt(res, processing_time);
                    storage.routed_messages.save(routed_msg);
                    storage.attempt_log.save(log);

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

                    // todo add message that wasn't delivered to some storage
                }
                Error::Rejected => {
                    debug!(
                        "Endpoint {} is closed. Message {} rejected.",
                        key, routed_msg.msg_id
                    );

                    // todo do something with message? add to some "not delivered" bucket?
                }
            },
        }

        delivery.ack(BasicAckOptions::default()).await.expect("ack");
    }
}

fn disable_endpoint(mut endpoint: RefMut<Endpoint>, storage: &Data<Storage>) {
    endpoint.disable_failing();
    storage.endpoints.save(endpoint.to_owned());

    debug!("Endpoint {} has been disabled", endpoint.id);
}
