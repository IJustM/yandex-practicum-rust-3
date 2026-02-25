use clap::{Parser, Subcommand, ValueEnum};
use client::BlogClientImpl;

#[derive(Parser)]
#[command(name = "blog", about = "CLI for blog")]
struct Cli {
    #[arg(value_enum, help = "Transport")]
    transport: Transport,

    #[arg(help = "Server address")]
    server: String,

    #[command(subcommand, help = "Command for server")]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Register user
    Register {
        #[arg(long)]
        email: String,

        #[arg(long)]
        password: String,

        #[arg(long)]
        username: String,
    },
}

#[derive(ValueEnum, Clone)]
enum Transport {
    Http,
    Grpc,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let Cli {
        transport,
        server,
        command,
    } = cli;

    let transport = match transport {
        Transport::Http => client::Transport::Http(server),
        Transport::Grpc => client::Transport::Grpc(server),
    };

    let mut client = BlogClientImpl::new(transport).await?;

    match command {
        Commands::Register {
            email,
            password,
            username,
        } => {
            client.register(&username, &email, &password).await?;
        }
    };

    Ok(())
}
