use crate::Point;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

type Links = HashSet<(String, String)>;
type Nodes = HashMap<String, Node>;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct Node {
    pub id: String,
    pub longitude: String,
    pub latitude: String,
    pub neighbours: Vec<String>,
}

impl Node {
    pub fn point(&self) -> Point {
        return Point {
            x: self.longitude.parse().unwrap(),
            y: self.latitude.parse().unwrap(),
        };
    }
}

pub fn separator() -> char {
    return '␟';
}

pub fn deterministic_link(source: &str, target: &str) -> (String, String) {
    let link = if source < target {
        (source.to_owned(), target.to_owned())
    } else {
        (target.to_owned(), source.to_owned())
    };
    return link;
}

pub fn load_graph() -> (Nodes, Links) {
    let mut nodes: Nodes = HashMap::new();
    let mut links: Links = HashSet::new();
    let input = io::stdin();
    for line in input.lock().lines() {
        let line = line.unwrap();
        let data: Vec<&str> = line.split(separator()).collect();
        match data.len() {
            2 => {
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
            3 => {
                nodes.entry(data[0].to_owned()).or_insert(Node {
                    id: data[0].to_owned(),
                    longitude: data[2].to_owned(),
                    latitude: data[1].to_owned(),
                    neighbours: Vec::new(),
                });
            }
            _ => panic!("Wrong input length!\n expect 2 or 3 and got {}\nnodes: id latitude longitude\nlinks: node_id node_id\n", data.len())
        }
    }
    return (nodes, links);
}

pub fn print_graph(nodes: &Nodes, links: &Links) {
    for (node_id, node) in nodes.iter() {
        println!(
            "{}{}{}{}{}",
            node_id,
            separator(),
            node.latitude,
            separator(),
            node.longitude,
        )
    }
    for link in links {
        let source = &link.0;
        let target = &link.1;
        println!("{}{}{}", source, separator(), target);
    }
}
