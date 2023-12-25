use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Reverse;

fn stoer_wagner(graph: &HashMap<String, Vec<String>>) -> (i32, Vec<(String, String)>) {
    let mut min_cut = i32::MAX;
    let mut min_cut_set = HashSet::new();
    let mut vertices = graph.keys().cloned().collect::<HashSet<_>>();

    while vertices.len() > 1 {
        let mut max_adj = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut added = HashSet::new();
        let start = vertices.iter().next().unwrap().clone();

        max_adj.insert(start.clone(), 0);
        heap.push(Reverse((0, start.clone())));
        added.insert(start.clone());

        let mut last = start.clone();

        while let Some(Reverse((_, vertex))) = heap.pop() {
            if max_adj.contains_key(&vertex) && !added.contains(&vertex) {
                last = vertex.clone();
                added.insert(vertex.clone());
                for neighbor in &graph[&vertex] {
                    if !max_adj.contains_key(neighbor) || !added.contains(neighbor) {
                        let weight = 1; // Assuming weight of 1 for each edge
                        let entry = max_adj.entry(neighbor.clone()).or_insert(0);
                        *entry += weight;
                        heap.push(Reverse((*entry, neighbor.clone())));
                    }
                }
            }
        }

        let cut_weight = max_adj[&last];
        if cut_weight < min_cut {
            min_cut = cut_weight;
            min_cut_set = added;
        }

        // Merging vertices
        for neighbor in &graph[&last] {
            if let Some(neigh_vec) = graph.get_mut(neighbor) {
                neigh_vec.retain(|x| x != &last);  // Remove 'last' from its neighbors
                if !max_adj.contains_key(neighbor) {
                    neigh_vec.push(start.clone()); // Add 'start' to its neighbors
                }
            }
        }
        vertices.remove(&last);
    }

    let mut cut_edges = Vec::new();
    for (u, neighbors) in graph {
        if min_cut_set.contains(u) {
            for v in neighbors {
                if !min_cut_set.contains(v) {
                    cut_edges.push((u.clone(), v.clone()));
                }
            }
        }
    }

    (min_cut, cut_edges)
}

fn main() {
    let mut graph = HashMap::new();
    graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
    graph.insert("B".to_string(), vec!["A".to_string(), "C".to_string(), "D".to_string()]);
    graph.insert("C".to_string(), vec!["A".to_string(), "B".to_string(), "D".to_string()]);
    graph.insert("D".to_string(), vec!["B".to_string(), "C".to_string()]);

    let (min_cut, cut_edges) = stoer_wagner(&graph);
    println!("Minimum cut: {}", min_cut);
    println!("Edges in the cut: {:?}", cut_edges);
}