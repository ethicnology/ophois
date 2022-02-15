use quick_xml::de::from_str;
use serde::Deserialize;
use std::io;
use std::io::prelude::*;

#[derive(Deserialize)]
struct Node {
    id: String,
    lat: String,
    lon: String,
}

#[derive(Deserialize)]
struct Ways {
    #[serde(rename = "nd")]
    nodes: Vec<NodeRef>,
}

#[derive(Deserialize)]
struct NodeRef {
    r#ref: String,
}

pub fn format_xml() {
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

fn extract_node(line: String, separator: char) -> String {
    let node: Node = from_str(&line).unwrap();
    return format!(
        "{}{}{}{}{}",
        node.id, separator, node.lat, separator, node.lon
    );
}

fn extract_link(line: String, separator: char) -> String {
    let way: Ways = from_str(&line).unwrap();
    let nodes = way.nodes;
    let mut output: Vec<String> = vec![];
    for i in 0..nodes.len() - 1 {
        output.push(format!(
            "{}{}{}\n",
            nodes[i].r#ref,
            separator,
            nodes[i + 1].r#ref
        ));
    }
    return output.join("");
}

pub fn extract(line: String, separator: char) {
    if line.starts_with("<node") {
        println!("{}", extract_node(line, separator));
    } else if line.starts_with("<way") {
        print!("{}", extract_link(line, separator));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_extract_node() {
        let line = "<node id=\"618904\" lat=\"50.6011263\" lon=\"3.2519549\"/>";
        assert!(extract_node(line.to_owned(), '␟') == "618904␟50.6011263␟3.2519549")
    }

    #[test]
    fn test_extract_link() {
        let line = "<way id=\"951505353\"><nd ref=\"8807254574\"/><nd ref=\"8807254575\"/><nd ref=\"8507963130\"/><tag k=\"highway\" v=\"residential\"/></way>";
        let expected = "8807254574␟8807254575\n8807254575␟8507963130\n";
        assert!(extract_link(line.to_owned(), '␟') == expected)
    }
}
