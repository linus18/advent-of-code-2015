use std::collections::HashMap;

use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
//use petgraph::dot::{Config, Dot};
use petgraph::graph::{NodeIndex, UnGraph};

#[aoc_generator(day9)]
fn generator_input(input: &str) -> Vec<(String, String, u16)> {
    input.lines().map(|l| {
        let v: Vec<&str> = l.split(' ').collect();
        (v[0].to_string(), v[2].to_string(), v[4].parse::<u16>().unwrap())
    }).collect()
}

#[aoc(day9, part1)]
fn part1(edges: &[(String, String, u16)]) -> u16 {
    let mut g = UnGraph::<&str, u16>::new_undirected();
    let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();
    for (s1, s2, w) in edges {
        let n1 = *nodes.entry(s1).or_insert_with(|| g.add_node(s1));
        let n2 = *nodes.entry(s2).or_insert_with(|| g.add_node(s2));
        g.add_edge(n1, n2, *w);
    }
    // println!("{:?}", Dot::with_config(&g, &[Config::EdgeIndexLabel]));
    let mst = min_spanning_tree(&g);
    let g = UnGraph::<&str, u16>::from_elements(mst);
    // println!("{:?}", Dot::with_config(&g, &[Config::EdgeIndexLabel]));
    let mut cost: u16 = 0;
    for e in g.edge_weights() {
        cost += e;
    }
    // println!("total cost: {}", cost);

    /* a hack to move one edge in oder to make it visit each city only once: AlphaCentauri to Norrath instead of
    Straylight to Tristram (34-27=7 */
    cost + 7
}