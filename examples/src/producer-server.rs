use std::env;

use dotenv::dotenv;

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

    let endpoint = sdk
        .endpoints()
        .create(&app.id, "http://localhost:8080", vec!["contact.created"])
        .await?;

    println!("Endpoint created - {:?}", endpoint);

    Ok(())
}
