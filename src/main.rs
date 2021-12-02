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

mod geo;

use geo::*;
use quick_xml::de::from_str;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
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

type Links = HashSet<(String, String)>;
type Nodes = HashMap<String, Node>;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct Node {
    id: String,
    longitude: String,
    latitude: String,
    neighbours: Vec<String>,
}

impl Node {
    fn point(&self) -> Point {
        return Point {
            x: self.longitude.parse().unwrap(),
            y: self.latitude.parse().unwrap(),
        };
    }
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

fn remove_degree_two_nodes(mut nodes: Nodes, mut links: Links) -> (Nodes, Links) {
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

fn remove_under_delta_nodes(mut nodes: Nodes, mut links: Links, delta: f32) -> (Nodes, Links) {
    let mut shuffled_nodes: Vec<String> = nodes.keys().cloned().collect();
    let mut rng = thread_rng();
    shuffled_nodes.shuffle(&mut rng);
    for node_id in shuffled_nodes {
        let node = nodes.get(&node_id).unwrap();
        let mut remove = true;
        for neighbour_id in &node.neighbours {
            if links.contains(&deterministic_link(&node_id, &neighbour_id)) {
                let neighbour = nodes.get(neighbour_id).unwrap();
                let distance = haversine_distance(&node.point(), &neighbour.point());
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

fn replace_node_by_links(mut nodes: Nodes, mut links: Links, node_id: String) -> (Nodes, Links) {
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

fn remove_under_delta_links(mut nodes: Nodes, mut links: Links, delta: f32) -> (Nodes, Links) {
    let mut is_link_below_delta = true;
    while is_link_below_delta {
        let mut shuffled_links: Vec<(String, String)> = links.clone().into_iter().collect();
        let mut rng = thread_rng();
        shuffled_links.shuffle(&mut rng);
        for link in shuffled_links.iter() {
            if links.contains(link) {
                let source = nodes.get(&link.0).unwrap();
                let target = nodes.get(&link.1).unwrap();
                let distance = haversine_distance(&source.point(), &target.point());
                if distance < delta {
                    (nodes, links) = replace_link_by_node(nodes, links, link.clone());
                }
            }
        }
        is_link_below_delta = false;
        for link in links.iter() {
            let source = nodes.get(&link.0).unwrap();
            let target = nodes.get(&link.1).unwrap();
            let distance = haversine_distance(&source.point(), &target.point());
            if distance < delta {
                is_link_below_delta = true;
            }
        }
    }
    return (nodes, links);
}

fn replace_link_by_node(
    mut nodes: Nodes,
    mut links: Links,
    link: (String, String),
) -> (Nodes, Links) {
    let source = nodes.get(&link.0).unwrap().clone();
    let target = nodes.get(&link.1).unwrap().clone();
    nodes.remove(&source.id);
    nodes.remove(&target.id);
    links.remove(&link);
    let mut new_neighbours = [&source.neighbours[..], &target.neighbours[..]].concat();
    new_neighbours.sort_unstable();
    new_neighbours.dedup();
    let new_node_id = format!("{}-{}", source.id, target.id); // non deterministic id -> duplicate risks
    for neighbour_id in new_neighbours.iter() {
        if nodes.contains_key(neighbour_id) {
            links.insert(deterministic_link(&new_node_id, &neighbour_id));
            links.remove(&deterministic_link(&source.id, &neighbour_id));
            links.remove(&deterministic_link(&target.id, &neighbour_id));
            let neighbour = nodes.get_mut(neighbour_id).unwrap();
            neighbour.neighbours.push(new_node_id.clone());
        }
    }
    let midpoint = midpoint(&source.point(), &target.point());
    nodes.entry(new_node_id.clone()).or_insert(Node {
        id: new_node_id.clone(),
        longitude: midpoint.x.to_string(),
        latitude: midpoint.y.to_string(),
        neighbours: new_neighbours.clone(),
    });
    return (nodes, links);
}

fn bfs_connected_components_distribution_and_largest(
    nodes: &Nodes,
    links: &Links,
) -> (Nodes, Links) {
    let mut queue: VecDeque<String> = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut distribution: HashMap<u32, u32> = HashMap::new();
    let mut largest_component_links: Links = HashSet::new();
    let mut largest_component_nodes: Nodes = HashMap::new();
    let mut largest_component_size: u32 = 0;
    for (node_id, _) in nodes.iter() {
        if !visited.contains(node_id) {
            let mut component_size: u32 = 0;
            let mut current_component_links: Links = HashSet::new();
            let mut current_component_nodes: Nodes = HashMap::new();
            queue.push_back(node_id.clone());
            visited.insert(node_id.clone());
            while !queue.is_empty() {
                component_size += 1;
                let current_id = queue.pop_front().unwrap();
                let current_node = nodes.get(&current_id).unwrap();
                current_component_nodes.insert(current_id.clone(), current_node.clone());
                for neighbour_id in &current_node.neighbours {
                    if links.contains(&deterministic_link(&current_id, neighbour_id))
                        && !visited.contains(neighbour_id)
                    {
                        visited.insert(neighbour_id.clone());
                        queue.push_back(neighbour_id.clone());
                        current_component_links
                            .insert(deterministic_link(&current_id, neighbour_id));
                    }
                }
            }
            if component_size > largest_component_size {
                largest_component_size = component_size;
                largest_component_links = current_component_links;
                largest_component_nodes = current_component_nodes;
            }
            distribution.entry(component_size).or_insert(0);
            distribution.insert(component_size, distribution[&component_size] + 1);
        }
    }
    distribution_to_file("connected_components_distribution", distribution)
        .expect("connected components distribution");
    return (largest_component_nodes, largest_component_links);
}

fn load_graph() -> (Nodes, Links) {
    let mut nodes: Nodes = HashMap::new();
    let mut links: Links = HashSet::new();
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
                    longitude: data[4].to_owned(),
                    latitude: data[2].to_owned(),
                    neighbours: Vec::new(),
                });
            }
        }
    }
    return (nodes, links);
}

fn count_nodes(nodes: &Nodes) -> u32 {
    return nodes.len() as u32;
}

fn count_links(links: &Links) -> u32 {
    return links.len() as u32;
}

fn degree_distribution(nodes: &Nodes, links: &Links) -> HashMap<u32, u32> {
    let mut distribution: HashMap<u32, u32> = HashMap::new();
    for (node_id, node) in nodes {
        let mut degree = 0;
        for neighbour_id in &node.neighbours {
            if links.contains(&deterministic_link(&node_id, &neighbour_id)) {
                degree += 1;
            }
        }
        distribution.entry(degree).or_insert(0);
        distribution.insert(degree, distribution[&degree] + 1);
    }
    return distribution;
}

fn links_length_distribution(nodes: &Nodes, links: &Links) -> HashMap<u32, u32> {
    let mut distribution: HashMap<u32, u32> = HashMap::new();
    for (node_id, node) in nodes.iter() {
        for neighbour_id in &node.neighbours {
            if links.contains(&deterministic_link(&node_id, &neighbour_id)) {
                let neighbour = nodes.get(neighbour_id).unwrap();
                let distance = haversine_distance(&node.point(), &neighbour.point()) as u32;
                distribution.entry(distance).or_insert(0);
                distribution.insert(distance, distribution[&distance] + 1);
            }
        }
    }
    return distribution;
}

fn substitute_nodes_distribution(nodes: &Nodes) -> HashMap<u32, u32> {
    let mut distribution: HashMap<u32, u32> = HashMap::new();
    for (node_id, _) in nodes.iter() {
        let splitted_id: Vec<&str> = node_id.split('-').collect();
        let substitute = splitted_id.len() as u32;
        distribution.entry(substitute).or_insert(0);
        distribution.insert(substitute, distribution[&substitute] + 1);
    }
    return distribution;
}

fn print_graph(nodes: &Nodes, links: &Links) {
    for (id, node) in nodes {
        println!(
            "{}{}{}{}{}{}{}{}{}",
            id,
            SEPARATOR,
            "lat",
            SEPARATOR,
            node.latitude,
            SEPARATOR,
            "lon",
            SEPARATOR,
            node.longitude,
        )
    }
    for link in links {
        let source = &link.0;
        let target = &link.1;
        println!("{}{}{}", source, SEPARATOR, target);
    }
}

fn metrics(nodes: &Nodes, links: &Links, param: (&str, String)) {
    let _n = count_nodes(&nodes);
    let _m = count_links(&links);
    let degree = degree_distribution(&nodes, &links);
    let links_length = links_length_distribution(&nodes, &links);
    let substitutes = substitute_nodes_distribution(&nodes);
    distribution_to_file(
        &format!("degree_step:{}_delta:{}", param.0, param.1),
        degree,
    )
    .expect("degree distribution");
    distribution_to_file(
        &format!("links_length_step:{}_delta:{}", param.0, param.1),
        links_length,
    )
    .expect("links length distribution");
    distribution_to_file(
        &format!("substitutes_step:{}_delta:{}", param.0, param.1),
        substitutes,
    )
    .expect("substitutes nodes distribution");
}

fn distribution_to_file(file_name: &str, distribution: HashMap<u32, u32>) -> std::io::Result<()> {
    let mut string: String = "".to_owned();
    for (key, value) in distribution {
        string.push_str(&format!("{} {}\n", key, value))
    }
    let directory = "./distributions";
    fs::create_dir_all(directory)?;
    let mut file = fs::File::create(format!("./distributions/{}", file_name))?;
    file.write_all(string.as_bytes())?;
    Ok(())
}

fn discretize(mut nodes: Nodes, mut links: Links, delta: f32) -> (Nodes, Links) {
    let links_clone = links.clone();
    for link in links_clone {
        let source = nodes.get(&link.0).unwrap().clone();
        let target = nodes.get(&link.1).unwrap().clone();
        let distance = haversine_distance(&source.point(), &target.point());
        if distance >= 2.0 * delta {
            links.remove(&deterministic_link(&source.id.clone(), &target.id.clone()));
            let a = (distance / delta) as u32;
            let mut new_nodes = Vec::new();
            for i in 1..a {
                let b = get_point_from_line(&source.point(), &target.point(), i as f32 / a as f32);
                let node = Node {
                    id: format!("{}-{}-{}/{}", source.id, target.id, i, a), // non deterministic id -> duplicate risks
                    longitude: b.x.to_string(),
                    latitude: b.y.to_string(),
                    neighbours: Vec::new(),
                };
                new_nodes.push(node.id.clone());
                nodes.entry(node.id.clone()).or_insert(node);
            }
            for j in 1..a {
                let new_node_id = &new_nodes[(j - 1) as usize];
                let mut previous = format!("{}-{}-{}/{}", source.id, target.id, j - 1, a);
                let mut next = format!("{}-{}-{}/{}", source.id, target.id, j + 1, a);
                if j == 1 {
                    previous = source.id.clone();
                }
                if j == a - 1 {
                    next = target.id.clone();
                }
                let node = nodes.get_mut(new_node_id).unwrap();
                node.neighbours.push(previous.clone());
                node.neighbours.push(next.clone());
                let previous_node = nodes.get_mut(&previous).unwrap();
                previous_node.neighbours.push(new_node_id.clone());
                let next_node = nodes.get_mut(&next).unwrap();
                next_node.neighbours.push(new_node_id.clone());
                links.insert((previous, new_node_id.to_owned()));
                links.insert((new_node_id.to_owned(), next));
            }
        }
    }
    return (nodes, links);
}

fn main() {
    match OsmToGraph::from_args() {
        OsmToGraph::Format => format_xml(),
        OsmToGraph::Nodes => extract_nodes(),
        OsmToGraph::Links => extract_links(),
        OsmToGraph::Ways => extract_ways(),
        OsmToGraph::Heuristics { delta } => {
            let (mut nodes, mut links) = load_graph();
            metrics(&nodes, &links, ("0", delta.to_string()));
            (nodes, links) = remove_degree_two_nodes(nodes, links);
            metrics(&nodes, &links, ("1", delta.to_string()));
            (nodes, links) = remove_under_delta_nodes(nodes, links, delta);
            metrics(&nodes, &links, ("2", delta.to_string()));
            (nodes, links) = remove_under_delta_links(nodes, links, delta);
            metrics(&nodes, &links, ("3", delta.to_string()));
            (nodes, links) = bfs_connected_components_distribution_and_largest(&nodes, &links);
            metrics(&nodes, &links, ("4", delta.to_string()));
            (nodes, links) = discretize(nodes, links, delta);
            metrics(&nodes, &links, ("5", delta.to_string()));
            print_graph(&nodes, &links);
        }
    }
}
