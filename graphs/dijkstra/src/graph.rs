// undirected weighted graph
#[derive(Debug, Clone, Default)]
pub(crate) struct UWGraph {
    nodes: Vec<Node>,
}

impl UWGraph {
    // for init from test file
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
                .map(|tuple| Edge { node_id: tuple[0], weight: tuple[1] })
                .collect::<Vec<Edge>>();

            graph.nodes.push(Node { edges: edges })
        }
        graph
    }
}


#[derive(Debug, Clone)]
pub struct Node {
    edges: Vec<Edge>,
}

#[derive(Debug, Clone)]
pub struct Edge {
    node_id: usize,
    weight: usize,
}

