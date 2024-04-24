#[derive(Debug, Default, Clone)]
pub struct Heap<T: Ord + Identity> {
    data: Vec<T>,
}


impl<T: Identity + Ord> Heap<T> {
    pub fn insert(&mut self, item: T) {}

    pub fn extract_min(&self) -> Option<T> {
        None
    }

    // pub fn heapify(items: &[T]) -> Self {
    //     let mut heap = Heap::<T>::default();
    //     for x in items {
    //         heap.insert(x)
    //     }
    //     heap
    // }


    pub fn delete(&mut self, index: usize) -> T {}


    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

pub trait Identity {
    fn get_id(&self) -> usize;
}

