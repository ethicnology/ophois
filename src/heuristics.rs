use crate::haversine_distance;
use crate::Graph;
use crate::Node;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn bfs_largest_component(graph: Graph) -> Graph {
    let mut queue: VecDeque<String> = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut distribution: HashMap<u32, u32> = HashMap::new();
    let mut largest_component: Graph = Graph::new();
    let mut largest_component_size: u32 = 0;
    for (node_id, _) in graph.nodes.iter() {
        if !visited.contains(node_id) {
            let mut component_size: u32 = 0;
            let mut current_component_nodes: HashMap<String, Node> = HashMap::new();
            queue.push_back(node_id.clone());
            visited.insert(node_id.clone());
            while !queue.is_empty() {
                component_size += 1;
                let node = graph.get_node(&queue.pop_front().unwrap());
                current_component_nodes.insert(
                    node.id.clone(),
                    Node {
                        id: node.id.clone(),
                        latitude: node.latitude.clone(),
                        longitude: node.longitude.clone(),
                        neighbours: Vec::new(),
                    },
                );
                for neighbour_id in &node.neighbours {
                    if !visited.contains(neighbour_id) {
                        visited.insert(neighbour_id.clone());
                        queue.push_back(neighbour_id.clone());
                    }
                }
            }
            if component_size > largest_component_size {
                largest_component_size = component_size;
                largest_component.nodes = current_component_nodes;
            }
            distribution.entry(component_size).or_insert(0);
            distribution.insert(component_size, distribution[&component_size] + 1);
        }
    }
    for (node_id, _) in largest_component.nodes.clone() {
        let old_neighbours = graph.get_node(&node_id).neighbours.clone();
        for neighbour_id in old_neighbours {
            if largest_component.nodes.contains_key(&neighbour_id) {
                largest_component.insert_link((node_id.clone(), neighbour_id.clone()));
            }
        }
    }
    return largest_component;
}

pub fn replace_node_by_links(mut graph: Graph, node_id: &str) -> Graph {
    let neighbours = graph.nodes.get(node_id).unwrap().neighbours.clone();
    graph.remove_node(&node_id);
    for i in 0..neighbours.len() {
        for j in i + 1..neighbours.len() {
            graph.insert_link((neighbours[i].clone(), neighbours[j].clone()));
            graph.insert_link((neighbours[j].clone(), neighbours[i].clone()));
        }
    }

    return graph;
}

pub fn remove_degree_two_nodes(mut graph: Graph) -> Graph {
    let mut degree_two_nodes: Vec<String> = Vec::new();
    for (node_id, node) in graph.nodes.iter() {
        if node.neighbours.len() == 2 {
            degree_two_nodes.push(node_id.clone());
        }
    }
    for to_delete in degree_two_nodes {
        graph = replace_node_by_links(graph, &to_delete);
    }
    return graph;
}

