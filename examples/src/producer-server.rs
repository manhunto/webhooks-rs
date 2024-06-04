use std::env;

use dotenv::dotenv;
use serde_json::json;

use sdk::error::Error;
use sdk::WebhooksSDK;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let url: String = env::var("SERVER_URL").unwrap();

    println!("{}", url);

    let sdk = WebhooksSDK::new(url.as_str());
    let app = sdk.application().create("dummy").await?;

    println!("App created - {:?}", app);

    let topic = "contact.created";
    let endpoint = sdk
        .endpoints()
        .create(&app.id, "http://localhost:8080", vec![topic])
        .await?;

    println!("Endpoint created - {:?}", endpoint);

    let payload = json!({
        "foo": {
            "bar": "baz"
        }
    });

    let event = sdk.events().create(&app.id, topic, &payload).await?;

    println!("Event created - {:?}", event);

    Ok(())
}
