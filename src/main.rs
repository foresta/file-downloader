#[macro_use]
extern crate anyhow;

use anyhow::Result;
use futures::StreamExt;
use pbr::{ProgressBar, Units};
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let url = "https://blog.foresta.me/404.html";
    let filename = "blog-404.html";

    if let Err(e) = download_file(url, filename).await {
        println!("Error occurred: {:?}", e);
    } else {
        println!("Download Successfly!");
    }
}

async fn download_file(url: &str, filepath: &str) -> Result<()> {
    println!("Download Started: {}", url);

    let client = reqwest::Client::new();

    // send HEAD request for get content-length
    let content_length = get_content_length(&client, url).await?;

    // create file
    let mut file = tokio::fs::File::create(filepath).await?;

    // Initialize progressBar
    let mut pb = ProgressBar::new(content_length);
    pb.set_units(Units::Bytes);
    pb.set_width(Some(100));

    // send GET request for download
    let mut stream = client.get(url).send().await?.bytes_stream();
    while let Some(chunk_result) = stream.next().await {
        let chunk = &chunk_result?;

        // update progress bar
        pb.add(chunk.len() as u64);

        // write to file
        file.write_all(&chunk).await?;
    }

    file.flush().await?;

    println!("Download Finished: to {}", filepath);

    Ok(())
}

async fn get_content_length(client: &reqwest::Client, url: &str) -> Result<u64> {
    let head_result = client.head(url).send().await?;
    let headers = head_result.headers();
    let content_length_header = headers
        .get("content-length")
        .ok_or(anyhow!("failed to get content-length for {}", url))?;

    let content_length = content_length_header.to_str()?.parse::<u64>()?;
    Ok(content_length)
}
