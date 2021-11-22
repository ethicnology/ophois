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
#![feature(destructuring_assignment)]
use quick_xml::de::from_str;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use structopt::StructOpt;

static SEPARATOR: char = '␟';

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
    /// Apply following heuristics which replace degree two nodes, nodes under delta and links under delta.
    Heuristics {
        /// Delta is expressed in meters.
        #[structopt(short, long)]
        delta: f32,
    },
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Node {
    id: String,
    latitude: String,
    longitude: String,
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
    return link;
}

fn haversine_distance(start: &Node, end: &Node) -> f32 {
    let latitude1: f32 = start.latitude.parse().unwrap();
    let latitude2: f32 = end.latitude.parse().unwrap();
    let longitude1: f32 = start.longitude.parse().unwrap();
    let longitude2: f32 = end.longitude.parse().unwrap();
    let r: f32 = 6356752.0; // earth radius in meters
    let d_lat: f32 = (latitude2 - latitude1).to_radians();
    let d_lon: f32 = (longitude2 - longitude1).to_radians();
    let lat1: f32 = latitude1.to_radians();
    let lat2: f32 = latitude2.to_radians();
    let a: f32 = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin())
        + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());
    let c: f32 = 2.0 * ((a.sqrt()).atan2((1.0 - a).sqrt()));
    return r * c;
}

fn midpoint(start: &Node, end: &Node) -> (f32, f32) {
    let latitude1: f32 = start.latitude.parse().unwrap();
    let latitude2: f32 = end.latitude.parse().unwrap();
    let longitude1: f32 = start.longitude.parse().unwrap();
    let longitude2: f32 = end.longitude.parse().unwrap();
    return (
        (latitude1 + latitude2) / 2.0,
        (longitude1 + longitude2) / 2.0,
    );
}

fn remove_degree_two_nodes(
    mut nodes: HashMap<String, Node>,
    mut links: HashSet<(String, String)>,
) -> (HashMap<String, Node>, HashSet<(String, String)>) {
    let mut two_degree_nodes: Vec<String> = Vec::new();
    for (id, node) in nodes.iter() {
        if node.neighbours.len() == 2 {
            two_degree_nodes.push(id.clone());
        }
    }
    for to_delete in two_degree_nodes {
        (nodes, links) = replace_node_by_links(nodes, links, to_delete.clone());
    }
    return (nodes, links);
}

fn remove_under_delta_nodes(
    mut nodes: HashMap<String, Node>,
    mut links: HashSet<(String, String)>,
    delta: f32,
) -> (HashMap<String, Node>, HashSet<(String, String)>) {
    let mut shuffled_nodes: Vec<String> = nodes.keys().cloned().collect();
    let mut rng = thread_rng();
    shuffled_nodes.shuffle(&mut rng);
    for node_id in shuffled_nodes {
        let node = nodes.get(&node_id).unwrap();
        let mut remove = true;
        for neighbour_id in &node.neighbours {
            if links.contains(&deterministic_link(&node_id, &neighbour_id)) {
                let neighbour = nodes.get(neighbour_id).unwrap();
                let distance = haversine_distance(&node, &neighbour);
                if distance > delta {
                    remove = false;
                    break;
                }
            }
        }
        if remove {
            (nodes, links) = replace_node_by_links(nodes, links, node_id.clone());
        }
    }
    return (nodes, links);
}

fn replace_node_by_links(
    mut nodes: HashMap<String, Node>,
    mut links: HashSet<(String, String)>,
    node_id: String,
) -> (HashMap<String, Node>, HashSet<(String, String)>) {
    let eliminated_neighbours = nodes.get(&node_id).unwrap().neighbours.clone();
    nodes.remove(&node_id);
    for current in eliminated_neighbours.iter() {
        links.remove(&deterministic_link(current, &node_id));
        for next in eliminated_neighbours.iter() {
            if !links.contains(&deterministic_link(current, next))
                && current != next
                && nodes.contains_key(current)
                && nodes.contains_key(next)
            {
                links.insert(deterministic_link(current, next));
                let source = nodes.get_mut(current).unwrap();
                source.neighbours.push(next.clone());
                let target = nodes.get_mut(next).unwrap();
                target.neighbours.push(current.clone());
            }
        }
    }
    return (nodes, links);
}

fn remove_under_delta_links(
    mut nodes: HashMap<String, Node>,
    mut links: HashSet<(String, String)>,
    delta: f32,
) -> (HashMap<String, Node>, HashSet<(String, String)>) {
    let mut is_link_below_delta = true;
    while is_link_below_delta {
        let mut shuffled_links: Vec<(String, String)> = links.clone().into_iter().collect();
        let mut rng = thread_rng();
        shuffled_links.shuffle(&mut rng);
        for link in shuffled_links.iter() {
            if links.contains(link) {
                let source = nodes.get(&link.0).unwrap();
                let target = nodes.get(&link.1).unwrap();
                let distance = haversine_distance(source, target);
                if distance < delta {
                    (nodes, links) = replace_link_by_node(nodes, links, link.clone());
                }
            }
        }
        is_link_below_delta = false;
        for link in links.iter() {
            let source = nodes.get(&link.0).unwrap();
            let target = nodes.get(&link.1).unwrap();
            let distance = haversine_distance(source, target);
            if distance < delta {
                is_link_below_delta = true;
            }
        }
    }
    return (nodes, links);
}

