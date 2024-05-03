use std::time::{Duration, Instant};

use log::debug;
use reqwest::StatusCode;
use url::Url;

use crate::events::domain::Payload;
use crate::sender::Status::{Numeric, Unknown};

#[derive(Debug, Clone)]
pub enum Status {
    Numeric(u16),
    Unknown(String),
}

pub struct SentResult {
    pub status: Status,
    #[allow(dead_code)]
    pub response_time: Duration,
    #[allow(dead_code)]
    pub body: Option<String>,
}

impl SentResult {
    fn with_body(status: Status, response_time: Duration, body: String) -> Self {
        Self {
            status,
            response_time,
            body: Some(body),
        }
    }

    fn without_body(status: Status, response_time: Duration) -> Self {
        Self {
            status,
            response_time,
            body: None,
        }
    }
}

pub struct Sender {
    payload: Payload,
    url: Url,
}

impl Sender {
    #[must_use]
    pub fn new(payload: Payload, url: Url) -> Self {
        Self { payload, url }
    }

    pub async fn send(&self) -> Result<SentResult, SentResult> {
        let start = Instant::now();

        let response = reqwest::Client::new()
            .post(self.url.to_owned())
            .json(&self.payload)
            .send()
            .await;

        let end = start.elapsed();

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    debug!("Success response! {}", res.status());

                    return Ok(SentResult::with_body(
                        Numeric(res.status().as_u16()),
                        end,
                        res.text().await.unwrap(),
                    ));
                }

                let status_code = res.status();
                let status = status_code.as_u16();
                let body = res.text().await.unwrap();

                self.log_error_response(Some(status_code), body.clone());

                Err(SentResult::with_body(Numeric(status), end, body))
            }
            Err(err) => {
                self.log_error_response(err.status(), err.to_string());

                Err(SentResult::without_body(Unknown(err.to_string()), end))
            }
        }
    }

    fn log_error_response(&self, status_code: Option<StatusCode>, response: String) {
        let status: String = status_code.map_or(String::from("-"), |s| s.to_string());

        debug!("Error response! Status: {}, Error: {}", status, response);
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use mockito::Matcher::Json;
    use serde_json::json;
    use url::Url;

    use crate::events::domain::Payload;
    use crate::sender::Sender;

    #[test_case::test_case(200, Ok(()))]
    #[test_case::test_case(201, Ok(()))]
    #[test_case::test_case(299, Ok(()))]
    #[test_case::test_case(300, Err(()))]
    #[test_case::test_case(304, Err(()))]
    #[test_case::test_case(400, Err(()))]
    #[test_case::test_case(403, Err(()))]
    #[test_case::test_case(500, Err(()))]
    #[test_case::test_case(505, Err(()))]
    #[tokio::test]
    async fn only_status_2xx_is_valid_as_response(status_code: usize, expected: Result<(), ()>) {
        let mut server = mockito::Server::new_async().await;
        let url = Url::from_str(server.url().as_str()).unwrap();
        let payload = Payload::from(json!({"foo": "bar"}));

        let mock = server
            .mock("POST", "/")
            .match_body(Json(json!({"foo": "bar"})))
            .with_body("response")
            .with_status(status_code)
            .create_async()
            .await;

        let result = Sender::new(payload, url).send().await;

        mock.assert_async().await;

        match expected {
            Ok(_) => assert!(result.is_ok()),
            Err(_) => assert!(result.is_err()),
        }
    }

    #[tokio::test]
    async fn request_to_unavailable_server_is_error() {
        let url = Url::from_str("http://localhost:0").unwrap();
        let payload = Payload::from(json!({"foo": "bar"}));

        let result = Sender::new(payload, url).send().await;

        assert!(result.is_err())
    }

    //todo: test response object
}
