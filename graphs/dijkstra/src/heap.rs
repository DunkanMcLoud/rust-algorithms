use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Default, Clone)]
pub struct Heap<T: Ord + Identity> {
    data: Vec<T>,
    // {id : index}
    locations: HashMap<usize, usize>,
}

impl<T: Identity + Ord + Default + Debug + Clone> Heap<T> {
    pub fn insert(&mut self, item: T) {
        let new_item_id = item.get_id();
        self.data.push(item.clone());
        self.locations.insert(new_item_id, self.data.len() - 1);
        self.bubble_up(self.data.len());
    }

    fn bubble_up(&mut self, checked_pos: usize) {
        if (checked_pos <= 1) {
            return;
        }
        // floor(div by 2)
        let parent_pos = checked_pos >> 1;
        let parent_node = &self.data[parent_pos - 1];
        let child_node = &self.data[checked_pos - 1];
        if parent_node <= child_node {
            return;
        } else {
            self.swap_elements(parent_pos - 1, checked_pos - 1);
            self.bubble_up(parent_pos)
        }
    }

    fn swap_elements(&mut self, a_index: usize, b_index: usize) {
        self.data.swap(a_index, b_index);
        self.locations.insert(self.data[a_index].get_id(), a_index);
        self.locations.insert(self.data[b_index].get_id(), b_index);
    }

    // should remove from heap
    pub fn extract_min(&mut self) -> Option<T> {
        if self.data.len() == 0 {
            None
        } else {
            self.swap_elements(0, self.data.len() - 1);
            let ans = self.pop_last();
            self.bubble_down(1);
            ans
        }
    }

    // pop() would have been better, but let's follow The Book
    pub fn delete_by_id(&mut self, id: usize) -> Option<T> {
        if !self.locations.contains_key(&id) {
            None
        } else {
            let i = self.locations[&id];
            self.swap_elements(i, self.data.len() - 1);
            let ans = self.pop_last();
            self.bubble_down(i + 1);
            ans
        }
    }

    pub fn contains_id(&self, id: &usize) -> bool {
        self.locations.contains_key(id)
    }

    pub fn size(&self) -> usize {
        assert_eq!(self.locations.len(), self.data.len());
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    fn bubble_down(&mut self, checked_pos: usize) {
        if (checked_pos == self.data.len()) {
            return;
        }
        // multiply by 2
        let left_child_pos = checked_pos << 1;
        let right_child_pos = (checked_pos << 1) + 1;

        self.check_child_inv(checked_pos, left_child_pos);
        self.check_child_inv(checked_pos, right_child_pos);
    }

    fn check_child_inv(&mut self, checked_pos: usize, child_pos: usize) {
        if let Some(child) = self.data.get(child_pos - 1) {
            if child >= &self.data[checked_pos - 1] {
                // invariant holds
                return;
            } else {
                self.swap_elements(checked_pos - 1, child_pos - 1);
                self.bubble_down(child_pos);
            }
        } else {
            return;
        }
    }

    fn pop_last(&mut self) -> Option<T> {
        let ans = self.data.pop();
        self.locations.remove(&ans.clone().unwrap().get_id());
        ans
    }
}

// for retrieving elements from the heap by there ids
pub trait Identity {
    fn get_id(&self) -> usize;
}

