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
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "osmtograph")]
enum OsmToGraph {
    /// Format OSM filtered by way file to put one xml element by line
    Format,
    /// Extract all nodes data : node_id␟key␟value␟key␟value…
    Nodes,
    /// Extract links from ways nodes : node_id␟node_id␟way_id
    Links,
    /// Extract ways data : way_id␟key␟value␟key␟value…
    Ways,
    /// Applies 3 heuristics which removes degree two nodes, nodes < delta and links < delta.
    Heuristics {
        /// Delta is expressed in meters.
        #[structopt(short, long)]
        delta: f64,
    },
}

#[derive(Clone)]
struct Point {
    latitude: f64,
    longitude: f64,
}
#[derive(Clone)]
struct Node {
    id: String,
    lat: String,
    lon: String,
    neighbours: Vec<String>,
    data: String,
}

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
            let node: OsmNode = from_str(&row).unwrap();
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
                        data = data.replace('\n', " ").replace('\r', " ");
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
            let way: OsmWayNodes = from_str(&row).unwrap();
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
            let way: OsmWayTags = from_str(&row).unwrap();
            let mut data: String = "".to_owned();
            match way.tags {
                Some(tags) => {
                    if tags.len() > 0 {
                        for tag in tags {
                            let s = format!("{}{}{}{}", SEPARATOR, tag.k, SEPARATOR, tag.v);
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

fn deterministic_link(source: &str, target: &str) -> (String, String) {
    let link = if source < target {
        (source.to_owned(), target.to_owned())
    } else {
        (target.to_owned(), source.to_owned())
    };
    link
}

fn remove_degree_two_nodes(mut graph: HashMap<String, Node>) -> HashMap<String, Node> {
    let mut two_degree_nodes: Vec<String> = Vec::new();
    for (id, node) in graph.iter() {
        if node.neighbours.len() == 2 {
            two_degree_nodes.push(id.to_owned());
        }
    }
    for id in two_degree_nodes {
        graph = replace_node_by_links(graph, id);
    }
    graph
}

fn remove_under_delta_nodes(mut graph: HashMap<String, Node>, delta: f64) -> HashMap<String, Node> {
    let mut under_delta_nodes: Vec<String> = Vec::new();
    for (id, node) in graph.iter() {
        let start = Point {
            latitude: node.lat.parse().unwrap(),
            longitude: node.lon.parse().unwrap(),
        };
        let mut remove = true;
        for neighbour_id in &node.neighbours {
            let neighbour = graph.get(neighbour_id).unwrap();
            let end = Point {
                latitude: neighbour.lat.parse().unwrap(),
                longitude: node.lon.parse().unwrap(),
            };
            let distance = compute_distance(start.clone(), end);
            if distance > delta {
                remove = false;
            }
        }
        if remove {
            under_delta_nodes.push(id.to_owned());
            remove = true;
        }
    }
    for id in under_delta_nodes {
        graph = replace_node_by_links(graph, id);
    }
    graph
}

fn replace_node_by_links(mut graph: HashMap<String, Node>, id: String) -> HashMap<String, Node> {
    let node = graph.get(&id).unwrap().clone();
    graph.remove(&node.id);
    for current in 0..node.neighbours.len() {
        graph
            .entry(node.neighbours[current].clone())
            .and_modify(|e| {
                let id_index = e.neighbours.iter().position(|value| *value == id).unwrap();
                e.neighbours.swap_remove(id_index);
            });
        for next in current + 1..node.neighbours.len() {
            graph
                .entry(node.neighbours[current].clone())
                .and_modify(|e| e.neighbours.push(node.neighbours[next].clone()));
            graph
                .entry(node.neighbours[next].clone())
                .and_modify(|e| e.neighbours.push(node.neighbours[current].clone()));
        }
    }
    graph
}

// Haversine formula
fn compute_distance(start: Point, end: Point) -> f64 {
    let r: f64 = 6356752.0; // earth radius in meters
    let d_lat: f64 = (end.latitude - start.latitude).to_radians();
    let d_lon: f64 = (end.longitude - start.longitude).to_radians();
    let lat1: f64 = (start.latitude).to_radians();
    let lat2: f64 = (end.latitude).to_radians();

    let a: f64 = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin())
        + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());
    let c: f64 = 2.0 * ((a.sqrt()).atan2((1.0 - a).sqrt()));
    return r * c;
}

fn load_graph() -> HashMap<String, Node> {
    let mut graph: HashMap<String, Node> = HashMap::new();
    let input = io::stdin();
    for line in input.lock().lines() {
        let line = line.unwrap();
        let data: Vec<&str> = line.split(SEPARATOR).collect();
        match data.len() {
            3 => {
                let source = data[0].to_owned();
                let target = data[1].to_owned();
                graph
                    .entry(source.to_owned())
                    .and_modify(|e| e.neighbours.push(target.clone()));
                graph
                    .entry(target.to_owned())
                    .and_modify(|e| e.neighbours.push(source.clone()));
            }
            _ => {
                graph.entry(data[0].to_owned()).or_insert(Node {
                    id: data[0].to_owned(),
                    lat: data[2].to_owned(),
                    lon: data[4].to_owned(),
                    data: data[5..].join(&SEPARATOR.to_string()),
                    neighbours: Vec::new(),
                });
            }
        }
    }
    graph
}

fn print_graph(graph: HashMap<String, Node>) {
    let mut links: HashSet<(String, String)> = HashSet::new();
    for (id, node) in graph.into_iter() {
        println!("{}{}{}{}{}", id, SEPARATOR, node.lat, SEPARATOR, node.lon,);
        for source in 0..node.neighbours.len() - 1 {
            for target in 1..node.neighbours.len() {
                links.insert(deterministic_link(
                    &node.neighbours[source],
                    &node.neighbours[target],
                ));
            }
        }
    }
    for link in links {
        println!("{}{}{}", link.0, SEPARATOR, link.1);
    }
}

fn main() {
    match OsmToGraph::from_args() {
        OsmToGraph::Format => format_xml(),
        OsmToGraph::Nodes => extract_nodes(),
        OsmToGraph::Links => extract_links(),
        OsmToGraph::Ways => extract_ways(),
        OsmToGraph::Heuristics { delta } => {
            let mut graph = load_graph();
            graph = remove_degree_two_nodes(graph);
            graph = remove_under_delta_nodes(graph, delta);
            print_graph(graph)
        }
    }
}
