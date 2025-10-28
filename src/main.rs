mod claude;
mod cli;
mod display;
mod error;
mod filters;
mod fzf;
mod history;

use clap::Parser;
use cli::Args;
use error::{AppError, Result};

fn main() {
    if let Err(e) = run() {
        match e {
            AppError::SelectionCancelled => {
                // User cancelled, exit silently
                std::process::exit(0);
            }
            _ => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    // Get current working directory
    let current_dir = std::env::current_dir().map_err(|e| {
        AppError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Failed to get current directory: {}", e),
        ))
    })?;

    // Convert to Claude projects directory path
    let projects_dir = history::get_claude_projects_dir(&current_dir)?;

    // If --show-dir flag is set, print directory and exit
    if args.show_dir {
        println!("{}", projects_dir.display());
        return Ok(());
    }

    // Verify directory exists
    if !projects_dir.exists() {
        return Err(AppError::ProjectsDirNotFound(
            projects_dir.display().to_string(),
        ));
    }

    // Load all conversations (reads each file once)
    let conversations = history::load_conversations(&projects_dir, args.last)?;

    if conversations.is_empty() {
        return Err(AppError::NoHistoryFound(projects_dir.display().to_string()));
    }

    // Use fzf to select a conversation
    let selected_path = fzf::select_conversation(&conversations)?;

    // Display the selected conversation
    display::display_conversation(&selected_path, args.no_tools)?;

    Ok(())
}
