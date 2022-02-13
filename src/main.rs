mod geo;
mod graph;
mod heuristics;
mod openstreetmap;
mod overpass;

use clap::Parser;
use geo::*;
use graph::*;
use heuristics::*;
use openstreetmap::*;
use overpass::*;
use std::io;
use std::io::prelude::*;

#[derive(Parser)]
#[clap(author, about, version, bin_name = "ophois")]
enum Ophois {
    Download {
        /// Select any available cities/areas in overpass-api: Pantin, Damas, Mexico, Paris, London, Tokyo, Moscow…
        #[clap(short, long)]
        city: String,
        /// ⚠With caution⚠: please learn overpass QL. City variable is stored in 'area'.
        #[clap(short, long, default_value = "(way(area)[highway]; ); (._;>;);")]
        overpassql: String,
    },
    Format,
    Extract {
        /// Specify a custom separator such as space: -s ' '. Beware that data already contains the dot '.' and comma ','
        #[clap(short, long, default_value_t = '␟')]
        separator: char,
    },
    Heuristics {
        /// Specify a custom separator such as space: -s ' '. Beware that data already contains the dot '.' and comma ','
        #[clap(short, long, default_value_t = '␟')]
        separator: char,
        /// Delta is expressed in meters
        #[clap(short, long)]
        delta: f64,
    },
}

fn main() {
    match Ophois::parse() {
        Ophois::Download { city, overpassql } => download_map(city, overpassql).unwrap(),
        Ophois::Format => format_xml(),
        Ophois::Extract { separator } => {
            for line in io::stdin().lock().lines() {
                extract(line.unwrap(), separator);
            }
        }
        Ophois::Heuristics { separator, delta } => {
            let mut graph = Graph::load(separator);
            graph = bfs_largest_component(graph);
            graph = remove_degree_two_nodes(graph);
            graph = remove_under_delta_nodes(graph, delta);
            graph = remove_under_delta_links(graph, delta);
            graph.show(separator);
        }
    }
}
