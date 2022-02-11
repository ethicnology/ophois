use crate::Point;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

pub type Link = (String, String);

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct Node {
    pub id: String,
    pub longitude: String,
    pub latitude: String,
    pub neighbours: Vec<String>,
}

impl Node {
    pub fn _new(id: String) -> Node {
        return Node {
            id: id,
            latitude: "".to_string(),
            longitude: "".to_string(),
            neighbours: Vec::new(),
        };
    }

    pub fn point(&self) -> Point {
        return Point {
            x: self.longitude.parse().unwrap(),
            y: self.latitude.parse().unwrap(),
        };
    }
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub links: HashMap<(String, String), usize>,
}

impl Graph {
    pub fn new() -> Graph {
        return Graph {
            nodes: HashMap::new(),
            links: HashMap::new(),
        };
    }

    pub fn _from(input: &str, separator: char) -> Graph {
        let mut graph = Graph::new();
        for line in input.lines() {
            let data: Vec<&str> = line.split(separator).collect();
            match data.len() {
                3 => {
                    graph.insert_node(Node {
                        id: data[0].to_string(),
                        latitude: data[1].to_string(),
                        longitude: data[2].to_string(),
                        neighbours: Vec::new(),
                    });
                }
                2 => {
                    graph.insert_link((data[0].to_string(), data[1].to_string()));
                    graph.insert_link((data[1].to_string(), data[0].to_string()));
                }
                _ => panic!(
                    "Wrong input length -> {}!\nnode=id\nlink=node_id node_id",
                    data.len()
                ),
            }
        }
        return graph;
    }

    pub fn load(separator: char) -> Graph {
        let mut graph = Graph::new();
        let input = io::stdin();
        for line in input.lock().lines() {
            let line = line.unwrap();
            let data: Vec<&str> = line.split(separator).collect();
            match data.len() {
                3 => {
                    graph.insert_node(Node {
                        id: data[0].to_string(),
                        latitude: data[1].to_string(),
                        longitude: data[2].to_string(),
                        neighbours: Vec::new(),
                    });
                }
                2 => {
                    graph.insert_link((data[0].to_string(), data[1].to_string()));
                    graph.insert_link((data[1].to_string(), data[0].to_string()));
                }
                _ => panic!("Wrong input length!\nnode=id\nlink=node_id node_id"),
            }
        }
        return graph;
    }

    pub fn show(&self, separator: char) {
        for (_, node) in self.nodes.iter() {
            println!(
                "{}{}{}{}{}",
                node.id, separator, node.latitude, separator, node.longitude
            )
        }
        let mut printed: HashSet<(String, String)> = HashSet::new();
        for (link, _) in self.links.iter() {
            let formatted_link = if link.0 < link.1 {
                (link.0.to_owned(), link.1.to_owned())
            } else {
                (link.1.to_owned(), link.0.to_owned())
            };
            if !printed.contains(&formatted_link) {
                println!("{}{}{}", formatted_link.0, separator, formatted_link.1);
                printed.insert(formatted_link);
            }
        }
    }

    pub fn insert_node(&mut self, node: Node) {
        self.nodes.entry(node.id.clone()).or_insert(node);
    }

    pub fn remove_node(&mut self, node_id: &str) {
        let node = self.nodes.get(node_id).unwrap().clone();
        if node.neighbours.len() > 0 {
            for neighbour_id in node.neighbours {
                self.remove_link(&(node_id.to_string(), neighbour_id.clone()));
                self.remove_link(&(neighbour_id, node_id.to_string()));
            }
        }
        self.nodes.remove(node_id);
    }

    pub fn get_node(&self, node_id: &str) -> &Node {
        return self.nodes.get(node_id).unwrap();
    }

    pub fn insert_link(&mut self, link: Link) {
        let (source_id, target_id) = link;
        let source = self.nodes.get_mut(&source_id).unwrap();
        source.neighbours.push(target_id.clone());
        self.links
            .insert((source_id, target_id), source.neighbours.len() - 1);
    }

