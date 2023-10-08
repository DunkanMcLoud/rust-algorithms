use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    println!("{:?}", read_test_data());
}

fn sort_and_count_inversions(arr: &[u32]) -> (Vec<u32>, u32) {
    //base case
    if arr.len() <= 1 {
        return (arr.to_vec(), 0);
    }

    let (left, right) = arr.split_at(arr.len() / 2);

    let (l_sorted, left_inv) = sort_and_count_inversions(left);
    let (r_sorted, right_inv) = sort_and_count_inversions(right);

    let (sorted, split_inv) = merge_and_count_inv(&l_sorted, &r_sorted);
    (sorted, left_inv + right_inv + split_inv)
}

fn merge_and_count_inv(a: &[u32], b: &[u32]) -> (Vec<u32>, u32) {
    let mut res = Vec::new();

    let mut split_inv = 0_usize;
    let mut k = 0;
    let mut n = 0;
    while k < a.len() && n < b.len() {
        if a[k] < b[n] {
            res.push(a[k]);
            k += 1;
        } else {
            res.push(b[n]);
            n += 1;

            // the rest of sorted elements in a
            split_inv += a.len() - k;
        }
    }

    while k < a.len() {
        res.push(a[k]);
        k += 1;
    }

    while n < b.len() {
        res.push(b[n]);
        n += 1;
    }

    (res, split_inv as u32)
}

fn read_test_data() -> Vec<u32> {
    let file = File::open("./test.txt").expect("Not found file");
    let mut reader = io::BufReader::new(file);
    let mut s = String::new();
    reader.read_to_string(&mut s).expect("Cannot read");
    s.lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<u32>>()
}

#[test]
fn name() {
    let (arr, count) = sort_and_count_inversions(read_test_data().as_slice());
    println!("{:?}", arr);
    assert_eq!(28, count);
}
