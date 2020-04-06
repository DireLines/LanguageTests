#[derive(Debug)]
struct Edge {
    weight: f64,
    dest: Node,
}
#[derive(Debug)]
pub struct Node {
    edges: Vec<Edge>,
}
