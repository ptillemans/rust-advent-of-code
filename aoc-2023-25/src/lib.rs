use rand::Rng;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::UnGraph;
use rustworkx_core::Result;

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Write,
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub graph: HashMap<String, Vec<String>>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
    #[error("IO Error")]
    IOError,
    #[error("No solution found")]
    NoSolution,
}

impl From<std::io::Error> for AocError {
    fn from(value: std::io::Error) -> Self {
        AocError::IOError
    }
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let graph = s
            .lines()
            .map(|line| {
                let parts = line.split(": ").collect::<Vec<_>>();
                if parts.len() != 2 {
                    return Err(AocError::ParseError);
                }
                let node = parts[0].to_string();
                let edges = parts[1]
                    .split(" ")
                    .flat_map(|s| {
                        [
                            (node.to_string(), s.to_string()),
                            (s.to_string(), node.to_string()),
                        ]
                    })
                    .collect::<Vec<_>>();
                Ok(edges)
            })
            .fold(
                Ok(HashMap::new()),
                |acc: Result<HashMap<String, Vec<String>>, Self::Err>, v| {
                    let mut acc = acc?;
                    let v = v?;
                    v.iter().for_each(|(n1, n2)| {
                        let e1 = acc.entry(n1.to_string()).or_insert_with(Vec::new);
                        if !e1.contains(n2) {
                            e1.push(n2.to_string());
                        }
                    });
                    Ok(acc)
                },
            )?;
        Ok(InputModel { graph })
    }
}

pub fn flood_fill(graph: &HashMap<String, Vec<String>>, start: &str) -> HashSet<String> {
    
    let mut seen = HashSet::<String>::new();
    let mut todo = vec![start];
    while let Some(node) = todo.pop() {
        seen.insert(node.to_string());
        if seen.len() == graph.len() {
            break;
        }

        let n = graph
            .get(node);
        if n.is_none() {
            println!("node missing: {:?}", node);
        } else {
            n.unwrap()
                .iter()
                .filter(|node| !seen.contains(*node))
                .for_each(|node| todo.push(node));
        }
    }
    seen
}

fn is_graph_connected(graph: &HashMap<String, Vec<String>>) -> bool {
    let start = graph.keys().next().unwrap();
    let seen = flood_fill(graph, start);
    seen.len() == graph.len()
}

pub fn cut_graph(
    graph: &HashMap<String, Vec<String>>,
    n1: &str,
    n2: &str,
) -> HashMap<String, Vec<String>> {
    let mut new_graph = graph.clone();
    new_graph
        .entry(n1.to_string())
        .or_default()
        .retain_mut(|n| n != n2);
    new_graph
        .entry(n2.to_string())
        .or_default()
        .retain_mut(|n| n != n1);
    new_graph
}

pub fn output_graphviz(graph: &HashMap<String, Vec<String>>) -> Result<(), std::io::Error> {
    let mut f = File::create("graph.dot")?;
    writeln!(f, "digraph {}", "{")?;
    for (n1, ns) in graph.iter() {
        for n in ns {
            writeln!(f, "  {} -> {}", n1, n)?
        }
    }
    writeln!(f, "{}", "}")?;
    Ok(())
}

pub fn find_path(
    graph: &HashMap<String, Vec<String>>,
    start: &str,
    finish: &str,
) -> Result<Vec<String>, AocError> {
    let start = start.to_string();
    let mut todo = vec![(start.clone(), vec![])];
    let mut seen: HashSet<String> = HashSet::new();
    while let Some((node, path)) = todo.pop() {
        let mut path = path.clone();
        path.push(node.clone());
        seen.insert(node.clone());
        if node == finish {
            return Ok(path);
        }

        graph
            .get(&node)
            .unwrap()
            .iter()
            .filter(|node| !path.contains(*node))
            .filter(|node| !seen.contains(*node))
            .for_each(|n| todo.push((n.clone(), path.clone())));
    }

    Err(AocError::NoSolution)
}

