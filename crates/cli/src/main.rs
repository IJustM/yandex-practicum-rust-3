use std::fs;

use clap::{Parser, Subcommand, ValueEnum};
use client::BlogClientImpl;
use uuid::Uuid;

const BLOG_TOKEN_FILE: &str = ".blog_token";

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
    /// Login user
    Login {
        #[arg(long)]
        email: String,

        #[arg(long)]
        password: String,
    },
    // Create post
    CreatePost {
        #[arg(long)]
        title: String,

        #[arg(long)]
        content: String,
    },
    // Update post
    UpdatePost {
        #[arg(long)]
        id: Uuid,

        #[arg(long)]
        title: String,

        #[arg(long)]
        content: String,
    },
    // Delete post
    DeletePost {
        #[arg(long)]
        id: Uuid,
    },
    // Delete post
    GetPost {
        #[arg(long)]
        id: Uuid,
    },
    // Get post list
    GetPostList {
        #[arg(long)]
        limit: Option<i64>,

        #[arg(long)]
        offset: Option<i64>,
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

    let token = fs::read_to_string(BLOG_TOKEN_FILE).unwrap_or("".to_string());
    if !token.is_empty() {
        client.set_token(&token);
    }

    match command {
        Commands::Register {
            email,
            password,
            username,
        } => {
            client.register(&username, &email, &password).await?;
            println!(
                "user with email={} and username={} created",
                email, username
            );
        }
        Commands::Login { email, password } => {
            let res = client.login(&email, &password).await?;
            fs::write(BLOG_TOKEN_FILE, res.access_token)?;
            println!("token saved");
        }
        Commands::CreatePost { title, content } => {
            let post = client.create_post(&title, &content).await?;
            println!("created post {}", post);
        }
        Commands::UpdatePost { id, title, content } => {
            let post = client.update_post(&id, &title, &content).await?;
            println!("updated post {}", post);
        }
        Commands::DeletePost { id } => {
            client.delete_post(&id).await?;
            println!("post {} deleted", id);
        }
        Commands::GetPost { id } => {
            let post = client.get_post(&id).await?;
            println!("get post {}", post);
        }
        Commands::GetPostList { limit, offset } => {
            let post_list = client.list_posts(limit, offset).await?;
            println!("post list {}", post_list);
        }
    };

    Ok(())
}
