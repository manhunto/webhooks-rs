use std::env;

use clap::{Parser, Subcommand};
use dotenv::dotenv;

use sdk::WebhooksSDK;

/// Cli app to manage webhook-rs server
#[derive(Debug, Parser, PartialEq)]
#[clap(name = "webhooks-cli", version, about)]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, Subcommand, PartialEq)]
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

#[derive(Clone, Debug, Subcommand, PartialEq)]
enum ApplicationSubcommand {
    /// Creates an application
    Create {
        /// Application name
        name: String,
    },
}

#[derive(Clone, Debug, Subcommand, PartialEq)]
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
    use clap::error::ErrorKind::MissingRequiredArgument;
    use clap::{CommandFactory, Parser};

    use crate::Cli;
    use crate::Command::Endpoint;
    use crate::EndpointSubcommand::Create;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert()
    }

    #[test]
    fn endpoint_create_topics_cannot_be_empty() {
        let result = Cli::try_parse_from([
            "webhooks-cli",
            "endpoint",
            "create",
            "app_2hRzcGs8D5aLaHBWHyqIcibuFA1",
            "http://localhost:8080",
        ]);

        assert!(result.is_err());
        assert_eq!(MissingRequiredArgument, result.err().unwrap().kind());
    }

    #[test]
    fn endpoint_create_single_topic() {
        let result = Cli::try_parse_from([
            "webhooks-cli",
            "endpoint",
            "create",
            "app_2hRzcGs8D5aLaHBWHyqIcibuFA1",
            "http://localhost:8080",
            "contact.created",
        ]);

        let expected = Cli {
            command: Endpoint {
                subcommand: Create {
                    app_id: "app_2hRzcGs8D5aLaHBWHyqIcibuFA1".to_string(),
                    url: "http://localhost:8080".to_string(),
                    topics: vec!["contact.created".to_string()],
                },
            },
        };

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn endpoint_create_multiple_topics() {
        let result = Cli::try_parse_from([
            "webhooks-cli",
            "endpoint",
            "create",
            "app_2hRzcGs8D5aLaHBWHyqIcibuFA1",
            "http://localhost:8080",
            "contact.created,contact.updated,contact.deleted",
        ]);

        let expected = Cli {
            command: Endpoint {
                subcommand: Create {
                    app_id: "app_2hRzcGs8D5aLaHBWHyqIcibuFA1".to_string(),
                    url: "http://localhost:8080".to_string(),
                    topics: vec![
                        "contact.created".to_string(),
                        "contact.updated".to_string(),
                        "contact.deleted".to_string(),
                    ],
                },
            },
        };

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }
}
