#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Indexer {
    name: String,
    uri: String,
}

impl Indexer {
    pub fn new(name: String, uri: String) -> Self {
        Indexer { name, uri }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn uri(&self) -> &String {
        &self.uri
    }
}
