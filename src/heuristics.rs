use crate::deterministic_link;
use crate::distribution_to_file;
use crate::get_point_from_line;
use crate::haversine_distance;
use crate::midpoint;
use crate::Node;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Links = HashSet<(String, String)>;
type Nodes = HashMap<String, Node>;

pub fn remove_degree_two_nodes(mut nodes: Nodes, mut links: Links) -> (Nodes, Links) {
    let mut two_degree_nodes: Vec<String> = Vec::new();
    for (node_id, node) in nodes.iter() {
        if node.neighbours.len() == 2 {
            two_degree_nodes.push(node_id.clone());
        }
    }
    for to_delete in two_degree_nodes {
        (nodes, links) = replace_node_by_links(nodes, links, to_delete.clone());
    }
    return (nodes, links);
}

pub fn remove_degree_one_nodes(mut nodes: Nodes, mut links: Links) -> (Nodes, Links) {
    let mut still = true;
    while still == true {
        let mut one_degree_nodes: Vec<String> = Vec::new();
        for (node_id, node) in nodes.iter() {
            if node.neighbours.len() == 1 {
                one_degree_nodes.push(node_id.clone());
            }
        }
        for to_delete in one_degree_nodes {
            let node = nodes.get(&to_delete).unwrap().clone();
            nodes.remove(&to_delete);
            links.remove(&deterministic_link(&to_delete, &node.neighbours[0]));
        }

        let mut remain = false;
        for (_, node) in nodes.iter() {
            if node.neighbours.len() == 1 {
                remain = true;
            }
        }
        if remain == false {
            still = false
        }
    }
    return (nodes, links);
}

pub fn remove_under_delta_nodes(mut nodes: Nodes, mut links: Links, delta: f32) -> (Nodes, Links) {
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

pub fn remove_under_delta_links(mut nodes: Nodes, mut links: Links, delta: f32) -> (Nodes, Links) {
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
    let id = deterministic_link(&source.id, &target.id);
    let new_node_id = format!("{}-{}", id.0, id.1); // non deterministic id -> duplicate risks
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

pub fn bfs_connected_components_distribution_and_largest(
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
                    }
                }
            }
            if component_size > largest_component_size {
                largest_component_size = component_size;
                largest_component_nodes = current_component_nodes;
            }
            distribution.entry(component_size).or_insert(0);
            distribution.insert(component_size, distribution[&component_size] + 1);
        }
    }
    for (node_id, node) in largest_component_nodes.iter() {
        for neighbour_id in &node.neighbours {
            if links.contains(&deterministic_link(node_id, neighbour_id)) {
                largest_component_links.insert(deterministic_link(node_id, neighbour_id));
            }
        }
    }
    distribution_to_file("connected_components_distribution", distribution);
    return (largest_component_nodes, largest_component_links);
}

pub fn discretize(mut nodes: Nodes, mut links: Links, delta: f32) -> (Nodes, Links) {
    let links_clone = links.clone();
    for link in links_clone {
        let source = nodes.get(&link.0).unwrap().clone();
        let target = nodes.get(&link.1).unwrap().clone();
        let distance = haversine_distance(&source.point(), &target.point());
        if distance >= 2.0 * delta {
            links.remove(&deterministic_link(&source.id.clone(), &target.id.clone()));
            let part = (distance / delta) as u32;
            let mut new_nodes = Vec::new();
            for i in 1..part {
                let point =
                    get_point_from_line(&source.point(), &target.point(), i as f32 / part as f32);
                let id = deterministic_link(&source.id, &target.id);
                let node = Node {
                    id: format!("{}-{}:{}/{}", id.0, id.1, i, part),
                    longitude: point.x.to_string(),
                    latitude: point.y.to_string(),
                    neighbours: Vec::new(),
                };
                new_nodes.push(node.id.clone());
                nodes.entry(node.id.clone()).or_insert(node);
            }
            for j in 1..part {
                let new_node_id = &new_nodes[(j - 1) as usize];
                let mut previous = format!("{}-{}:{}/{}", source.id, target.id, j - 1, part);
                let mut next = format!("{}-{}:{}/{}", source.id, target.id, j + 1, part);
                if j == 1 {
                    previous = source.id.clone();
                }
                if j == part - 1 {
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
