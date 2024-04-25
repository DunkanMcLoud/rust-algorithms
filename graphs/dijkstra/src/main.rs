mod graph;
pub mod heap;

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;
    use std::fs::File;

    use crate::graph::{DijkstraScore, UWGraph};
    use crate::heap::Heap;

    #[test]
    fn test_shortest_path() {
        let test_data_as_txt = utils::read_as_string(File::open("./test_data/1.txt").unwrap());
        let graph = UWGraph::from_text(test_data_as_txt);

        assert_eq!(graph.shortest_path_to(1), 0);
        assert_eq!(graph.shortest_path_to(2), 1);
        assert_eq!(graph.shortest_path_to(3), 2);
        assert_eq!(graph.shortest_path_to(4), 3);
        assert_eq!(graph.shortest_path_to(5), 4);
        assert_eq!(graph.shortest_path_to(6), 4);
        assert_eq!(graph.shortest_path_to(7), 3);
        assert_eq!(graph.shortest_path_to(8), 2);
    }

    #[test]
    fn test_binary_heap() {
        let mut heap = Heap::<DijkstraScore>::default();
        let mut std_heap = BinaryHeap::<DijkstraScore>::default();

        heap.insert(DijkstraScore::from(1, 3));
        heap.insert(DijkstraScore::from(2, 2));
        heap.insert(DijkstraScore::from(3, 1));

        std_heap.push(DijkstraScore::from(1, 3));
        std_heap.push(DijkstraScore::from(2, 2));
        std_heap.push(DijkstraScore::from(3, 1));

        // max-heap
        let std_score = std_heap.pop().unwrap();
        assert_eq!(std_score, DijkstraScore::from(1, 3));

        // min-heap
        let score = heap.extract_min().unwrap();
        assert_eq!(score, &DijkstraScore::from(3, 1));
    }
}