// osmtograph extract graph object from OpenStreetMap data
// Copyright (C) 2021 Jules Azad EMERY a.k.a. ethicnology
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use quick_xml::de::from_str;
use serde::Deserialize;
use std::io;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "osmtograph")]
struct Opt {
    /// Format OSM filtered by way file to put one xml element by line
    #[structopt(short, long)]
    format: bool,
    /// Extract all nodes data : node_id␟key␟value␟key␟value…
    #[structopt(short, long)]
    nodes: bool,
    /// Extract links from ways nodes : node_id␟node_id␟way_id
    #[structopt(short, long)]
    links: bool,
    /// Extract ways data : way_id␟key␟value␟key␟value…
    #[structopt(short, long)]
    ways: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Node {
    id: String,
    lat: String,
    lon: String,
    #[serde(rename = "tag")]
    tags: Option<Vec<Tag>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct WayNodes {
    id: String,
    #[serde(rename = "nd")]
    nodes: Vec<NodeRef>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct WayTags {
    id: String,
    #[serde(rename = "tag")]
    tags: Option<Vec<Tag>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct NodeRef {
    r#ref: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Tag {
    k: String,
    v: String,
}

static SEPARATOR: char = '␟';

fn format_xml() {
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

fn extract_nodes() {
    let input = io::stdin();
    let latitude = "lat";
    let longitude = "lon";
    for line in input.lock().lines() {
        let row = line.unwrap();
        if row.starts_with("<node") {
            let node: Node = from_str(&row).unwrap();
            let mut data: String = "".to_owned();
            let coordinates = format!(
                "{}{}{}{}{}{}{}{}",
                SEPARATOR, latitude, SEPARATOR, node.lat, SEPARATOR, longitude, SEPARATOR, node.lon
            );
            data.push_str(&coordinates);
            match node.tags {
                Some(tags) => {
                    if tags.len() > 0 {
                        for tag in tags {
                            let s = format!("{}{}{}{}", SEPARATOR, tag.k, SEPARATOR, tag.v);
                            data.push_str(&s);
                        }
                    }
                }
                None => (),
            }
            println!("{}{}", node.id, data);
        }
    }
}

fn extract_links() {
    let input = io::stdin();
    for line in input.lock().lines() {
        let row = line.unwrap();
        if row.starts_with("<way") {
            let way: WayNodes = from_str(&row).unwrap();
            let nodes = way.nodes;
            for i in 0..nodes.len() - 1 {
                println!(
                    "{}{}{}{}{}",
                    nodes[i].r#ref,
                    SEPARATOR,
                    nodes[i + 1].r#ref,
                    SEPARATOR,
                    way.id
                );
            }
        }
    }
}

fn extract_ways() {
    let input = io::stdin();
    for line in input.lock().lines() {
        let row = line.unwrap();
        if row.starts_with("<way") {
            let way: WayTags = from_str(&row).unwrap();
            let mut data: String = "".to_owned();
            match way.tags {
                Some(tags) => {
                    if tags.len() > 0 {
                        for tag in tags {
                            let s = format!("{}{}{}{}", SEPARATOR, tag.k, SEPARATOR, tag.v);
                            data.push_str(&s);
                        }
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

fn main() {
    let opt = Opt::from_args();
    if opt.format {
        format_xml();
    }
    if opt.nodes {
        extract_nodes();
    }
    if opt.links {
        extract_links();
    }
    if opt.ways {
        extract_ways();
    }
}
