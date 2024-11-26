use fst::{Set, SetBuilder};
use logos::{Logos, Lexer as LogosLexer};
use std::collections::HashSet;
use fst::{IntoStreamer, Set};
use fst::automaton::Levenshtein;
pub struct Suggestion {
    set: Set<String>,
}


impl Suggestion 
{
    fn new(collection: Vec<String>) -> Self {
        let mut builder = SetBuilder::memory();
        let mut tokens = HashSet::new();
        

        Self {
            set: builder.into_set()
        }
    }
}

impl Suggestion {
    pub fn get(&self, prefix: &str) -> Vec<String> {
        let prefix = prefix.to_lowercase();
        let lev = Levenshtein::new(&prefix, 1).unwrap();
        let stream = self.set.search(lev).into_stream();
        let keys = stream.into_strs().unwrap();
        keys
    }
}