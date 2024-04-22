use std::env::var;

use dotenv::dotenv;

use sdk::WebhooksSDK;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port: u16 = var("SERVER_PORT").unwrap().parse().unwrap();
    let url = format!("http://localhost:{}", port);

    println!("{}", url);

    let sdk = WebhooksSDK::new(url.as_str());

    let app = sdk.application().create("dummy").await;

    println!("App created - {:?}", app);
}
