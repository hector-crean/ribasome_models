use crate::node::Node;
use uuid::Uuid;

pub struct Notebook {
    notebook_id: Uuid,
    nodes: Vec<Node>,
}
