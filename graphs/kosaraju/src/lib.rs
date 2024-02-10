use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    rc::Rc,
    usize,
};

#[derive(Debug, Default)]
struct DirectedGraph {
    nodes: Vec<RefCell<Node>>,
}

impl DirectedGraph {
    fn add_edge(&mut self, from: usize, to: usize) {
        if self.nodes.len() < from {
            self.nodes.resize_with(from, Node::new);
        }
        self.nodes[from - 1].borrow_mut().connect_to(to)
    }

    fn reversed(&self) -> DirectedGraph {
        let mut rev = DirectedGraph::default();

        for (index, node) in self.nodes.iter().enumerate() {
            for adj_node_id in node.borrow().adjacent_node_ids.iter() {
                rev.add_edge(*adj_node_id, index + 1)
            }
        }
        rev
    }
}

#[derive(Debug, Default)]
struct Node {
    // ordering starts from 1
    adjacent_node_ids: Vec<usize>,
    meta: TraverseMeta,
}

impl Node {
    fn connect_to(&mut self, to: usize) {
        self.adjacent_node_ids.push(to);
    }

    fn is_visited(&self) -> bool {
        self.meta.is_visited
    }

    fn mark_as_visited(&mut self) {
        self.meta.mark_as_visited()
    }

    fn new() -> RefCell<Node> {
        RefCell::new(Node::default())
    }
}

impl From<Vec<Vec<usize>>> for DirectedGraph {
    fn from(src: Vec<Vec<usize>>) -> Self {
        let mut graph = DirectedGraph::default();
        src.into_iter()
            .for_each(|tuple| graph.add_edge(tuple[0], tuple[1]));
        graph
    }
}

#[derive(Default, Debug)]
struct TraverseMeta {
    topo_order: usize,
    is_visited: bool,
}

impl TraverseMeta {
    fn mark_as_visited(&mut self) {
        self.is_visited = true
    }
}

fn topo_sort(graph: Rc<DirectedGraph>) {
    let mut cur_label = graph.nodes.len();
    for (index, node) in graph.nodes.iter().enumerate() {
        if !node.borrow().is_visited() {
            dfs_topo(Rc::clone(&graph), index, &mut cur_label)
        }
    }
}

fn dfs_topo(graph: Rc<DirectedGraph>, node_id: usize, topo_label: &mut usize) {
    let selected_node = graph.nodes.get(node_id).unwrap();
    selected_node.borrow_mut().mark_as_visited();
    for node_id in &selected_node.borrow().adjacent_node_ids {
        if !graph.nodes[node_id - 1].borrow().is_visited() {
            dfs_topo(Rc::clone(&graph), node_id - 1, topo_label)
        }
    }
    selected_node.borrow_mut().meta.topo_order = *topo_label;
    *topo_label -= 1;
}

fn top_5_scc_sizes(graph: &DirectedGraph) -> [usize; 5] {
    let rev: DirectedGraph = graph.reversed();
    [0, 0, 0, 0, 0]
}

#[cfg(test)]
mod tests {

    use std::{fs::File, rc::Rc};

    use crate::{top_5_scc_sizes, topo_sort, DirectedGraph};

    #[test]
    fn test_one() {
        let m1 = File::open("./test_data/1.txt").unwrap();
        let matrix = utils::read_matrix(m1);

        dbg!(&matrix);
        let graph = Rc::new(DirectedGraph::from(matrix));

        // assert_eq!(top_5_scc_sizes(&graph), [3, 3, 3, 0, 0]);
        topo_sort(Rc::clone(&graph));
        dbg!(&graph);
    }
}
