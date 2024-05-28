use std::time::Duration;

use futures_lite::StreamExt;
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use lapin::Channel;
use log::{debug, error, info};

use crate::amqp::{Publisher, Serializer};
use crate::circuit_breaker::{CircuitBreaker, Error};
use crate::cmd::AsyncMessage;
use crate::config::AMQPConfig;
use crate::retry::RetryPolicyBuilder;
use crate::sender::Sender;
use crate::storage::Storage;
use crate::time::Clock;

pub async fn consume(
    channel: Channel,
    consumer_tag: &str,
    storage: Storage,
    amqp_config: AMQPConfig,
) {
    let retry_policy = RetryPolicyBuilder::new()
        .max_retries(5)
        .exponential(2, Duration::from_secs(2))
        .randomize(0.5)
        .build()
        .unwrap();

    let mut circuit_breaker = CircuitBreaker::default();

    let mut consumer = channel
        .basic_consume(
            &amqp_config.sent_message_queue_name(),
            consumer_tag,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    let publisher = Publisher::new(channel, amqp_config);
    let clock = Clock::chrono();

    info!("consumer is ready");

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("error in consumer");
        let async_msg: AsyncMessage = Serializer::deserialize(&delivery.data);

        let AsyncMessage::SentMessage(cmd) = async_msg;

        info!("message consumed: {:?}", cmd);

        let msg = storage.messages.get(cmd.msg_id()).await;
        if msg.is_err() {
            error!(
                "Message {} doesn't exist and cannot be dispatched",
                cmd.msg_id()
            );

            delivery.ack(BasicAckOptions::default()).await.expect("ack");

            continue;
        }

        let mut msg = msg.unwrap();

        let event = storage.events.get(msg.event_id).await;
        if event.is_err() {
            error!(
                "Message {} doesn't exist and cannot be dispatched",
                msg.event_id
            );

            delivery.ack(BasicAckOptions::default()).await.expect("ack");

            continue;
        }

        let endpoint_id = msg.endpoint_id;
        let endpoint = storage.endpoints.get(&endpoint_id).await;
        if endpoint.is_err() {
            error!(
                "Endpoint {} doesn't not exists and message {} cannot be dispatched",
                endpoint_id, msg.event_id
            );

            delivery.ack(BasicAckOptions::default()).await.expect("ack");

            continue;
        }

        let event = event.unwrap();
        let endpoint = endpoint.unwrap();

        let sender = Sender::new(event.payload.clone(), endpoint.url.clone());
        let key = endpoint_id.to_string();

        if endpoint.is_active() && circuit_breaker.revive(&key).is_some() {
            debug!("Endpoint {} has been reopened", key);
        }

        let processing_time = event.calculate_processing_time(&clock);

        debug!(
            "Message {} for endpoint {} is being prepared to send. Processing time: {:?}",
            event.id.to_string(),
            endpoint.id.to_string(),
            processing_time,
        );

        match circuit_breaker.call(&key, || sender.send()).await {
            Ok(res) => {
                let log = msg.record_attempt(res, processing_time);
                storage.messages.save(msg).await;
                storage.attempt_log.save(log);
            }
            Err(err) => match err {
                Error::Closed(res) => {
                    let log = msg.record_attempt(res, processing_time);
                    storage.messages.save(msg).await;
                    storage.attempt_log.save(log);

                    let mut endpoint = endpoint;
                    let endpoint_id = endpoint.id;

                    endpoint.disable_failing();
                    storage.endpoints.save(endpoint).await;

                    debug!("Endpoint {} has been disabled", endpoint_id);
                }
                Error::Open(res) => {
                    let log = msg.record_attempt(res, processing_time);
                    storage.messages.save(msg).await;
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
                        key, msg.event_id
                    );

                    // todo do something with message? add to some "not delivered" bucket?
                }
            },
        }

        delivery.ack(BasicAckOptions::default()).await.expect("ack");
    }
}
