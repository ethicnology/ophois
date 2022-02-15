mod discretize;
mod geo;
mod graph;
mod heuristics;
mod metrics;
mod openstreetmap;
mod overpass;
mod utils;

use clap::Parser;
use discretize::*;
use geo::*;
use graph::*;
use heuristics::*;
use metrics::*;
use openstreetmap::*;
use overpass::*;
use std::io;
use std::io::prelude::*;
use utils::*;

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
        /// Specify a custom separator such as space: -s ' '. Beware that data already contains: [.-:/]
        #[clap(short, long, default_value_t = '␟')]
        separator: char,
    },
    Simplify {
        /// Specify a custom separator such as space: -s ' '. Beware that data already contains: [.-:/]
        #[clap(short, long, default_value_t = '␟')]
        separator: char,
        /// Delta is expressed in meters
        #[clap(short, long)]
        delta: f64,
    },
    Discretize {
        /// Specify a custom separator such as space: -s ' '. Beware that data already contains: [.-:/]
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
        Ophois::Simplify { separator, delta } => {
            let mut graph = Graph::load(separator);
            graph = bfs_largest_component(graph);
            graph = remove_degree_two_nodes(graph);
            graph = remove_under_delta_nodes(graph, delta);
            graph = remove_under_delta_links(graph, delta);
            metrics(&graph, format!("simplify={}", delta));
            graph.show(separator);
        }
        Ophois::Discretize { separator, delta } => {
            let mut graph = Graph::load(separator);
            graph = discretize(graph, delta);
            metrics(&graph, format!("discretize={}", delta));
            graph.show(separator);
        }
    }
}
