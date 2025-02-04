use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::detector::GpuInfo;

pub async fn download_driver(gpu: &GpuInfo) -> Result<String, crate::error::Error> {
    let client = Client::new();
    let url = format!(
        "https://drivers.universalgpu.org/{}/{}/latest",
        gpu.vendor, gpu.model
    );
    
    let response = client.get(&url).send().await?;
    let temp_path = std::env::temp_dir().join("ugd_driver.zip");
    
    let mut file = File::create(&temp_path).await?;
    let content = response.bytes().await?;
    file.write_all(&content).await?;
    
    temp_path.to_str()
        .map(|s| s.to_string())
        .ok_or(crate::error::Error::IoError("Invalid path".into()))
}