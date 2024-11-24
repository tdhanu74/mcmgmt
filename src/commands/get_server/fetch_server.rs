use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use reqwest::Error;
use std::fmt::format;
use std::fs::File;
use std::io::Write;

pub async fn get_server_jar(version: &str) -> Result<(), Error> {
    let body = reqwest::get("https://mcversions.net/download/".to_owned()+version).await?.text().await?;
    let refs = body.lines();
    let mut server_a_string: &str = "";
    let mut server_string: &str = "";

    for r in refs {
        if r.contains("</a>") && r.contains("server.jar") {
            server_a_string = r.trim();
        }
    }
    for s in server_a_string.split("\"") {
        if s.contains("/server.jar"){
            server_string = s;
        }
    }

    let mut server_file_download = reqwest::get(server_string).await?;
    let total_size: u64 = server_file_download.content_length().expect("Content Length not present");
    let pb: ProgressBar = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").expect("Error in template")
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading server-{}.jar file....", version));
    
    let mut out: File = File::create(format(format_args!("server-{}.jar", version))).expect("Failed to create file");
    let mut downloaded: u64 = 0;
    while let Some(chunk) = server_file_download.chunk().await? {
        let _ = out.write(&chunk);
        let new = downloaded + chunk.len() as u64;
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded server-{}.jar file....", version));

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[tokio::test]
    async fn test_fetch_server() {
        let version = "1.21.3";
        let file_name = format(format_args!("server-{}.jar", version));
        let _ = get_server_jar(version).await;
        let file_exists = File::open(&file_name);
        assert!(file_exists.is_ok());
        let _ = fs::remove_file(&file_name);
    }
}

// https://maven.minecraftforge.net/net/minecraftforge/forge/1.20.6-50.1.23/forge-1.20.6-50.1.23-installer.jar
// https://maven.minecraftforge.net/net/minecraftforge/forge/json/version.json