use crate::deterministic_link;
use crate::haversine_distance;
use crate::Node;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io::prelude::*;

type Links = HashSet<(String, String)>;
type Nodes = HashMap<String, Node>;

pub fn count_nodes(nodes: &Nodes) -> u32 {
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
    for (node_u, node_v) in links {
        let u = nodes.get(node_u).unwrap();
        let v = nodes.get(node_v).unwrap();
        let distance = haversine_distance(&u.point(), &v.point()) as u32;
        distribution.entry(distance).or_insert(0);
        distribution.insert(distance, distribution[&distance] + 1);
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

pub fn distribution_to_file(file_name: &str, distribution: HashMap<u32, u32>) {
    let mut string: String = "".to_owned();
    for (key, value) in distribution {
        string.push_str(&format!("{} {}\n", key, value))
    }
    values_to_file(file_name, string.as_bytes()).expect("distribution file");
}

fn values_to_file(file_name: &str, values: &[u8]) -> std::io::Result<()> {
    let directory = "./metrics";
    fs::create_dir_all(directory)?;
    let mut file = fs::File::create(format!("./metrics/{}", file_name))?;
    file.write_all(values)?;
    Ok(())
}

pub fn metrics(nodes: &Nodes, links: &Links, param: (&str, String)) {
    let n = count_nodes(&nodes);
    let m = count_links(&links);
    values_to_file(
        &format!("nodes_links_step:{}_delta:{}", param.0, param.1),
        format!("{} {}", n, m).as_bytes(),
    )
    .expect("graph size file");
    let degree = degree_distribution(&nodes, &links);
    let links_length = links_length_distribution(&nodes, &links);
    let substitutes = substitute_nodes_distribution(&nodes);
    distribution_to_file(
        &format!("degree_distribution_step:{}_delta:{}", param.0, param.1),
        degree,
    );
    distribution_to_file(
        &format!(
            "links_length_distribution_step:{}_delta:{}",
            param.0, param.1
        ),
        links_length,
    );
    distribution_to_file(
        &format!(
            "substitutes_distribution_step:{}_delta:{}",
            param.0, param.1
        ),
        substitutes,
    );
}
