use sdk::WebhooksSDK;

#[tokio::main]
async fn main() {
    let sdk = WebhooksSDK::new("http://localhost:8090".to_string());

    let app = sdk.application().create("dummy".to_string()).await;

    println!("{:?}", app);
}
