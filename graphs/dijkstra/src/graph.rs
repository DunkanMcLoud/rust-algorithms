use std::cmp::{min, Ordering};

use crate::heap::{Heap, Identity};

// undirected weighted graph
#[derive(Debug, Clone, Default)]
pub(crate) struct UWGraph {
    nodes: Vec<Node>,
}

impl UWGraph {
    /// For init from test file
    pub fn from_text(text: String) -> Self {
        let mut graph = UWGraph::default();
        for (i, line) in text.lines().enumerate() {
            let node_and_edge = line.split("\t").collect::<Vec<&str>>();
            let current_node_index = node_and_edge[0].parse::<usize>().expect("Parsing error");
            assert!(current_node_index == i + 1);
            assert!(graph.nodes.len() < current_node_index);

            let edges = node_and_edge[1..]
                .iter()
                .map(|pair| pair.split(",")
                    .map(|char| char.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>())
                .map(|tuple| Edge { node_num: tuple[0], weight: tuple[1] })
                .collect::<Vec<Edge>>();

            graph.nodes.push(Node { edges: edges })
        }
        graph
    }

    /// Computes shortest path distance from source node (1)
    pub fn shortest_path_to(&self, node_id: usize) -> usize {
        assert!(node_id < self.nodes.len());
        let mut shortest_path_len = 0_usize;
        let mut heap: Heap<DijkstraScore> = Heap::default();
        for (index, node) in self.nodes.iter().enumerate() {
            let score = match index {
                0 => DijkstraScore { node_num: 1, score: Some(0) },
                x => DijkstraScore { node_num: x + 1, score: None }
            };
            heap.insert(score)
        }

        while !heap.is_empty() {
            let round_winner = heap.extract_min().expect("Heap should not be empty");
            if (round_winner.node_num == node_id) {
                return shortest_path_len;
            }
            shortest_path_len = round_winner.score.expect("Should give reachable vertices' scores");
            // update heap to maintain invariant
            for adjacent_edge in &self.nodes[round_winner.node_num].edges {
                let old_score = heap.delete(adjacent_edge.node_num);
                heap.insert(DijkstraScore {
                    node_num: adjacent_edge.node_num,
                    score: Some(
                        min(old_score.score.expect("Expect to be a value"),
                            shortest_path_len + adjacent_edge.weight)),
                })
            }
        }
        // assert path exists
        0
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    edges: Vec<Edge>,
}

#[derive(Debug, Clone)]
pub struct Edge {
    node_num: usize,
    weight: usize,
}

#[derive(Eq, Debug, Copy, Clone, Default)]
pub struct DijkstraScore {
    pub node_num: usize,
    pub score: Option<usize>,
}

impl DijkstraScore {
    pub fn from(node_num: usize, score: usize) -> Self {
        Self { node_num, score: Some(score) }
    }
}

impl Identity for DijkstraScore {
    fn get_id(&self) -> usize {
        self.node_num
    }
}

impl PartialEq for DijkstraScore {
    fn eq(&self, other: &Self) -> bool {
        self.node_num == other.node_num
    }
}

impl Ord for DijkstraScore {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.score, other.score) {
            (None, None) => Ordering::Equal,
            (None, Some(x)) => Ordering::Greater,
            (Some(x), Some(y)) => x.cmp(&y),
            (Some(x), None) => Ordering::Less
        }
    }
}

impl PartialOrd for DijkstraScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}