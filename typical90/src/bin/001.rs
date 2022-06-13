use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i32,
        k: i32,
        a: [i32; n],
    };

    let solve = |m: i32| {
        let mut cnt = 0;
        let mut pre = 0;
    
        for i in 0..n {
            if a[i] - pre >= m && l - a[i] >= m {
                cnt += 1;
                pre = a[i];
            }
        }
        if cnt >= k {
            return true;
        } else {
            return false;
        }
    };

    let mut left: i32 = -1;
    let mut right: i32 = l + 1;
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if solve(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", left);
}