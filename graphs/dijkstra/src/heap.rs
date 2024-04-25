use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Heap<T: Ord + Identity> {
    data: Vec<T>,
    locations: HashMap<usize, usize>,
}

impl<T: Identity + Ord + Default> Heap<T> {
    pub fn insert(&mut self, item: T) {
        self.data.push(item);
        //todo save locations
        self.bubble_up(self.data.len())
    }

    fn bubble_up(&mut self, current_pos: usize) {
        if (current_pos == 1) {
            return;
        }
        let parent_pos = current_pos >> 1;
        if (self.data[parent_pos - 1] <= self.data[current_pos - 1]) {
            return;
        } else {
            self.data.swap(parent_pos - 1, current_pos - 1);
            self.bubble_up(parent_pos)
        }
    }

    pub fn extract_min(&self) -> Option<&T> {
        self.data.get(0)
    }

    pub fn delete(&mut self, id: usize) -> T {
        T::default()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

pub trait Identity {
    fn get_id(&self) -> usize;
}

