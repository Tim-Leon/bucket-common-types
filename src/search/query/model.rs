use crate::search::lexer::token::{BucketIdentifier, SerachTerm, UserIdentifer};


#[derive(Debug)]
pub struct SearchQuery {
    pub terms: Vec<SerachTerm>,
    pub search: String,

}

impl SearchQuery {
    pub fn new() -> Self {
        Self { terms: Vec::new(), search: "".to_owned() }
    }

    pub fn sort_terms(&mut self) {
        self.terms.sort_by_key(|term| match term {
            SerachTerm::Bucket(_) => 0,
            SerachTerm::User(_) => 1,
            SerachTerm::Tag(_) => 2,
            SerachTerm::Description(_) => 3,
            _ => 999,
        });
    }

    pub fn add_term(&mut self, term: SerachTerm) {
        // Check if the term is already in the query only one of theses items are supported per query
        match &term {
            SerachTerm::Bucket(_) => {
                if self.terms.iter().any(|t| matches!(t, SerachTerm::Bucket(_))) {
                    return;
                }
            }
            SerachTerm::User(_) => {
                if self.terms.iter().any(|t| matches!(t, SerachTerm::User(_))) {
                    return;
                }
            }
            _ => {}
        }
        self.terms.push(term);
    }
}