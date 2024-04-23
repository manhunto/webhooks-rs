use clap::{Parser, Subcommand};

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

fn main() {
    let app = App::parse();

    match app.command {
        Command::App { subcommand } => match subcommand {
            AppSubcommand::Create { name } => println!("Creating app with name {}", name),
        },
    }
}
