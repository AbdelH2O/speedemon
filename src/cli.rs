use clap::Parser;
use reqwest::blocking::Client;
use std::cmp::min;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use validator::Validate;

#[derive(Parser, Validate, Debug, Clone)]
#[command(version, about)]
pub struct Args {
    /// The link to the file to download.
    #[clap(short, long)]
    #[validate(url(code = "The link is not valid"))]
    pub(crate) link: String,

    /// The output folder the file will be saved to.
    #[clap(short, long, default_value = ".")]
    pub(crate) output: String,

    /// The number of threads to use for downloading.
    #[clap(short = 'p', long, default_value = "4")]
    #[validate(range(
        min = 1,
        max = 32,
        code = "The number of threads must be between 1 and 32"
    ))]
    pub(crate) threads: u32,

    /// The number of retries to use for downloading.
    #[clap(short, long, default_value = "3")]
    #[validate(range(
        min = 1,
        max = 10,
        code = "The number of retries must be between 1 and 10"
    ))]
    pub(crate) retries: u32,

    /// The timeout for each request.
    #[clap(short, long, default_value = "10")]
    #[validate(range(min = 1, max = 60, code = "The timeout must be between 1 and 60"))]
    pub(crate) timeout: u32,
}

pub(crate) fn downloader_thread(
    url: &str,
    output: &str,
    retries: u32,
    timeout: u32,
    start: u64,
    end: u64,
    index: u32,
    file_name: &str,
    total: u32,
    progress: Arc<Mutex<f64>>,
) -> Result<(), String> {
    // Create a new reqwest client
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(timeout as u64))
        .build()
        .unwrap();

    let mut current_bytes = start;
    let mut downloaded_bytes = 0;
    let buffer_size = 1024 * 1024 * 15; // 10MB
    let mut try_count = 0;

    let file_path = format!("{}/{}.part{}_{}", output, file_name, index, total);
    let mut f = match File::open(&file_path) {
        Ok(mut file) => {
            let mut buffer = vec![];
            file.read_to_end(&mut buffer).unwrap();
            current_bytes += buffer.len() as u64;
            downloaded_bytes += buffer.len() as u64;
            file
        }
        Err(_) => {
            let f = File::create(&file_path).unwrap();
            f
        }
    };
    while current_bytes < end && try_count < retries {
        match client
            .get(url)
            .header(
                "Range",
                format!(
                    "bytes={}-{}",
                    current_bytes,
                    min(current_bytes + buffer_size as u64, end)
                ),
            )
            .send()
        {
            Ok(response) => match response.bytes() {
                Ok(bytes) => {
                    current_bytes += bytes.len() as u64;
                    downloaded_bytes += bytes.len() as u64;
                    f.write_all(&bytes).unwrap();
                    let mut progress_val = progress.lock().unwrap();
                    *progress_val += bytes.len() as f64 / ((end - start) * total as u64) as f64;
                    println!("Downloaded {:.2}% of the file", *progress_val * 100.0);
                }
                Err(e) => {
                    println!(
                        "[downloader_thread] Error: {}. Retrying for the {}th time.",
                        e,
                        try_count + 1
                    );
                    try_count += 1;
                }
            },
            Err(e) => {
                println!(
                    "[downloader_thread] Error: {}. Retrying for the {}th time.",
                    e,
                    try_count + 1
                );
                try_count += 1;
            }
        }
    }
    println!(
        "Thread {} downloaded {} bytes, expected {} bytes",
        index,
        downloaded_bytes,
        end - start
    );
    if try_count == retries {
        return Err("Error: download failed".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_downloader_thread() {
        let url = "https://i.imgur.com/z4d4kWk.jpg";
        let output = ".";
        let retries = 3;
        let timeout = 10;
        let start = 0;
        let end = 3000;
        let index = 0;
        let file_name = "100MB.bin";
        let total = 2;
        let progress = Arc::new(Mutex::new(0.0));
        assert_eq!(
            downloader_thread(
                url, output, retries, timeout, start, end, index, file_name, total, progress
            ),
            Ok(())
        );
    }

    #[test]
    fn test_downloader_thread_invalid_url() {
        let url = "https://speed.hetzner.de/100MB";
        let output = "/tmp/";
        let retries = 3;
        let timeout = 10;
        let start = 0;
        let end = 1024 * 1024 * 10;
        let index = 0;
        let file_name = "100MB.bin";
        let total = 4;
        let progress = Arc::new(Mutex::new(0.0));
        assert_eq!(
            downloader_thread(
                url, output, retries, timeout, start, end, index, file_name, total, progress
            ),
            Err("Error: download failed".to_string())
        );
    }

    #[test]
    fn args_validation() {
        let args = Args {
            link: "https://example.com".to_string(),
            output: "/tmp/".to_string(),
            threads: 4,
            retries: 3,
            timeout: 10,
        };
        assert_eq!(args.validate().is_ok(), true);
    }

    #[test]
    fn args_validation_invalid_url() {
        let args = Args {
            link: "example.comodo".to_string(),
            output: "/tmp/".to_string(),
            threads: 4,
            retries: 3,
            timeout: 10,
        };
        assert_eq!(args.validate().is_err(), true);
    }
}
