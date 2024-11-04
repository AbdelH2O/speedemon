use std::fs::File;
use std::io::{Read, Write};
use reqwest::blocking::Client;

pub(crate) fn validate_url(url: &str) -> Result<(), String> {
    if url.is_empty() {
        return Err("The link is empty".to_string());
    }

    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("The link is not valid".to_string());
    }

    Ok(())
}

pub fn check_server_compatibility(link: &str) -> Result<(u64, String), String> {
    let client = Client::new();

    match client.head(link)
        .send() {
        Ok(response) => {
            let file_name = response.url().path().split("/").last().unwrap().to_string();
            match response.headers().get("Content-Length") {
                Some(value) => {
                    let content_length = value.to_str().unwrap().parse().unwrap();
                    match response.headers().get("Accept-Ranges") {
                        Some(_) => {
                            Ok((content_length, file_name))
                        }
                        None => {
                            Err("The server does not support range requests".to_string())
                        }
                    }
                }
                None => {
                    Err("The server did not return the content length".to_string())
                }
            }
        },
        Err(e) => {
            Err(format!("Error: {}", e))
        }
    }
}

pub(crate) fn merge_files(output: String, file_name: String, threads: u32) {
    let mut output_file = File::create(format!("{}/{}", output, file_name)).unwrap();
    for index in 0..threads {
        let mut f = File::open(format!("{}/{}.part{}_{}", output, file_name, index, threads)).unwrap();
        let mut buffer = vec![];
        f.read_to_end(&mut buffer).unwrap();
        output_file.write_all(&buffer).unwrap();
        std::fs::remove_file(format!("{}/{}.part{}_{}", output, file_name, index, threads)).unwrap();
    }
    println!("Done!");
}