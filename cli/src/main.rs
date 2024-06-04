use std::env;

use clap::{Parser, Subcommand};
use dotenv::dotenv;

use sdk::WebhooksSDK;

/// Cli app to manage webhook-rs server
#[derive(Debug, Parser)]
#[clap(name = "webhooks-cli", version, about)]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, Subcommand)]
enum Command {
    /// Resource for application management
    Application {
        #[clap(subcommand)]
        subcommand: ApplicationSubcommand,
    },
    /// Resource for endpoints management
    Endpoint {
        #[clap(subcommand)]
        subcommand: EndpointSubcommand,
    },
}

#[derive(Clone, Debug, Subcommand)]
enum ApplicationSubcommand {
    /// Creates an application
    Create {
        /// Application name
        name: String,
    },
}

#[derive(Clone, Debug, Subcommand)]
enum EndpointSubcommand {
    /// Creates an endpoint
    Create {
        app_id: String,
        url: String,
        #[arg(value_parser, num_args = 1.., value_delimiter = ',', required = true)]
        topics: Vec<String>,
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    let url = env::var("SERVER_URL").expect("env SERVER_URL is not set");
    let sdk = WebhooksSDK::new(url.as_str());

    match cli.command {
        Command::Application { subcommand } => match subcommand {
            ApplicationSubcommand::Create { name } => {
                let app = sdk.application().create(name.as_str()).await.unwrap();

                println!("App {} with name '{}' has been created", app.id, app.name);
            }
        },
        Command::Endpoint { subcommand } => match subcommand {
            EndpointSubcommand::Create {
                app_id,
                url,
                topics,
            } => {
                println!("{}", app_id);
                println!("{}", url);
                println!("{:?}", topics);
            }
        },
    };
}

#[cfg(test)]
mod test {
    use clap::CommandFactory;

    use crate::Cli;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert()
    }
}
