use log::debug;
use reqwest::StatusCode;
use serde_json::Value;
use url::Url;

use crate::events::domain::Payload;

pub struct Sender {
    payload: Payload,
    url: Url,
}

impl Sender {
    #[must_use]
    pub fn new(payload: Payload, url: Url) -> Self {
        Self { payload, url }
    }

    pub async fn send(&self) -> Result<(), ()> {
        let body: Value = serde_json::from_str(self.payload.to_string().as_str()).unwrap();

        let response = reqwest::Client::new()
            .post(self.url.to_owned())
            .json(&body)
            .send()
            .await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    debug!("Success response! {}", res.status());

                    return Ok(());
                }

                self.log_error_response(Some(res.status()), res.text().await.unwrap());

                Err(())
            }
            Err(res) => {
                self.log_error_response(res.status(), res.to_string());

                Err(())
            }
        }
    }

    fn log_error_response(&self, status_code: Option<StatusCode>, response: String) {
        let status: String = status_code.map_or(String::from("-"), |s| s.to_string());

        debug!("Error response! Status: {}, Error: {}", status, response);
    }
}
