use std::env;

use clap::{Parser, Subcommand};
use dotenv::dotenv;

use sdk::WebhooksSDK;

/// Cli app to manage webhook-rs server
#[derive(Debug, Parser)]
#[clap(name = "webhooks-cli", version, about)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, Subcommand)]
enum Command {
    /// Resource for application management
    App {
        #[clap(subcommand)]
        subcommand: AppSubcommand,
    },
}

#[derive(Clone, Debug, Subcommand)]
enum AppSubcommand {
    /// Creates an application
    Create {
        /// App name
        name: String,
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = App::parse();

    let url = env::var("SERVER_URL").expect("env SERVER_URL is not set");
    let sdk = WebhooksSDK::new(url.as_str());

    match app.command {
        Command::App { subcommand } => match subcommand {
            AppSubcommand::Create { name } => {
                let app = sdk.application().create(name.as_str()).await;

                println!("App {} with name '{}' has been created", app.id, app.name);
            }
        },
    };
}
