use std::{os::unix::prelude::MetadataExt, path::Path, string::ParseError, time::SystemTime};

use clap::Parser;
use colored::Colorize;
use file_mode::ModePath;
use steam_games::AppIdCacheInterface;

mod cli;
mod steam_games;

#[tokio::main]
pub async fn main() {
    // Get the CLI arts
    let args = cli::Args::parse();

    // Access the appid cache
    let mut appid_cache = AppIdCacheInterface::new();

    // Determine the target dir
    let search_dir = args.dir.unwrap_or_else(|| ".".to_string());
    let search_dir = Path::new(&search_dir);

    // Handle the target not being a directory or not existing
    if !search_dir.is_dir() {
        eprintln!(
            "gamels: cannot access '{}': No such file or directory",
            search_dir.to_str().unwrap()
        );
        std::process::exit(2);
    }
    if search_dir.is_file() {
        println!("{}", search_dir.to_str().unwrap());
        std::process::exit(0);
    }

    // List all files and directories in the target directory
    let dir_entries = match std::fs::read_dir(search_dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!(
                "gamels: cannot access '{}': {}",
                search_dir.to_str().unwrap(),
                e
            );
            std::process::exit(2);
        }
    };

    // Sort the target entries by size
    let mut dir_entries: Vec<_> = dir_entries.collect();
    dir_entries.sort_by_key(|entry| entry.as_ref().unwrap().metadata().unwrap().len() as i64 * -1);

    // Iterate the target entries, printing info about them
    for entry in dir_entries {
        match entry {
            Ok(entry) => {

                // Track the filename
                let filename = entry.path();
                let filename = filename.file_name().unwrap().to_str().unwrap();

                // Get the file metadata
                let metadata = match entry.metadata() {
                    Ok(metadata) => metadata,
                    Err(e) => {
                        eprintln!(
                            "gamels: cannot access '{}': {}",
                            entry.path().to_str().unwrap(),
                            e
                        );
                        continue;
                    }
                };

                // Get the file permissions
                let permissions = entry.path().mode().unwrap().to_string();

                // Get the owner and group
                let owner = users::get_user_by_uid(metadata.uid())
                    .map(|user| user.name().to_string_lossy().to_string())
                    .unwrap_or_else(|| metadata.uid().to_string());
                let group = users::get_group_by_gid(metadata.gid())
                    .map(|group| group.name().to_string_lossy().to_string())
                    .unwrap_or_else(|| metadata.gid().to_string());

                // Get the file size
                let size = metadata.len();
                let size = humansize::format_size(size, humansize::DECIMAL);

                // Get the date of last modification
                let mtime = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
                let mtime = chrono::DateTime::<chrono::Local>::from(mtime);

                // Query the appid cache for the appid
                let appid = entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .parse::<u64>()
                    .map(|appid| appid_cache.query(appid));

                // Print the file info
                println!("{} {}:{} {:>size_width$} {} {}{}",
                    permissions,
                    owner,
                    group,
                    size,
                    mtime.format("%Y-%m-%d %H:%M:%S"),
                    if metadata.is_dir() {
                        filename.bright_blue().to_string()
                    } else {
                        filename.to_string()
                    },
                    match appid {
                        Ok(query) => match query.await{
                            Ok(Some(name)) => format!(" ({})", name.bright_cyan()),
                            _ => "".to_string(),
                        }
                        _ => "".to_string(),
                    },
                    size_width = 8
                )
            }
            Err(e) => {
                eprintln!(
                    "gamels: cannot access '{}': {}",
                    search_dir.to_str().unwrap(),
                    e
                );
            }
        }
    }
}
