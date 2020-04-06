mod node;

#[derive(Debug)]
struct Layer {
    nodes: Vec<node::Node>,
    bias: f64,
}

#[derive(Debug)]
struct Network {
    nodes: Vec<node::Node>,
}
