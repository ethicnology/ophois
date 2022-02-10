use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
pub async fn download_map(city: String) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(format!("https://overpass-api.de/api/interpreter?data=[out:xml]; area[name = \"{}\"]; (way(area)[highway]; ); (._;>;); out;", city))
        .await?
        .text()
        .await?;
    let mut file = File::create(format!("{}.osm", city))?;
    file.write_all(response.as_bytes())?;
    Ok(())
}
