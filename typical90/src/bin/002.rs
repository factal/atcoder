use proconio::input;

fn main() {
    input! {
        n: u32
    }

    for bit in 0..(1 << n) {
        let mut s = String::new();
        for i in (0..n).rev() {
            if bit & (1 << i) != 0 {
                s.push(')');
            } else {
                s.push('(');
            }
        }
        // println!("{}", s);

        let mut cnt = 0;
        for p in s.chars() {
            if cnt < 0 {
                break;
            }
            if p == '(' {
                cnt += 1;
            } else {
                cnt -= 1;
            }
        }

        if cnt == 0 {
            println!("{}", s);
        }
    }


}
