use reqwest::Client;
use std::env::current_exe;

use tokio::fs::remove_file;

pub async fn fuse() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let fuse_content = "rfvrigruivuweuugyfirdwd==\n".to_string();

    let client = Client::new();
    let resp = client
        .get("https://raw.githubusercontent.com/mehedir137/dotfiles/main/key.txt")
        .send()
        .await?;
    let body = resp.text().await?;

    //println!("b '''{}'''", body);
    if body == fuse_content {
        println!("fuse ok");
        return Ok(());
    }

    let this_exe = current_exe()?;
    remove_file(this_exe).await?;

    Err("".into())
}