pub fn find_most_used_edges(
    graph: &HashMap<String, Vec<String>>,
) -> Result<Vec<(String, String)>, AocError> {
    let mut edge_count: HashMap<(String, String), i32> = HashMap::new();
    let nodes = graph.keys().collect::<Vec<_>>();
    let mut rng = rand::thread_rng();

    for i in 0..1000 {
        let n1 = nodes[rng.gen_range(0..nodes.len())];
        let n2 = nodes[rng.gen_range(0..nodes.len())];
        println!("i: {}, n1: {}, n2: {}", i, n1, n2);
        
        let path = find_path(&graph, n1, n2)?;

        path.iter()
            .zip(path[1..].iter())
            .map(|(n1, n2)| {
                if n1 < n2 {
                    (n1.clone(), n2.clone())
                } else {
                    (n2.clone(), n1.clone())
                }
            })
            .for_each(|p| {
                let v = edge_count.entry(p).or_default();
                *v += 1;
            })
    }

    let mut edges = edge_count.iter().map(|(k, v)| (-v, k)).collect::<Vec<_>>();
    edges.sort();

    println!("best: {:?}", &edges[0..10]);

    let edges = edges[0..3]
        .into_iter()
        .map(|(_, v)| *v)
        .cloned()
        .collect::<Vec<_>>();

    Ok(edges)
}


pub fn mincut_stoer_wagner(graph: &HashMap<String, Vec<String>>) -> (usize, Vec<String>) {

    let edges = graph.into_iter()
        .flat_map(|(k, v)| v.iter().map(|n| (k.clone(), n.clone())))
        .filter(|(a, b)| a < b)
        .to_owned()
        .collect::<Vec<(String, String)>>();

    let mut grf = UnGraph::<String, ()>::new_undirected();

    let mut node_map = HashMap::new();
    for node in graph.keys() {
        node_map.insert(node.clone(), grf.add_node(node.clone()));
    }

    for (n1, n2) in edges {
        let a = node_map.get(&n1).unwrap();
        let b = node_map.get(&n2).unwrap();
        grf.add_edge(*a, *b, ());
    }

    let min_cut_res: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&grf, |_| Ok(1));

    let (min_cut, partition) = min_cut_res.unwrap().unwrap();
    let nodes = partition.iter().map(|ix| grf[*ix].clone()).collect::<Vec<String>>();

    (min_cut, nodes) 
    
}


#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_parse() {
        let input: super::InputModel = TEST_INPUT.parse().unwrap();
        assert_eq!(
            input.graph.get("jqt").unwrap(),
            &vec!["rhn", "xhk", "nvd", "ntq"]
        );
        assert_eq!(input.graph.len(), 15);
    }

    #[test]
    fn test_is_graph_connected() {
        let input: super::InputModel = TEST_INPUT.parse().unwrap();
        let actual = is_graph_connected(&input.graph);
        assert_eq!(actual, true);
    }

    #[test]
    fn test_cut_graph() {
        let input: super::InputModel = TEST_INPUT.parse().unwrap();
        let graph = input.graph;
        println!("graph: {:?}", graph);
        let graph = cut_graph(&graph, "hfx", "pzl");
        let graph = cut_graph(&graph, "bvb", "cmg");
        let graph = cut_graph(&graph, "nvd", "jqt");
        println!("graph: {:?}", graph);
        let f1 = flood_fill(&graph, "hfx");
        println!("f1: {:?}", f1);
        assert_eq!(f1.len(), 6);
        let f2 = flood_fill(&graph, "pzl");
        println!("f2: {:?}", f2);
        assert_eq!(f2.len(), 9);
        let actual = is_graph_connected(&graph);
        assert_eq!(actual, false);
    }

    #[test]
    fn test_find_path() {
        let input: super::InputModel = TEST_INPUT.parse().unwrap();
        let graph = input.graph;
        let actual = find_path(&graph, "ntq", "lsr").unwrap();
        assert_eq!(
            actual,
            vec!["ntq", "xhk", "bvb", "hfx", "pzl", "nvd", "lhk", "frs", "lsr"]
        );
    }

    //[test]
    fn test_most_used_edges() {
        let input: super::InputModel = TEST_INPUT.parse().unwrap();
        let graph = input.graph;
        let actual = find_most_used_edges(&graph).unwrap();
        assert!(actual.contains(&("hfx".to_string(), "pzl".to_string())));
        assert!(actual.contains(&("bvb".to_string(), "cmg".to_string())));
        assert!(actual.contains(&("jqt".to_string(), "nvd".to_string())));
    }

    #[test]
    fn test_mincut_stoer_wagner() {
        let input: super::InputModel = TEST_INPUT.parse().unwrap();
        let graph = input.graph;
        let (min_cut, part) = mincut_stoer_wagner(&graph);
        println!("actual: {} {:?}", min_cut, part);
        assert_eq!(min_cut, 3);
        assert_eq!(part.len(), 6);
    }
}
