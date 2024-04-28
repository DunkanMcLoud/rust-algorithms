mod graph;
pub mod heap;

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;
    use std::fs::File;

    use crate::graph::{DijkstraScore, UWGraph};
    use crate::heap::Heap;

    // tests were used for development and debugging only
    #[test]
    fn test_shortest_path() {
        let test_data_as_txt = utils::read_as_string(File::open("./test_data/1.txt").unwrap());
        let graph = UWGraph::from_text(test_data_as_txt);

        assert_eq!(graph.shortest_path_to(1), Some(0));
        assert_eq!(graph.shortest_path_to(2), Some(1));
        assert_eq!(graph.shortest_path_to(3), Some(2));
        assert_eq!(graph.shortest_path_to(4), Some(3));
        assert_eq!(graph.shortest_path_to(5), Some(4));
        assert_eq!(graph.shortest_path_to(6), Some(4));
        assert_eq!(graph.shortest_path_to(7), Some(3));
        assert_eq!(graph.shortest_path_to(8), Some(2));
    }

    #[test]
    fn test_binary_heap() {
        let mut custom_heap = Heap::<DijkstraScore>::default();
        let mut std_heap = BinaryHeap::<DijkstraScore>::default();

        custom_heap.insert(DijkstraScore::from(1, 3));
        custom_heap.insert(DijkstraScore::from(2, 2));
        custom_heap.insert(DijkstraScore::from(3, 1));

        std_heap.push(DijkstraScore::from(1, 3));
        std_heap.push(DijkstraScore::from(2, 2));
        std_heap.push(DijkstraScore::from(3, 1));

        // max-heap
        let std_score = std_heap.pop().unwrap();
        assert_eq!(std_score, DijkstraScore::from(1, 3));

        // min-heap
        let score = custom_heap.extract_min().unwrap();
        assert_eq!(score, DijkstraScore::from(3, 1));

        custom_heap.insert(DijkstraScore::from(6, 7));
        custom_heap.insert(DijkstraScore::from(5, 6));
        custom_heap.insert(DijkstraScore::from(4, 5));
        /// delete element
        assert_eq!(custom_heap.delete_by_id(1_usize).unwrap(), DijkstraScore::from(1, 3));
        assert_eq!(custom_heap.delete_by_id(2_usize).unwrap(), DijkstraScore::from(2, 2));
        assert_eq!(custom_heap.extract_min().unwrap(), DijkstraScore::from(4, 5));
    }

    #[test]
    fn test_bubble_down() {
        let mut heap = Heap::<DijkstraScore>::default();

        heap.insert(DijkstraScore { node_num: 1, score: None });
        heap.insert(DijkstraScore { node_num: 2, score: None });
        heap.insert(DijkstraScore { node_num: 3, score: Some(2) });
        heap.insert(DijkstraScore { node_num: 4, score: Some(1) });
        heap.insert(DijkstraScore { node_num: 5, score: None });

        assert_eq!(4, heap.extract_min().unwrap().node_num);
        assert_eq!(3, heap.extract_min().unwrap().node_num);
        assert_eq!(heap.size(), 3);
    }

    #[test]
    fn delete_by_id_from_heap() {
        let mut heap = Heap::<DijkstraScore>::default();

        heap.insert(DijkstraScore { node_num: 1, score: None });
        heap.insert(DijkstraScore { node_num: 2, score: None });
        heap.insert(DijkstraScore { node_num: 3, score: Some(2) });
        heap.insert(DijkstraScore { node_num: 4, score: Some(1) });
        heap.insert(DijkstraScore { node_num: 5, score: None });

        assert_eq!(heap.delete_by_id(1).unwrap().node_num, 1);
        assert_eq!(heap.size(), 4);

        assert_eq!(heap.delete_by_id(4).unwrap().node_num, 4);
        assert_eq!(heap.extract_min().unwrap().node_num, 3);
    }

    #[test]
    fn test_score_ordering() {
        assert!(DijkstraScore { node_num: 1, score: None } > DijkstraScore { node_num: 2, score: Some(2) });
    }
}