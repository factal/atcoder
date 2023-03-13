use proconio::input;

fn main() {
    input! {
        n: usize,
        students: [(usize, usize); n],
        q: usize,
        queries: [(usize, usize); q]
    }
    
    let mut class_1 = vec![0; n+1];
    let mut class_2 = vec![0; n+1];

    for i in 1..n+1 {
        if i !=0 {
            class_1[i] += class_1[i-1];
            class_2[i] += class_2[i-1];
        }

        if students[i-1].0 == 1 {
            class_1[i] += students[i-1].1;
        } else {
            class_2[i] += students[i-1].1;
        }
    }
    
    for i in 0..q {
        let left = queries[i].0;
        let right = queries[i].1;
        let ans_1 = class_1[right] - class_1[left-1];
        let ans_2 = class_2[right] - class_2[left-1];
        println!("{} {}", ans_1, ans_2);
    }
}
