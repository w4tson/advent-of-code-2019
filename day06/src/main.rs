use std::str::FromStr;
use crate::graph::{Graph, NodeData};
use std::fs;
use std::fmt::{Display, Formatter, Error};

pub mod graph;

const ROOT : &str =  "COM";

fn main() {
    let input : String = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    eprintln!("input = {:#?}", input);

    let graph: Graph<String> = input.parse().expect("Not a graph");
    
    //no longer works after part2 since the directionality was flipped to make back tracking easy 
    let orbits = total_orbits(&graph);
    eprintln!("orbits = {:#?}", orbits);

    let transfers = orbital_transfer(&graph);
    eprintln!("transfers = {:#?}", transfers);
}

fn orbital_transfer(graph : &Graph<String>) -> usize {
    let you_route = path_to(&graph, "YOU", ROOT);
    let san_route = path_to(&graph, "SAN", ROOT);
    
    // count to a common ancestor from YOU
    let count1 = you_route.iter()
        .take_while(|&&node| !san_route.iter().any(|&n| n.node == node.node))
        .count();

    // count to a common ancestor from SAN
    let count2 = san_route.iter()
        .take_while(|&&node| !you_route.iter().any(|&n| n.node == node.node))
        .count();
     
    count1 + count2 - 2
}

fn total_orbits(graph: &Graph<String>) -> usize {
    let root = graph.find_node(&ROOT.to_string()).expect("Missing root");
    let mut visited = vec![];
    total_orbits_recur(graph, &mut visited, root, 0)
}

fn  path_to<'graph>(graph: &'graph Graph<String>, source: &str, target: &str) -> Vec<&'graph NodeData<String>> {
    let root = graph.find_node(&source.to_string()).expect("Missing root");
    let mut visited = vec![];
    path_to_recur(graph, target, &mut visited, root, vec![])
}

fn path_to_recur<'graph>(graph: &'graph Graph<String>, target: &str, visited: &mut Vec<usize>, next: usize, path: Vec<&'graph NodeData<String>>) -> Vec<&'graph NodeData<String>> {
    visited.push(next);
    let mut new_path = path.clone();
    let mut result = vec![];
    let node = graph.node_at(next);
    new_path.push(node);

    if &node.node == target {
        result = new_path;
    } else {
        for successor in graph.successors(next) {
            if !visited.contains(&successor) {
                let mut successor_path = path_to_recur(graph, target, visited, successor, new_path.clone());
                result.append(&mut successor_path);
            }
        }
    }
    result
}

fn total_orbits_recur(graph: &Graph<String>, visited: &mut Vec<usize>, next: usize, depth: usize) -> usize {
    visited.push(next);
    let mut total = depth;

    for successor in graph.successors(next) {
        if !visited.contains(&successor) {
            total += total_orbits_recur(graph, visited, successor, depth + 1);
        }
    }

    total
}

impl FromStr for Graph<String> {
    type Err = ();
    

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut g : Graph<String> = Default::default();
        s.lines()
            .for_each(|line| {
                let nodes = line.split(")").collect::<Vec<&str>>();
                
                let node1_index = g.add_node(nodes[1].to_string());
                let node2_index = g.add_node(nodes[0].to_string());
                g.add_edge(node1_index, node2_index)
            });
        
        Ok(g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::NodeIndex;

    const INPUT: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    #[test]
    fn graph() {
        let test_graph: Graph<String> = INPUT.parse().expect("Not a graph");

        assert_eq!(test_graph.successors(1).collect::<Vec<_>>().len(), 2);
    }
    
    #[test]
    fn count() {
        let test_graph: Graph<String> = INPUT.parse().expect("Not a graph");
        let total_orbits = total_orbits(&test_graph);
        assert_eq!(total_orbits, 42);
    }
    
    const INPUT2 : &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
    
    #[test]
    fn test2() {
        let test_graph: Graph<String> = INPUT2.parse().expect("Not a graph");
        let total_orbits = total_orbits(&test_graph);
    }
    
    #[test]
    fn path() {
        let graph: Graph<String> = INPUT2.parse().expect("Not a graph");
        let all = path_to(&graph, "H", ROOT);
        
        for node in all {
            eprintln!("node = {}", node);
        }
    }
    
    #[test]
    fn orbital_transfore() {
        let graph: Graph<String> = INPUT2.parse().expect("Not a graph");
        let all = orbital_transfer(&graph);

        
    }
}

impl Display for NodeData<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.node)
    }
}



