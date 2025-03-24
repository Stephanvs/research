#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentType {
    PDF,
    URL,
    Text,
    Image,
    Other(String),
}

#[derive(Debug, Clone)]
pub struct Document {
    pub id: usize,
    pub name: String,
    pub doc_type: DocumentType,
    pub path: String,
    pub children: Vec<usize>,
}

pub struct Model {
    pub documents: Vec<Document>,
    pub root_documents: Vec<usize>, // Documents at the root level
    pub next_id: usize,
    pub selected_document_id: Option<usize>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
            root_documents: Vec::new(),
            next_id: 0,
            selected_document_id: None,
        }
    }

    pub fn add_document(
        &mut self,
        name: String,
        doc_type: DocumentType,
        path: String,
        parent_id: Option<usize>,
    ) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let document = Document {
            id,
            name,
            path,
            doc_type,
            children: Vec::new(),
        };
        self.documents.push(document);

        if let Some(parent_id) = parent_id {
            if let Some(parent_doc) = self.documents.iter_mut().find(|d| d.id == parent_id) {
                parent_doc.children.push(id);
            }
        } else {
            self.root_documents.push(id);
        }

        if parent_id.is_none() {
            self.root_documents.push(id);
        }

        id
    }

    pub fn get_document(&self, id: usize) -> Option<&Document> {
        self.documents.iter().find(|d| d.id == id)
    }

    pub fn set_selected_document(&mut self, id: Option<usize>) {
        self.selected_document_id = id;
    }

    pub fn selected_document(&self) -> Option<&Document> {
        if let Some(id) = self.selected_document_id {
            self.get_document(id)
        } else {
            None
        }
    }
}
