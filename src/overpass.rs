use std::fs::File;
use std::io::prelude::*;
use reqwest;

#[tokio::main]
pub async fn download_map(
    city: String,
    overpassql: String,
    user_agent: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
                "https://overpass-api.de/api/interpreter?data=[out:xml]; area[name = \"{}\"]; {} out;",
                city, overpassql
            ))
        .header("User-Agent", user_agent)
        .send()
        .await?
        .text()
        .await?;
    let mut file = File::create(format!("{}.osm", city))?;
    file.write_all(response.as_bytes())?;
    Ok(())
}
