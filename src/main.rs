mod adapters;
mod article;
mod config;
mod error;
mod platform;
mod state;
mod writer;

use std::path::PathBuf;

use chrono::NaiveDate;
use clap::{Parser, Subcommand};

use adapters::devto::DevToPuller;
use adapters::{PullOptions, Puller};
use config::Config;
use error::{PullError, Result};
use platform::Platform;
use state::PullState;
use writer::Writer;

#[derive(Parser)]
#[command(name = "puller")]
#[command(about = "Pull/archive existing posts from social networks")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Pull articles from a platform
    Pull {
        /// Platform to pull from (devto)
        #[arg(short, long)]
        platform: String,

        /// Output directory for pulled articles
        output_dir: PathBuf,

        /// Preview what would be pulled without writing files
        #[arg(long)]
        dry_run: bool,

        /// Only pull articles published since this date (YYYY-MM-DD)
        #[arg(long)]
        since: Option<String>,

        /// Force re-pull existing articles
        #[arg(long)]
        force: bool,

        /// Exclude draft articles
        #[arg(long)]
        exclude_drafts: bool,
    },

    /// List articles from a platform without downloading
    List {
        /// Platform to list from (devto)
        #[arg(short, long)]
        platform: String,

        /// Only list articles published since this date (YYYY-MM-DD)
        #[arg(long)]
        since: Option<String>,

        /// Exclude draft articles
        #[arg(long)]
        exclude_drafts: bool,
    },
}

fn parse_date(s: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| PullError::InvalidDate(format!("Expected YYYY-MM-DD, got: {}", s)))
}

fn create_puller(platform: &str, config: &Config) -> Result<Box<dyn Puller>> {
    let platform: Platform = platform.parse()?;
    match platform {
        Platform::DevTo => {
            let api_key = config.devto_api_key()?.to_string();
            Ok(Box::new(DevToPuller::new(api_key)?))
        }
    }
}

async fn run_pull(
    platform: &str,
    output_dir: PathBuf,
    dry_run: bool,
    since: Option<String>,
    force: bool,
    exclude_drafts: bool,
) -> Result<()> {
    let config = Config::from_env();
    let puller = create_puller(platform, &config)?;

    let options = PullOptions {
        since: since.map(|s| parse_date(&s)).transpose()?,
        include_drafts: !exclude_drafts,
    };

    let writer = Writer::new(&output_dir, dry_run);
    writer.ensure_output_dir()?;

    let mut state = if dry_run {
        PullState::default()
    } else {
        PullState::load(&output_dir)?
    };

    println!("Fetching article list from {}...", puller.platform());
    let articles = puller.list_articles(&options).await?;
    println!("Found {} articles", articles.len());

    let mut pulled_count = 0;
    let mut skipped_count = 0;

    for meta in &articles {
        let platform_id = meta.platform_id();

        if !force && state.is_pulled(&platform_id) {
            if let Some(path) = state.get_local_path(&platform_id) {
                println!("  Skipping: {} (already at {})", meta.title, path);
            }
            skipped_count += 1;
            continue;
        }

        println!("  Pulling: {}", meta.title);

        let article = puller.fetch_article(&meta.id).await?;
        let filename = writer.write_article(&article, &mut state)?;

        if dry_run {
            println!("    Would write: {}", filename);
        } else {
            println!("    Wrote: {}", filename);
        }

        pulled_count += 1;
    }

    if !dry_run {
        state.save(&output_dir)?;
    }

    println!();
    println!("Done! Pulled: {}, Skipped: {}", pulled_count, skipped_count);

    if dry_run {
        println!("(dry-run mode - no files were written)");
    }

    Ok(())
}

async fn run_list(platform: &str, since: Option<String>, exclude_drafts: bool) -> Result<()> {
    let config = Config::from_env();
    let puller = create_puller(platform, &config)?;

    let options = PullOptions {
        since: since.map(|s| parse_date(&s)).transpose()?,
        include_drafts: !exclude_drafts,
    };

    println!("Fetching article list from {}...", puller.platform());
    let articles = puller.list_articles(&options).await?;
    println!("Found {} articles:\n", articles.len());

    for meta in &articles {
        let status = if meta.is_draft { "[DRAFT]" } else { "" };
        let date = meta
            .published_at
            .map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "N/A".to_string());

        println!("  {} {} {}", date, meta.title, status);
        if let Some(url) = &meta.url {
            println!("    {}", url);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Pull {
            platform,
            output_dir,
            dry_run,
            since,
            force,
            exclude_drafts,
        } => run_pull(&platform, output_dir, dry_run, since, force, exclude_drafts).await,
        Commands::List {
            platform,
            since,
            exclude_drafts,
        } => run_list(&platform, since, exclude_drafts).await,
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
