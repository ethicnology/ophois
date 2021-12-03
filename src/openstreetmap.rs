use crate::separator;
use quick_xml::de::from_str;
use serde::Deserialize;
use std::io;
use std::io::prelude::*;

#[derive(Deserialize)]
struct Node {
    id: String,
    lat: String,
    lon: String,
}

#[derive(Deserialize)]
struct Ways {
    #[serde(rename = "nd")]
    nodes: Vec<NodeRef>,
}

#[derive(Deserialize)]
struct NodeRef {
    r#ref: String,
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
    for line in input.lock().lines() {
        let row = line.unwrap();
        if row.starts_with("<node") {
            let node: Node = from_str(&row).unwrap();
            let mut data: String = "".to_owned();
            let coordinates = format!("{}{}{}{}", separator(), node.lat, separator(), node.lon);
            data.push_str(&coordinates);
            println!("{}{}", node.id, data);
        }
    }
}

pub fn extract_links() {
    let input = io::stdin();
    for line in input.lock().lines() {
        let row = line.unwrap();
        if row.starts_with("<way") {
            let way: Ways = from_str(&row).unwrap();
            let nodes = way.nodes;
            for i in 0..nodes.len() - 1 {
                println!("{}{}{}", nodes[i].r#ref, separator(), nodes[i + 1].r#ref,);
            }
        }
    }
}
