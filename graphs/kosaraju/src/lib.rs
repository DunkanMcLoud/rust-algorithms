use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    usize,
};

#[derive(Debug, Default, Clone)]
pub struct DirectedGraph {
    nodes: Vec<RefCell<Node>>,
}


impl DirectedGraph {
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.ensure_allocation_for(from);
        self.ensure_allocation_for(to);
        // assign node index

        self.nodes[from - 1].borrow_mut().meta.node_id = from;
        // connect
        self.nodes[from - 1].borrow_mut().connect_to(to)
    }

    fn ensure_allocation_for(&mut self, cnt: usize) {
        let last_existing_node_id = if self.nodes.is_empty() { 0 } else { self.nodes.len() };
        if last_existing_node_id < cnt {
            for newly_created_node_id in (last_existing_node_id + 1)..=cnt {
                let node = Node::new();
                node.borrow_mut().meta.node_id = newly_created_node_id;
                self.nodes.push(node)
            }
        }
    }

    pub fn reversed(&self) -> DirectedGraph {
        let mut reversed_graph = DirectedGraph::default();

        for node in self.nodes.iter() {
            for adj_node_id in node.borrow().adjacent_node_ids.iter() {
                reversed_graph.add_edge(*adj_node_id, node.borrow().meta.node_id)
            }
        }
        reversed_graph
    }

    fn set_unvisited(&self) {
        for node in self.nodes.iter() {
            node.borrow_mut().mark_unexplored();
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Node {
    // ordering starts from 1
    adjacent_node_ids: Vec<usize>,
    meta: TraverseMeta,
}

impl Node {
    fn connect_to(&mut self, to: usize) {
        self.adjacent_node_ids.push(to);
    }

    fn is_explored(&self) -> bool {
        self.meta.is_explored
    }

    fn mark_as_explored(&mut self) {
        self.meta.mark_as_visited()
    }

    fn mark_unexplored(&mut self) {
        self.meta.is_explored = false
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

#[derive(Clone, Default, Debug)]
struct TraverseMeta {
    node_id: usize,
    topo_order: usize,
    is_explored: bool,
}

impl TraverseMeta {
    fn mark_as_visited(&mut self) {
        self.is_explored = true
    }
}

fn topo_sort(graph: Rc<DirectedGraph>) {
    let mut cur_label = graph.nodes.len();
    for (index, node) in graph.nodes.iter().enumerate() {
        if !node.borrow().is_explored() {
            dfs_topo(Rc::clone(&graph), index, &mut cur_label)
        }
    }
}

fn dfs_topo(graph: Rc<DirectedGraph>, node_id: usize, topo_label: &mut usize) {
    let selected_node = graph.nodes.get(node_id).unwrap();
    selected_node.borrow_mut().mark_as_explored();
    for node_id in &selected_node.borrow().adjacent_node_ids {
        if !graph.nodes[node_id - 1].borrow().is_explored() {
            dfs_topo(Rc::clone(&graph), node_id - 1, topo_label)
        }
    }
    selected_node.borrow_mut().meta.topo_order = *topo_label;
    *topo_label -= 1;
}

fn dfs_scc(graph: Rc<DirectedGraph>, node_id: usize, component_counter: usize) -> usize {
    let visited_node = graph.nodes.get(node_id - 1).unwrap();
    visited_node.borrow_mut().mark_as_explored();
    let mut inner_counter = 0_usize;
    for adjacent_node_id in &visited_node.borrow().adjacent_node_ids {
        if !graph.nodes[adjacent_node_id - 1].borrow().is_explored() {
            inner_counter += dfs_scc(
                Rc::clone(&graph),
                *adjacent_node_id,
                1);
        }
    }
    component_counter + inner_counter
}

fn kosaraju(source_graph: Rc<DirectedGraph>) -> HashMap<usize, usize> {
    // reverse and topo-sort source graph
    let rev = Rc::new(source_graph.reversed());
    topo_sort(Rc::clone(&rev));
    let mut reversed_copy = rev.as_ref().clone();

    reversed_copy.nodes.sort_by(|lnode, rnode| {
        lnode
            .borrow()
            .meta
            .topo_order
            .partial_cmp(&rnode.borrow().meta.topo_order)
            .unwrap()
    });
    // mark all verticies as unvisited for second df-search
    reversed_copy.set_unvisited();
    let reversed_graph = Rc::new(reversed_copy);

    let mut scc_index = 0_usize;
    let mut scc_map = HashMap::<usize, usize>::new();

    for ( node) in reversed_graph.nodes.iter() {
        let is_explored_on_source_graph = source_graph
            .nodes[node.borrow().meta.node_id - 1]
            .borrow()
            .is_explored();
        if !is_explored_on_source_graph {
            scc_index += 1;
            let number_of_components = dfs_scc(
                Rc::clone(&source_graph),
                node.borrow().meta.node_id,
                1);
            scc_map.insert(scc_index, number_of_components);
        }
    }
    scc_map
}


fn top_5_scc_sizes(graph: Rc<DirectedGraph>) -> Vec<usize> {
    let map = kosaraju(graph);
    let mut vals: Vec<usize> = map.into_values().collect();
    vals.sort();
    vals.reverse();
    vals.resize(5, 0);
    vals
}

#[cfg(test)]
mod tests {
    use std::{fs::File, rc::Rc};

    use crate::{DirectedGraph, top_5_scc_sizes};

    #[test]
    fn test_reversal() {
        let mut graph = DirectedGraph::default();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(4, 5);

        assert_eq!(graph.nodes.len(), 5);
        assert_eq!(graph.nodes[0].borrow().meta.node_id, 1);
        assert_eq!(graph.nodes[1].borrow().meta.node_id, 2);
        assert_eq!(graph.nodes[2].borrow().meta.node_id, 3);
        assert_eq!(graph.nodes[3].borrow().meta.node_id, 4);
        assert_eq!(graph.nodes[4].borrow().meta.node_id, 5);

        assert_eq!(graph.nodes[0].borrow().adjacent_node_ids, vec![2, 3]);
        assert_eq!(graph.nodes[1].borrow().adjacent_node_ids, vec![]);
        assert_eq!(graph.nodes[2].borrow().adjacent_node_ids, vec![]);
        assert_eq!(graph.nodes[3].borrow().adjacent_node_ids, vec![5]);
        assert_eq!(graph.nodes[4].borrow().adjacent_node_ids, vec![]);

        let reversed = graph.reversed();
        dbg!(&reversed);
        assert_eq!(reversed.nodes.len(), 5);
        assert_eq!(reversed.nodes[0].borrow().adjacent_node_ids, vec![]);
        assert_eq!(reversed.nodes[1].borrow().adjacent_node_ids, vec![1]);
        assert_eq!(reversed.nodes[2].borrow().adjacent_node_ids, vec![1]);
        assert_eq!(reversed.nodes[3].borrow().adjacent_node_ids, vec![]);
        assert_eq!(reversed.nodes[4].borrow().adjacent_node_ids, vec![4]);

        // dbg!(graph);
    }

    #[test]
    fn test_one() {
        let m1 = File::open("./test_data/1.txt").unwrap();
        let matrix = utils::read_matrix(m1);
        let graph = Rc::new(DirectedGraph::from(matrix));

        assert_eq!(top_5_scc_sizes(graph), [3, 3, 3, 0, 0]);
    }

    #[test]
    fn test_two() {
        let m1 = File::open("./test_data/2.txt").unwrap();
        let matrix = utils::read_matrix(m1);
        let graph = Rc::new(DirectedGraph::from(matrix));

        assert_eq!(top_5_scc_sizes(graph), [3, 3, 2, 0, 0]);
    }


    #[test]
    fn test_three() {
        let m1 = File::open("./test_data/3.txt").unwrap();
        let matrix = utils::read_matrix(m1);
        let graph = Rc::new(DirectedGraph::from(matrix));

        assert_eq!(top_5_scc_sizes(graph), [3, 3, 1, 1, 0]);
    }

    #[test]
    fn test_four() {
        let m1 = File::open("./test_data/4.txt").unwrap();
        let matrix = utils::read_matrix(m1);
        let graph = Rc::new(DirectedGraph::from(matrix));

        assert_eq!(top_5_scc_sizes(graph), [7, 1, 0, 0, 0]);
    }

    #[test]
    fn test_five() {
        let m1 = File::open("./test_data/5.txt").unwrap();
        let matrix = utils::read_matrix(m1);
        let graph = Rc::new(DirectedGraph::from(matrix));

        assert_eq!(top_5_scc_sizes(graph), [6, 3, 2, 1, 0]);
    }
}
