use petgraph::graph;
use petgraph::visit;
use proconio::input;

fn main() {
    input! {
        n: u32,
        e: [(u32, u32); n - 1],
    };

    let graph = graph::UnGraph::<u32, ()>::from_edges(e);
    let indices = graph.node_indices();

    let mut bfs = visit::Bfs::new(&graph, start);
        while let Some(nx) = bfs.next(&graph) {
        graph[nx] += 1;
    }
}
