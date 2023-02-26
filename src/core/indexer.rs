#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Indexer {
    name: String,
    uri: String,
}

impl Indexer {
    pub fn new(name: &str, uri: &str) -> Self {
        Indexer {
            name: name.to_owned(),
            uri: uri.to_owned(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }
}