pub fn remove_under_delta_nodes(mut graph: Graph, delta: f32) -> Graph {
    let mut nodes: Vec<String> = graph.nodes.keys().cloned().collect();
    nodes.shuffle(&mut thread_rng());
    for node_id in nodes {
        let node = graph.get_node(&node_id);
        let mut remove = true;
        for neighbour_id in &node.neighbours {
            if graph
                .links
                .contains_key(&(node_id.clone(), neighbour_id.clone()))
            {
                let neighbour = graph.get_node(neighbour_id);
                let distance = haversine_distance(&node.point(), &neighbour.point());
                if distance > delta {
                    remove = false;
                    break;
                }
            }
        }
        if remove {
            graph = replace_node_by_links(graph, &node_id);
        }
    }
    return graph;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bfs_and_largest_component() {
        let mut graph = Graph::from("21658501␟48.8279975␟2.3518307\n21658502␟48.8279276␟2.3513732\n92192237␟48.8275872␟2.3490245\n1829061602␟48.8275089␟2.3484223\n1829061607␟48.8278868␟2.347252\n1829061610␟48.8260051␟2.3474783\n1829061640␟48.827773␟2.3503086\n1829061642␟48.8278201␟2.3506517\n1829061648␟48.8277624␟2.3502336\n1829061667␟48.8265177␟2.3501273\n1829061676␟48.8269249␟2.348167\n1852590201␟48.8276523␟2.3494784\n2268836829␟48.8276001␟2.3486802\n2286779145␟48.8260569␟2.3475149\n2286779154␟48.8276739␟2.3496385\n2576426847␟48.8273391␟2.3487858\n2576426850␟48.8274242␟2.3486471\n2576426851␟48.8274323␟2.3487423\n2576426852␟48.8274347␟2.3487671\n2576426853␟48.8274352␟2.348721\n2576426854␟48.8274412␟2.3487844\n2576426855␟48.827493␟2.3485442\n2576426856␟48.8275026␟2.3485468\n2576426858␟48.8275464␟2.3489207\n2576426859␟48.8275541␟2.3489099\n2597215157␟48.8265578␟2.3500902\n2598270008␟48.8276879␟2.349736\n3758221284␟48.8273411␟2.3486982\n3758221292␟48.8274025␟2.3486929\n3758221295␟48.8275185␟2.3484976\n3758221301␟48.8275751␟2.3489308\n3761637482␟48.8274512␟2.3486719\n3761637486␟48.8275249␟2.348704\n3761637488␟48.8275416␟2.3486683\n3761637489␟48.8275453␟2.348698\n3761637490␟48.8275499␟2.348735\n3761637496␟48.8278544␟2.3473522\n6400885441␟48.8274338␟2.3488187\n6400933176␟48.8268914␟2.3481419\n1829061610␟2286779145\n2286779145␟6400933176\n6400933176␟1829061676\n1829061676␟3758221284\n3758221301␟3761637490\n92192237␟1852590201\n1852590201␟2286779154\n2286779154␟2598270008\n2598270008␟1829061648\n1829061648␟1829061640\n1829061640␟1829061642\n1829061642␟21658502\n21658502␟21658501\n3758221292␟2576426850\n1829061602␟3761637496\n3761637496␟1829061607\n1829061667␟2597215157\n2597215157␟2576426847\n2576426854␟2576426852\n2576426852␟2576426851\n2576426851␟2576426853\n2576426853␟3761637482\n3761637482␟2576426855\n2576426855␟2576426856\n2576426856␟3761637486\n3761637486␟2576426859\n2576426859␟2576426858\n2576426858␟2576426854\n3761637490␟3761637489\n3761637489␟3761637488\n3761637488␟3758221295\n2268836829␟3761637489\n3761637489␟3761637486\n3761637486␟3761637482\n3761637482␟2576426850\n3758221292␟2576426853", '␟');
        graph = bfs_largest_component(graph);
        assert!(graph.nodes.len() == 18);
        assert!(graph.links.len() == 40);
        assert!(graph.nodes.contains_key("3758221295"));
        assert!(graph.get_node("3758221295").neighbours.len() == 1);
        assert!(graph.nodes.contains_key("3761637488"));
        assert!(graph.get_node("3761637488").neighbours.len() == 2);
        assert!(graph.nodes.contains_key("3761637489"));
        assert!(graph.get_node("3761637489").neighbours.len() == 4);
        assert!(graph.nodes.contains_key("2268836829"));
        assert!(graph.get_node("2268836829").neighbours.len() == 1);
        assert!(graph.nodes.contains_key("3761637490"));
        assert!(graph.get_node("3761637490").neighbours.len() == 2);
        assert!(graph.nodes.contains_key("3758221301"));
        assert!(graph.get_node("3758221301").neighbours.len() == 1);
        assert!(graph.nodes.contains_key("3761637486"));
        assert!(graph.get_node(&"3761637486").neighbours.len() == 4);
        assert!(graph.nodes.contains_key("2576426856"));
        assert!(graph.get_node("2576426856").neighbours.len() == 2);
        assert!(graph.nodes.contains_key("2576426855"));
        assert!(graph.get_node("2576426855").neighbours.len() == 2);
        assert!(graph.nodes.contains_key("3761637482"));
        assert!(graph.get_node("3761637482").neighbours.len() == 4);
        assert!(graph.nodes.contains_key("2576426850"));
        assert!(graph.get_node("2576426850").neighbours.len() == 2);
        assert!(graph.nodes.contains_key("3758221292"));
        assert!(graph.get_node("3758221292").neighbours.len() == 2);
        assert!(graph.nodes.contains_key("2576426853"));
        assert!(graph.get_node("2576426853").neighbours.len() == 3);
        assert!(graph.nodes.contains_key("2576426851"));
        assert!(graph.get_node("2576426851").neighbours.len() == 2);
        assert!(graph.nodes.contains_key("2576426852"));
        assert!(graph.get_node("2576426852").neighbours.len() == 2);
        assert!(graph.nodes.contains_key("2576426854"));
        assert!(graph.get_node("2576426854").neighbours.len() == 2);
        assert!(graph.nodes.contains_key("2576426858"));
        assert!(graph.get_node("2576426858").neighbours.len() == 2);
        assert!(graph.nodes.contains_key("2576426859"));
        assert!(graph.get_node("2576426859").neighbours.len() == 2);
    }
    #[test]
    fn test_replace_node_by_links() {
        let mut graph = Graph::from("3761637488␟48.8275416␟2.3486683\n3761637486␟48.8275249␟2.348704\n3761637489␟48.8275453␟2.348698\n3761637490␟48.8275499␟2.348735\n2268836829␟48.8276001␟2.3486802\n3761637489␟3761637488\n3761637489␟2268836829\n3761637489␟3761637490\n3761637489␟3761637486", '␟');
        graph = replace_node_by_links(graph, "3761637489");
        assert!(graph.nodes.len() == 4);
        assert!(graph.links.len() == 12);
        assert!(graph.nodes.contains_key("3761637488"));
        assert!(graph.get_node("3761637488").neighbours.len() == 3);
        assert!(graph.nodes.contains_key("3761637486"));
        assert!(graph.get_node("3761637486").neighbours.len() == 3);
        assert!(graph.nodes.contains_key("3761637490"));
        assert!(graph.get_node("3761637490").neighbours.len() == 3);
        assert!(graph.nodes.contains_key("2268836829"));
        assert!(graph.get_node("2268836829").neighbours.len() == 3);
    }

    #[test]
    fn test_remove_degree_two_nodes() {
        let mut graph = Graph::from("2576426859␟48.8275541␟2.3489099\n2576426853␟48.8274352␟2.348721\n3761637489␟48.8275453␟2.348698\n2576426856␟48.8275026␟2.3485468\n3758221284␟48.8273411␟2.3486982\n92192237␟48.8275872␟2.3490245\n3761637486␟48.8275249␟2.348704\n3761637488␟48.8275416␟2.3486683\n1829061602␟48.8275089␟2.3484223\n3758221301␟48.8275751␟2.3489308\n2268836829␟48.8276001␟2.3486802\n2576426850␟48.8274242␟2.3486471\n3761637482␟48.8274512␟2.3486719\n2576426858␟48.8275464␟2.3489207\n6400885441␟48.8274338␟2.3488187\n3758221295␟48.8275185␟2.3484976\n1852590201␟48.8276523␟2.3494784\n2576426854␟48.8274412␟2.3487844\n2576426851␟48.8274323␟2.3487423\n3758221292␟48.8274025␟2.3486929\n1829061614␟48.8273732␟2.3487375\n2576426855␟48.827493␟2.3485442\n2576426852␟48.8274347␟2.3487671\n3761637490␟48.8275499␟2.348735\n3761637496␟48.8278544␟2.3473522\n2576426847␟48.8273391␟2.3487858\n3758221301␟92192237\n2576426855␟3761637482\n1829061614␟3758221284\n1829061602␟3761637496\n1852590201␟92192237\n1829061614␟6400885441\n2576426853␟3761637482\n2576426851␟2576426852\n2576426850␟3761637482\n2576426855␟2576426856\n3758221301␟3761637490\n3761637482␟3761637486\n6400885441␟92192237\n3761637488␟3761637489\n1829061614␟3758221292\n1829061602␟2576426850\n3758221295␟3761637488\n3761637486␟3761637489\n2576426853␟3758221292\n1829061614␟2576426847\n3761637489␟3761637490\n2576426858␟2576426859\n2576426856␟3761637486\n2576426851␟2576426853\n2576426859␟3761637486\n1829061602␟3758221295\n2576426852␟2576426854\n2268836829␟3761637489\n2576426850␟3758221292\n2576426854␟2576426858", '␟');
        graph = remove_degree_two_nodes(graph);
        assert!(graph.nodes.len() == 14);
        assert!(graph.links.len() == 34);
        assert!(graph.nodes.contains_key("1852590201"));
        assert!(graph.nodes.contains_key("92192237"));
        assert!(graph.nodes.contains_key("3761637489"));
        assert!(graph.nodes.contains_key("2268836829"));
        assert!(graph.nodes.contains_key("3761637486"));
        assert!(graph.nodes.contains_key("3761637482"));
        assert!(graph.nodes.contains_key("2576426853"));
        assert!(graph.nodes.contains_key("2576426850"));
        assert!(graph.nodes.contains_key("3758221292"));
        assert!(graph.nodes.contains_key("1829061614"));
        assert!(graph.nodes.contains_key("3758221284"));
        assert!(graph.nodes.contains_key("2576426847"));
        assert!(graph.nodes.contains_key("1829061602"));
        assert!(graph.nodes.contains_key("3761637496"));
    }
    #[test]
    fn test_remove_under_delta_nodes() {
        let mut graph = Graph::from("2576426859␟48.8275541␟2.3489099\n2576426853␟48.8274352␟2.348721\n3761637489␟48.8275453␟2.348698\n2576426856␟48.8275026␟2.3485468\n3758221284␟48.8273411␟2.3486982\n92192237␟48.8275872␟2.3490245\n3761637486␟48.8275249␟2.348704\n3761637488␟48.8275416␟2.3486683\n1829061602␟48.8275089␟2.3484223\n3758221301␟48.8275751␟2.3489308\n2268836829␟48.8276001␟2.3486802\n2576426850␟48.8274242␟2.3486471\n3761637482␟48.8274512␟2.3486719\n2576426858␟48.8275464␟2.3489207\n6400885441␟48.8274338␟2.3488187\n3758221295␟48.8275185␟2.3484976\n1852590201␟48.8276523␟2.3494784\n2576426854␟48.8274412␟2.3487844\n2576426851␟48.8274323␟2.3487423\n3758221292␟48.8274025␟2.3486929\n1829061614␟48.8273732␟2.3487375\n2576426855␟48.827493␟2.3485442\n2576426852␟48.8274347␟2.3487671\n3761637490␟48.8275499␟2.348735\n3761637496␟48.8278544␟2.3473522\n2576426847␟48.8273391␟2.3487858\n3758221301␟92192237\n2576426855␟3761637482\n1829061614␟3758221284\n1829061602␟3761637496\n1852590201␟92192237\n1829061614␟6400885441\n2576426853␟3761637482\n2576426851␟2576426852\n2576426850␟3761637482\n2576426855␟2576426856\n3758221301␟3761637490\n3761637482␟3761637486\n6400885441␟92192237\n3761637488␟3761637489\n1829061614␟3758221292\n1829061602␟2576426850\n3758221295␟3761637488\n3761637486␟3761637489\n2576426853␟3758221292\n1829061614␟2576426847\n3761637489␟3761637490\n2576426858␟2576426859\n2576426856␟3761637486\n2576426851␟2576426853\n2576426859␟3761637486\n1829061602␟3758221295\n2576426852␟2576426854\n2268836829␟3761637489\n2576426850␟3758221292\n2576426854␟2576426858", '␟');
    }
}
