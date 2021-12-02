use crate::separator;
use quick_xml::de::from_str;
use serde::Deserialize;
use std::io;
use std::io::prelude::*;

#[derive(Deserialize)]
struct OsmNode {
    id: String,
    lat: String,
    lon: String,
    #[serde(rename = "tag")]
    tags: Option<Vec<OsmTag>>,
}

#[derive(Deserialize)]
struct OsmWayNodes {
    id: String,
    #[serde(rename = "nd")]
    nodes: Vec<OsmNodeRef>,
}

#[derive(Deserialize)]
struct OsmWayTags {
    id: String,
    #[serde(rename = "tag")]
    tags: Option<Vec<OsmTag>>,
}

#[derive(Deserialize)]
struct OsmNodeRef {
    r#ref: String,
}

#[derive(Deserialize)]
struct OsmTag {
    k: String,
    v: String,
}

pub fn format_xml() {
    let mut data: String = "".to_owned();
    let mut way = false;
    let mut node = false;
    let input = io::stdin();
    for line in input.lock().lines() {
        let row = line.unwrap().trim().to_string();
        if row.starts_with("<node") && row.ends_with("/>") {
            println!("{}", row)
        }
        if row.starts_with("<way") && row.ends_with("/>") {
            println!("{}", row)
        }
        if row.starts_with("<node") && !row.ends_with("/>") {
            node = true;
            data = "".to_owned();
        }
        if row.starts_with("<way") && !row.ends_with("/>") {
            way = true;
            data = "".to_owned();
        }
        if node == true || way == true {
            data.push_str(&row);
        }
        if row.contains("</node>") {
            node = false;
            println!("{}", data);
        }
        if row.contains("</way>") {
            way = false;
            println!("{}", data);
        }
    }
}

pub fn extract_nodes() {
    let input = io::stdin();
    let latitude = "lat";
    let longitude = "lon";
    for line in input.lock().lines() {
        let row = line.unwrap();
        if row.starts_with("<node") {
            let node: OsmNode = from_str(&row).unwrap();
            let mut data: String = "".to_owned();
            let coordinates = format!(
                "{}{}{}{}{}{}{}{}",
                separator(),
                latitude,
                separator(),
                node.lat,
                separator(),
                longitude,
                separator(),
                node.lon
            );
            data.push_str(&coordinates);
            match node.tags {
                Some(tags) => {
                    if tags.len() > 0 {
                        for tag in tags {
                            let s = format!("{}{}{}{}", separator(), tag.k, separator(), tag.v);
                            data.push_str(&s);
                        }
                        data = data.replace('\n', " ").replace('\r', " ");
                    }
                }
                None => (),
            }
            println!("{}{}", node.id, data);
        }
    }
}

pub fn extract_links() {
    let input = io::stdin();
    for line in input.lock().lines() {
        let row = line.unwrap();
        if row.starts_with("<way") {
            let way: OsmWayNodes = from_str(&row).unwrap();
            let nodes = way.nodes;
            for i in 0..nodes.len() - 1 {
                println!(
                    "{}{}{}{}{}",
                    nodes[i].r#ref,
                    separator(),
                    nodes[i + 1].r#ref,
                    separator(),
                    way.id
                );
            }
        }
    }
}

pub fn extract_ways() {
    let input = io::stdin();
    for line in input.lock().lines() {
        let row = line.unwrap();
        if row.starts_with("<way") {
            let way: OsmWayTags = from_str(&row).unwrap();
            let mut data: String = "".to_owned();
            match way.tags {
                Some(tags) => {
                    if tags.len() > 0 {
                        for tag in tags {
                            let s = format!("{}{}{}{}", separator(), tag.k, separator(), tag.v);
                            data.push_str(&s);
                        }
                        data = data.replace('\n', " ").replace('\r', " ");
                    }
                }
                None => (),
            }
            if data.len() > 0 {
                println!("{}{}", way.id, data);
            } else {
                println!("{}", way.id);
            }
        }
    }
}
