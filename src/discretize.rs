use crate::Graph;
use crate::Node;
use crate::{determinist, get_point_from_line, haversine_distance};

pub fn discretize(mut graph: Graph, delta: f64) -> Graph {
    let links_clone = graph.links.clone();
    for ((u, v), _) in links_clone {
        if graph.contains_link((&u, &v)) && graph.contains_link((&v, &u)) {
            let source = graph.get_node(&u).clone();
            let target = graph.get_node(&v).clone();
            let distance = haversine_distance(&source.point(), &target.point());
            let part = (distance / delta) as u32;
            if part > 1 {
                graph.remove_link(&(u.clone(), v.clone()));
                graph.remove_link(&(v.clone(), u.clone()));
                let mut new_nodes = Vec::new();
                let new_id = determinist(u.clone(), v.clone());
                for i in 1..part {
                    let point = get_point_from_line(
                        &source.point(),
                        &target.point(),
                        i as f64 / part as f64,
                    );
                    let node = Node {
                        id: format!("{}-{}:{}/{}", new_id.0, new_id.1, i, part),
                        longitude: point.x.to_string(),
                        latitude: point.y.to_string(),
                        neighbours: Vec::new(),
                    };
                    new_nodes.push(node.id.clone());
                    graph.insert_node(node);
                }
                for j in 1..part {
                    let new_node_id = &new_nodes[(j - 1) as usize];
                    let mut previous = format!("{}-{}:{}/{}", new_id.0, new_id.1, j - 1, part);
                    let mut next = format!("{}-{}:{}/{}", new_id.0, new_id.1, j + 1, part);
                    if j == 1 {
                        previous = source.id.clone();
                        graph.insert_link((previous.clone(), new_node_id.clone()));
                    }
                    if j == part - 1 {
                        next = target.id.clone();
                        graph.insert_link((next.clone(), new_node_id.clone()));
                    }
                    graph.insert_link((new_node_id.clone(), previous.clone()));
                    graph.insert_link((new_node_id.clone(), next.clone()));
                }
            }
        }
    }
    return graph;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_discretize() {
        let mut graph = Graph::_from("92192237␟48.8275872␟2.3490245\n3761637486-3761637489␟48.8275351␟2.348701\n1852590201␟48.8276523␟2.3494784\n3761637496␟48.8278544␟2.3473522\n1829061602␟48.8275089␟2.3484223\n2576426850-2576426853␟48.827429699999996␟2.34868405\n1829061614␟48.8273732␟2.3487375\n1829061614␟2576426850-2576426853\n3761637486-3761637489␟92192237\n1829061614␟92192237\n1829061602␟3761637496\n1829061602␟2576426850-2576426853\n2576426850-2576426853␟3761637486-3761637489\n1829061602␟3761637486-3761637489\n1852590201␟92192237", '␟');
        graph = discretize(graph, 6.0);
        assert!(graph.nodes.len() == 35);
        assert!(graph.links.len() / 2 == 36);
        let expected: Vec<(&str, usize)> = vec![
            ("3761637496", 1),
            ("1829061602-3761637496:1/14", 2),
            ("1829061602-3761637496:2/14", 2),
            ("1829061602-3761637496:3/14", 2),
            ("1829061602-3761637496:4/14", 2),
            ("1829061602-3761637496:5/14", 2),
            ("1829061602-3761637496:6/14", 2),
            ("1829061602-3761637496:7/14", 2),
            ("1829061602-3761637496:8/14", 2),
            ("1829061602-3761637496:9/14", 2),
            ("1829061602-3761637496:10/14", 2),
            ("1829061602-3761637496:11/14", 2),
            ("1829061602-3761637496:12/14", 2),
            ("1829061602-3761637496:13/14", 2),
            ("1829061602", 3),
            ("1829061602-3761637486-3761637489:1/3", 2),
            ("1829061602-3761637486-3761637489:2/3", 2),
            ("2576426850-2576426853", 3),
            ("1829061602-2576426850-2576426853:1/3", 2),
            ("1829061602-2576426850-2576426853:2/3", 2),
            ("3761637486-3761637489", 3),
            ("3761637486-3761637489-92192237:1/4", 2),
            ("3761637486-3761637489-92192237:2/4", 2),
            ("3761637486-3761637489-92192237:3/4", 2),
            ("1829061614-92192237:1/5", 2),
            ("1829061614-92192237:2/5", 2),
            ("1829061614-92192237:3/5", 2),
            ("1829061614-92192237:4/5", 2),
            ("92192237", 3),
            ("1852590201-92192237:1/5", 2),
            ("1852590201-92192237:2/5", 2),
            ("1852590201-92192237:3/5", 2),
            ("1852590201-92192237:4/5", 2),
            ("1852590201", 1),
            ("1829061614", 2),
        ];
        for (node, degree) in expected {
            assert!(graph.nodes.contains_key(node));
            assert!(graph.get_node(node).neighbours.len() == degree);
        }
    }
}
