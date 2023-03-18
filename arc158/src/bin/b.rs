use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i64; n]
    }

    let mut x = x;
    x.sort();
    let mut after_0_idx = 0i64;

    for i in 0..n {
        if x[i] >= 0 {
            after_0_idx = i;
            break;
        }
    }

    let mut costs = vec![0; n];

    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                let mut fst = 0;
                let mut snd = 0;
                let mut trd = 0;

                let mut res_pos_l = 0;
                let mut res_pos_mid_l = after_0_idx - 1;
                let mut res_pos_mid_r = after_0_idx;
                let mut res_pos_r = n - 1;

                let mut choose = |&(mut x), l| {
                    match l {
                        0 => {
                            x = res_pos_l;
                            res_pos_l += 1;
                        }
                        1 => {
                            if res_pos_mid_l < 0 {
                                return;
                            }
                            x = res_pos_mid_l;
                            res_pos_mid_l -= 1;
                        }
                        2 => {
                            if res_pos_mid_r >= n {
                                return;
                            }
                            x = res_pos_mid_r;
                            res_pos_mid_r += 1;
                        }
                        3 => {
                            x = res_pos_r;
                            res_pos_r -= 1;
                        }
                        _ => unreachable!(),
                    }
                };

                choose(&fst, i);
                choose(&snd, j);
                choose(&trd, k);

                let a = x[fst];
                let b = x[snd];
                let c = x[trd];

                costs.push((a + b + c) / (a * b * c))
            }
        }
    }

    let min = costs.iter().min().unwrap();
    let max = costs.iter().max().unwrap();

    println!("{}", min);
    println!("{}", max);
}