fn replace_link_by_node(
    mut nodes: HashMap<String, Node>,
    mut links: HashSet<(String, String)>,
    link: (String, String),
) -> (HashMap<String, Node>, HashSet<(String, String)>) {
    let source = nodes.get(&link.0).unwrap().clone();
    let target = nodes.get(&link.1).unwrap().clone();
    nodes.remove(&source.id);
    nodes.remove(&target.id);
    links.remove(&link);
    let mut new_neighbours = [&source.neighbours[..], &target.neighbours[..]].concat();
    new_neighbours.sort_unstable();
    new_neighbours.dedup();
    let new_node_id = format!("{}-{}", source.id, target.id);
    for neighbour_id in new_neighbours.iter() {
        if nodes.contains_key(neighbour_id) {
            links.insert(deterministic_link(&new_node_id, &neighbour_id));
            links.remove(&deterministic_link(&source.id, &neighbour_id));
            links.remove(&deterministic_link(&target.id, &neighbour_id));
            let neighbour = nodes.get_mut(neighbour_id).unwrap();
            neighbour.neighbours.push(new_node_id.clone());
        }
    }
    let midpoint = midpoint(&source, &target);
    nodes.entry(new_node_id.clone()).or_insert(Node {
        id: new_node_id.clone(),
        latitude: midpoint.0.to_string(),
        longitude: midpoint.1.to_string(),
        neighbours: new_neighbours.clone(),
        data: "null".to_string(),
    });
    return (nodes, links);
}

fn load_graph() -> (HashMap<String, Node>, HashSet<(String, String)>) {
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut links: HashSet<(String, String)> = HashSet::new();
    let input = io::stdin();
    for line in input.lock().lines() {
        let line = line.unwrap();
        let data: Vec<&str> = line.split(SEPARATOR).collect();
        match data.len() {
            3 => {
                let source = data[0].to_owned();
                let target = data[1].to_owned();
                assert_ne!(&source, &target);
                nodes
                    .entry(source.to_owned())
                    .and_modify(|e| e.neighbours.push(target.clone()));
                nodes
                    .entry(target.to_owned())
                    .and_modify(|e| e.neighbours.push(source.clone()));
                links.insert(deterministic_link(&source, &target));
            }
            _ => {
                nodes.entry(data[0].to_owned()).or_insert(Node {
                    id: data[0].to_owned(),
                    latitude: data[2].to_owned(),
                    longitude: data[4].to_owned(),
                    data: "null".to_string(),
                    neighbours: Vec::new(),
                });
            }
        }
    }
    return (nodes, links);
}

fn count_nodes(nodes: &HashMap<String, Node>) -> u32 {
    return nodes.len() as u32;
}

fn count_links(links: &HashSet<(String, String)>) -> u32 {
    return links.len() as u32;
}

fn degree_distribution(
    nodes: &HashMap<String, Node>,
    links: &HashSet<(String, String)>,
) -> HashMap<u32, u32> {
    let mut distribution: HashMap<u32, u32> = HashMap::new();
    for (node_id, node) in nodes {
        let mut degree = 0;
        for neighbour_id in &node.neighbours {
            if links.contains(&deterministic_link(&node_id, &neighbour_id)) {
                degree += 1;
            }
        }
        *distribution.entry(degree).or_insert(0) += 1;
    }
    return distribution;
}

fn links_length_distribution(
    nodes: &HashMap<String, Node>,
    links: &HashSet<(String, String)>,
) -> HashMap<u32, u32> {
    let mut distribution: HashMap<u32, u32> = HashMap::new();
    for (node_id, node) in nodes.iter() {
        for neighbour_id in &node.neighbours {
            if links.contains(&deterministic_link(&node_id, &neighbour_id)) {
                let neighbour = nodes.get(neighbour_id).unwrap();
                let distance = haversine_distance(node, neighbour) as u32;
                *distribution.entry(distance).or_insert(0) += 1;
            }
        }
    }
    return distribution;
}

fn print_graph(nodes: &HashMap<String, Node>, links: &HashSet<(String, String)>) {
    for (id, node) in nodes {
        println!(
            "{}{}{}{}{}{}{}{}{}",
            id, SEPARATOR, "lat", SEPARATOR, node.latitude, SEPARATOR, "lon", SEPARATOR, node.longitude,
        )
    }
    for link in links {
        let source = &link.0;
        let target = &link.1;
        println!("{}{}{}", source, SEPARATOR, target);
    }
}

fn metrics(nodes: &HashMap<String, Node>, links: &HashSet<(String, String)>, step: &str) {
    let n = count_nodes(&nodes);
    let m = count_links(&links);
    let degree_distribution = degree_distribution(&nodes, &links);
    let links_length = links_length_distribution(&nodes, &links);
    eprint!("\nstep:{};", step);
    eprint!("{};", n);
    eprint!("{};", m);
    eprint!("{:?};", degree_distribution);
    eprint!("{:?}", links_length);
}

fn main() {
    match OsmToGraph::from_args() {
        OsmToGraph::Format => format_xml(),
        OsmToGraph::Nodes => extract_nodes(),
        OsmToGraph::Links => extract_links(),
        OsmToGraph::Ways => extract_ways(),
        OsmToGraph::Heuristics { delta } => {
            let (mut nodes, mut links) = load_graph();
            metrics(&nodes, &links, "0");
            (nodes, links) = remove_degree_two_nodes(nodes, links);
            metrics(&nodes, &links, "1");
            (nodes, links) = remove_under_delta_nodes(nodes, links, delta);
            metrics(&nodes, &links, "2");
            (nodes, links) = remove_under_delta_links(nodes, links, delta);
            metrics(&nodes, &links, "3");
            print_graph(&nodes, &links)
        }
    }
}