    pub fn remove_link(&mut self, link: &Link) {
        let index = self.links.get(&link).unwrap().clone();
        self.links.remove(&link);
        let (source, _) = link;
        let node = self.nodes.get_mut(source).unwrap();
        if node.neighbours.len() == 0 || index == node.neighbours.len() - 1 {
            node.neighbours.swap_remove(index);
        } else {
            node.neighbours.swap_remove(index);
            let swapped_node = &node.neighbours[index];
            self.links
                .insert((source.to_string(), swapped_node.to_string()), index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert_node() {
        let mut graph = Graph::new();
        let node_id = "u".to_string();
        graph.insert_node(Node::_new(node_id.clone()));
        let is_contained = graph.nodes.contains_key(&node_id);
        assert_eq!(is_contained, true);
    }
    #[test]
    fn get_node() {
        let mut graph = Graph::new();
        let u = "u".to_string();
        let v = "v".to_string();
        graph.insert_node(Node::_new(u.clone()));
        graph.insert_node(Node::_new(v.clone()));
        graph.insert_link((u.clone(), v.clone()));
        graph.insert_link((v.clone(), u.clone()));
        assert_eq!(graph.get_node(&u).neighbours[0], v);
    }
    #[test]
    fn insert_link() {
        let mut graph = Graph::new();
        let u = "u".to_string();
        let v = "v".to_string();
        let w = "w".to_string();
        graph.insert_node(Node::_new(u.clone()));
        graph.insert_node(Node::_new(v.clone()));
        graph.insert_node(Node::_new(w.clone()));
        graph.insert_link((u.clone(), v.clone()));
        graph.insert_link((v.clone(), u.clone()));
        graph.insert_link((v.clone(), w.clone()));
        graph.insert_link((w.clone(), v.clone()));
        assert_eq!(graph.links.contains_key(&(u.clone(), v.clone())), true);
        assert_eq!(graph.links.contains_key(&(v.clone(), u.clone())), true);
        assert_eq!(graph.links.contains_key(&(v.clone(), w.clone())), true);
        assert_eq!(graph.links.contains_key(&(w.clone(), v.clone())), true);
        assert_eq!(graph.links.get(&(u.clone(), v.clone())).unwrap(), &0);
        assert_eq!(graph.links.get(&(u, v.clone())).unwrap(), &0);
        assert_eq!(graph.links.get(&(v, w)).unwrap(), &1);
    }

    #[test]
    fn remove_node_without_neighbours() {
        let mut graph = Graph::new();
        let node_id = "u".to_string();
        graph.insert_node(Node::_new(node_id.clone()));
        graph.remove_node(&node_id);
        let is_contained = graph.nodes.contains_key(&node_id);
        assert_eq!(is_contained, false);
    }
    #[test]
    fn remove_node_with_neighbours() {
        let mut graph = Graph::new();
        let u = "u".to_string();
        let v = "v".to_string();
        let w = "w".to_string();
        graph.insert_node(Node::_new(u.clone()));
        graph.insert_node(Node::_new(v.clone()));
        graph.insert_node(Node::_new(w.clone()));
        graph.insert_link((u.clone(), v.clone()));
        graph.insert_link((v.clone(), u.clone()));
        graph.insert_link((v.clone(), w.clone()));
        graph.insert_link((w.clone(), v.clone()));
        graph.remove_node(&v);
        let empty_vector: Vec<String> = vec![];
        assert_eq!(graph.get_node(&u).neighbours, empty_vector);
        assert_eq!(graph.get_node(&w).neighbours, empty_vector);
        assert_eq!(graph.nodes.contains_key(&v), false);
        assert_eq!(graph.links.contains_key(&(u.clone(), v.clone())), false);
        assert_eq!(graph.links.contains_key(&(v.clone(), u.clone())), false);
        assert_eq!(graph.links.contains_key(&(v.clone(), w.clone())), false);
        assert_eq!(graph.links.contains_key(&(w, v)), false);
    }
}
