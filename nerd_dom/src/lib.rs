use std::collections::HashMap;

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub children: Vec<Self>,
}

#[derive(Debug)]
pub enum NodeKind {
    Element(Element),
    Text(String),
}

#[derive(Debug)]
pub struct Element {
    pub tag: String,
    pub attr: AttrMap,
}
