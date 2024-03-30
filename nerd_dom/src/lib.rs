use std::collections::HashMap;

pub type AttrMap = HashMap<String, String>;

pub struct Node {
    pub kind: NodeKind,
    pub children: Vec<Self>,
}

pub enum NodeKind {
    Element(Element),
    Text(String),
}

pub struct Element {
    pub tag: String,
    pub attr: AttrMap,
}
