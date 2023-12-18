use std::fmt::{Debug, Display};
use std::ops::{Div, Index};

use quicksort::{partition, quick_sort};

fn dselect<T: Ord + Debug + Clone>(arr: &mut [T], order: usize) -> T {
    if arr.len() == 1 {
        return arr[0].clone();
    }
    let mut first_round_medians = arr.rchunks_mut(5)
        .map(|chunk| find_median(chunk))
        .collect::<Vec<T>>();
    let median_of_medians = find_median(first_round_medians.as_mut_slice());
    let median_index = get_element_index(arr, &median_of_medians);

    // partition around pivot
    let pivot_index = partition(arr, |arr| median_index);

    println!("median {:?} , order {} , pivot_index {}, array {:?}", median_of_medians, order, pivot_index, arr);
    match pivot_index {
        pivot_index if pivot_index + 1 == order => median_of_medians,
        pivot_index if pivot_index + 1 > order => dselect(&mut arr[..pivot_index], order),
        _ => dselect(&mut arr[pivot_index + 1..], pivot_index.abs_diff(order) - 1),
    }
}

fn get_element_index<T: Ord + Debug + Clone>(arr: &mut [T], median: &T) -> usize {
    arr.iter().rposition(|el| el.eq(&median)).unwrap()
}

pub fn find_median<T: Ord + Debug + Clone>(arr: &mut [T]) -> T {
    if arr.len() == 1 {
        return arr[0].clone();
    }
    quick_sort(arr);
    if arr.len() <= 2 {
        return arr[0].clone();
    }
    let mid = arr.len().div(2);
    // println!("Array : {:?}, median : {:?}", arr, arr[mid].clone());
    arr[mid].clone()
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn find_median_tst() {
        assert_eq!(find_median(vec![0, 1, 2, 3, 4, 5, 6, 7].as_mut_slice()), 4);
        assert_eq!(find_median(vec![0, 1, 2, 3, 4].as_mut_slice()), 2);
    }


    #[test]
    fn test_1() {
        let mut numbers = utils::read_numbers_from_file(File::open("./test1.txt")
            .expect(utils::FILE_NOT_FOUND_MSG));

        println!("{:?}", numbers);
        assert_eq!(dselect(&mut numbers, 5), 5469);
    }

    #[test]
    fn test_2() {
        let mut numbers = utils::read_numbers_from_file(File::open("./test2.txt")
            .expect(utils::FILE_NOT_FOUND_MSG));


        println!("{:?}", numbers);

        assert_eq!(dselect(&mut numbers, 50), 4715);
    }

    #[test]
    fn lib_select_control() {
        let mut numbers = utils::read_numbers_from_file(File::open("./test1.txt")
            .expect(utils::FILE_NOT_FOUND_MSG));

        numbers.select_nth_unstable(5);

        println!("{:?}", numbers);
        assert_eq!(numbers[4], 5469);
    }

    #[test]
    fn lib_selec_control_2() {
        let mut numbers = utils::read_numbers_from_file(File::open("./test2.txt")
            .expect(utils::FILE_NOT_FOUND_MSG));

        numbers.select_nth_unstable(50);

        println!("{:?}", numbers);
        assert_eq!(numbers[49], 4715);
    }
}
