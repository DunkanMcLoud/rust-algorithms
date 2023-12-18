use std::fmt::Debug;

pub fn quick_sort<T: Ord + Debug>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    let pivot = partition(slice, select_first_as_pivot);
    let (left, right) = slice.split_at_mut(pivot + 1);
    quick_sort(&mut left[0..pivot]);
    quick_sort(right);
}

pub fn partition<F, T: Ord + Debug>(array: &mut [T], mut select_pivot: F) -> usize where F: FnMut(&mut [T]) -> usize {
    let mut pivot = select_pivot(array);
    array.swap(0, pivot);
    pivot = 0;
    let mut i = pivot + 1;
    let mut j = i;

    while j < array.len() {
        // loop invariant a[j] should be equal or greater than pivot
        if array[j] < array[pivot] {
            array.swap(i, j);
            i += 1;
        }
        j += 1
    }
    //restore the invariant
    //println!("swap {:?} with pivot = {:?}", array[i - 1], array[pivot]);
    array.swap(i - 1, pivot);
    //println!("Partitioned array = {:?}, pivot = {:?}", array, array[i - 1]);
    return i - 1;
}


fn select_first_as_pivot<T: Ord + Debug>(slice: &mut [T]) -> usize {
    0
}


#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::quick_sort;

    #[test]
    fn sort() {
        let mut numbers = utils::read_numbers_from_file(File::open("./test2.txt").expect("Can't find a file"));
        let mut sorted = numbers.clone();
        sorted.sort();
        quick_sort(&mut numbers);
        assert_eq!(sorted, numbers)
    }
}
