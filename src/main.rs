mod utils;
mod cli;

use std::time::Instant;
use clap::Parser;
use threadpool::ThreadPool;
use std::sync::{Arc, Mutex};

use crate::utils::{validate_url, merge_files, check_server_compatibility};
use crate::cli::{Args, downloader_thread};

fn main() {
    let start_time = Instant::now();
    let args = Args::parse();
    let link = args.link;
    let output = if args.output.ends_with("/") {
        args.output
    } else {
        args.output + "/"
    };
    let threads = args.threads;
    let retries = args.retries;
    let timeout = args.timeout;

    // Check if the link is valid
    match validate_url(&link) {
        Ok(()) => {}
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    }

    let (content_length, file_name) = match check_server_compatibility(&link.clone()) {
        Ok((content_length, file_name)) => (content_length, file_name),
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    let progress = Arc::new(Mutex::new(0.0));

    let pool = ThreadPool::new(threads as usize);
    let length = content_length / threads as u64;
    for index in 0..threads.clone() as u64 {
        let url = link.clone();
        let name = file_name.clone();
        let out = output.clone();

        let start = (length * index) + if index == 0 { 0 } else { 1 };
        let end = match index {
            x if x == threads as u64 - 1 => content_length,
            _ => length * (index + 1),
        };
        let progress = progress.clone();
        pool.execute(move || {
            match downloader_thread(&url, &out, retries, timeout, start, end, index as u32, &name, threads, progress) {
                Ok(()) => {}
                Err(_) => {
                    return;
                }
            }
        });
    }
    pool.join();
    merge_files(output, file_name.clone(), threads);
    println!("Downloaded in {} seconds", start_time.elapsed().as_secs());
}

