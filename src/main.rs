use anyhow::Result;
use clap::Parser;
use reqwest::Client;
use std::fs;

mod args;
mod novel;
mod save_helper;

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();
    let content = fs::read_to_string(&args.input);

    let novel = novel::Novel {
        content: content?,
        context: args.context,
    };
    let client = Client::new();

    let translated = novel.translate(&client, &args.target, &args.model).await?;

    save_helper::remove_path(&args.out)?;
    save_helper::save_file(&translated, &args.out)?;

    return Ok(());
}
