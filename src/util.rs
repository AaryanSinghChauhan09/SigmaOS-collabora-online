//! collabora-online model — web-based office suite.

#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub type_: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub documents: Vec<Document>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { documents: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create(&mut self, title: &str, type_: &str) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.documents.push(Document { id: id.clone(), title: title.into(), type_: type_.into(), url: format!("/document/{}", id) });
        id
    }

    pub fn share(&mut self, id: &str) -> bool {
        self.documents.iter().any(|d| d.id == id)
    }

    pub fn list(&self) -> &[Document] { &self.documents }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn document_create_share() {
        let mut c = Model::new();
        let id = c.create("Report", "word");
        assert!(c.share(&id));
    }
}
