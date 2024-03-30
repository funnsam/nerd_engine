#[derive(Debug, Clone)]
pub enum Token {
    Eof,
    String(String),
    Tag(Tag),
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub content: String,
    pub kind: TagKind,
    pub attr: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub enum TagKind {
    TagOpen,
    TagEnd,
}

impl Tag {
    pub fn new(kind: TagKind) -> Self {
        Self {
            content: String::new(),
            kind,
            attr: Vec::new(),
        }
    }
}
