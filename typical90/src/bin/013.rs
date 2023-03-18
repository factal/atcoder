use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize, usize); m]
    }

    let edges = edges
        .into_iter()
        .map(|(from, to, cost)| (from - 1, to - 1, cost))
        .collect::<Vec<_>>();
    
    // djikstra
    let mut dist_from_0 = vec![std::usize::MAX; n];
    dist_from_0[0] = 0;
    let mut heap = std::collections::BinaryHeap::new();
    heap.push(std::cmp::Reverse((0, 0)));

    while let Some(std::cmp::Reverse((dist, node))) = heap.pop() {
        if dist_from_0[node] < dist {
            continue;
        }

        for (from, to, cost) in &edges {
            if *from == node {
                if dist_from_0[*to] > dist + cost {
                    dist_from_0[*to] = dist + cost;
                    heap.push(std::cmp::Reverse((dist + cost, *to)));
                }
            }

            if *to == node {
                if dist_from_0[*from] > dist + cost {
                    dist_from_0[*from] = dist + cost;
                    heap.push(std::cmp::Reverse((dist + cost, *from)));
                }
            }
        }
    }

    // djikstra from n-1
    let mut dist_from_n = vec![std::usize::MAX; n];
    dist_from_n[n - 1] = 0;
    let mut heap = std::collections::BinaryHeap::new();
    heap.push(std::cmp::Reverse((0, n - 1)));

    while let Some(std::cmp::Reverse((dist, node))) = heap.pop() {
        if dist_from_n[node] < dist {
            continue;
        }

        for (from, to, cost) in &edges {
            if *to == node {
                if dist_from_n[*from] > dist + cost {
                    dist_from_n[*from] = dist + cost;
                    heap.push(std::cmp::Reverse((dist + cost, *from)));
                }
            }

            if *from == node {
                if dist_from_n[*to] > dist + cost {
                    dist_from_n[*to] = dist + cost;
                    heap.push(std::cmp::Reverse((dist + cost, *to)));
                }
            }
        }
    }

    for i in 0..n {
        println!("{}", dist_from_n[i] + dist_from_0[i]);
    }
}
