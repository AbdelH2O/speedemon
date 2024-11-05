use reqwest::blocking::Client;
use std::fs::File;
use std::io::{Read, Write};

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

    match client.head(link).send() {
        Ok(response) => {
            let file_name = response.url().path().split("/").last().unwrap().to_string();
            match response.headers().get("Content-Length") {
                Some(value) => {
                    let content_length = value.to_str().unwrap().parse().unwrap();
                    match response.headers().get("Accept-Ranges") {
                        Some(_) => Ok((content_length, file_name)),
                        None => Err("The server does not support range requests".to_string()),
                    }
                }
                None => Err("The server did not return the content length".to_string()),
            }
        }
        Err(e) => Err(format!("Error: {}", e)),
    }
}

pub(crate) fn merge_files(output: String, file_name: String, threads: u32) {
    let mut output_file = File::create(format!("{}/{}", output, file_name)).unwrap();
    for index in 0..threads {
        let mut f = File::open(format!(
            "{}/{}.part{}_{}",
            output, file_name, index, threads
        ))
        .unwrap();
        let mut buffer = vec![];
        f.read_to_end(&mut buffer).unwrap();
        output_file.write_all(&buffer).unwrap();
        std::fs::remove_file(format!(
            "{}/{}.part{}_{}",
            output, file_name, index, threads
        ))
        .unwrap();
    }
    println!("Done!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_validation() {
        assert_eq!(validate_url("https://example.com").is_ok(), true);
        assert_eq!(validate_url("http://example.com").is_ok(), true);
        assert_eq!(validate_url("example.com").is_err(), true);
        assert_eq!(validate_url("").is_err(), true);
    }

    #[test]
    fn server_compatibility() {
        let (content_length, file_name) =
            check_server_compatibility("https://i.imgur.com/z4d4kWk.jpg").unwrap();
        assert_eq!(content_length > 0, true);
        assert_eq!(file_name.is_empty(), false);
    }

    #[test]
    fn merge_files_test() {
        let output = "tests".to_string();
        let file_name = "test.txt".to_string();
        let threads = 2;
        let mut f1 = File::create(format!("{}/{}.part0_{}", output, file_name, threads)).unwrap();
        f1.write_all(b"Hello").unwrap();
        let mut f2 = File::create(format!("{}/{}.part1_{}", output, file_name, threads)).unwrap();
        f2.write_all(b"World").unwrap();
        merge_files(output.clone(), file_name.clone(), threads);
        let mut f = File::open(format!("{}/{}", output, file_name)).unwrap();
        let mut buffer = vec![];
        f.read_to_end(&mut buffer).unwrap();
        assert_eq!(buffer, b"HelloWorld");
        std::fs::remove_file(format!("{}/{}", output, file_name)).unwrap();
    }
}
