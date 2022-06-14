use proconio::{input,marker::Usize1};
use std::collections::VecDeque;

const INF: usize = 1 << 30;

fn main(){
    input!{
        n: usize
    }

    // create adjacency matrix
    let mut graph = vec![vec![]; n];
    for _ in 1..n{
        input!{
            a: Usize1,
            b: Usize1
        }
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut dist = vec![INF; n];

    bfs(&graph, &mut dist, 0 as usize);

    let mut u: usize = 0;
    let max = dist.iter().max().unwrap();

    // get the index of the node farthest from node 0
    for i in 0..n {
        if &dist[i] == max {
            u = i
        }
    }

    let mut dist = vec![INF; n];
    bfs(&graph, &mut dist, u);
    println!("{}", 1 + dist.iter().max().unwrap());
}

fn bfs(graph: &Vec<Vec<usize>>, dist: &mut Vec<usize>, s: usize){
    let mut q: VecDeque<usize> = VecDeque::new();
    dist[s] = 0;
    q.push_back(s);

    while q.len() > 0 {
        let i = q.pop_front().unwrap();
        for j in graph[i].iter() {
            if dist[*j] == INF {
                q.push_back(*j);
                dist[*j] = dist[i] + 1;
            }
        }
    }
}