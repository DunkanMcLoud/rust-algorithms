fn main() {
    let a1 = vec![5, 34, 42, 2, 678, 3, 76, 8, 9, 78, 3, 1, 46];
    println!("{:?}", sort(&a1));
}
// [1, 2, 3, 3, 5, 8, 9, 34, 42, 46, 76, 78, 678]
fn sort(v: &[i32]) -> Vec<i32> {
    let middle = v.len() / 2;
    if v.len() <= 1 {
        return v.to_vec();
    }
    let a1 = sort(&v[..middle]);
    let a2 = sort(&v[middle..]);
    let m = merge(&a1, &a2);
    println!("{:?}", m);
    m
}

fn merge(a1: &[i32], a2: &[i32]) -> Vec<i32> {
    let mut v = Vec::new();

    let mut k = 0_usize;
    let mut n = 0_usize;
    // [x,x,x]
    // [1,2,3]   [1,2]
    while k < a1.len() && n < a2.len() {
        if a1[k] > a2[n] {
            v.push(a2[n]);
            n += 1;
        } else {
            v.push(a1[k]);
            k += 1;
        }
    }

    while k < a1.len() {
        v.push(a1[k]);
        k += 1;
    }

    while n < a2.len() {
        v.push(a2[n]);
        n += 1;
    }
    return v;
}
