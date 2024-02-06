use std::usize;

#[derive(Debug, Default)]
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn add_connection(&mut self, from: usize, to: usize) {
        if self.nodes.len() < from {
            self.nodes.resize_with(from, Node::default);
        }
        self.nodes[from - 1].connect_to(to)
    }

    fn reversed(&self) -> Graph {
        let mut rev = Graph::default();

        for (index, node) in self.nodes.iter().enumerate() {
            for adj_node_id in node.adjacent_node_ids {
                rev.add_connection(adj_node_id, index + 1)
            }
        }
        rev
    }
}

#[derive(Debug, Default)]
struct Node {
    is_visited: bool,
    // ordering starts from 1
    adjacent_node_ids: Vec<usize>,
}

impl Node {
    fn connect_to(&mut self, to: usize) {
        self.adjacent_node_ids.push(to);
    }
}

impl From<Vec<Vec<usize>>> for Graph {
    fn from(src: Vec<Vec<usize>>) -> Self {
        let mut graph = Graph::default();
        src.into_iter()
            .for_each(|tuple| graph.add_connection(tuple[0], tuple[1]));
        graph
    }
}

fn top_5_scc_sizes(graph: &Graph) -> [usize; 5] {
    let rev: Graph = graph.reversed();
}

#[cfg(test)]
mod tests {

    use std::fs::File;

    use crate::{top_5_scc_sizes, Graph};

    #[test]
    fn test_one() {
        let m1 = File::open("./test_data/1.txt").unwrap();
        let matrix = utils::read_matrix(m1);

        dbg!(&matrix);
        let graph = Graph::from(matrix);

        assert_eq!(top_5_scc_sizes(&graph), [3, 3, 3, 0, 0]);
        dbg!(graph);
    }
}
