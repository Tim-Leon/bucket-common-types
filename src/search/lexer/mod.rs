use logos::Logos;
use rules::LexerRule;
use token::{BucketIdentifier, SerachTerm, UserIdentifer};

pub mod token;
pub mod rules;

pub struct Lexer<'source> {
    rules: logos::Lexer<'source,LexerRule>,
}


impl<'source> Lexer<'source> {
    pub fn new(input: &'source str) -> Self {
        Self {
            rules: LexerRule::lexer(input)
        }
    }

    pub fn tokenize(&mut self) -> Vec<SerachTerm> {
        let mut token_values = Vec::new();

        while let Some(token) = self.rules.next() {
            match token {
                Ok(LexerRule::Bucket) => {
                    if let Some(token_val) = self.rules.next() {
                        match token_val {
                            Ok(LexerRule::UuidValue(id)) => {
                                token_values.push(SerachTerm::Bucket(BucketIdentifier::Uuid(id)));
                            }
                            Ok(LexerRule::Generic(word)) => {
                                token_values.push(SerachTerm::Bucket(BucketIdentifier::Name(word.to_string())));
                            }
                            _ => {}
                        }
                    }
                }
                Ok(LexerRule::User) => {
                    if let Some(token_val) = self.rules.next() {
                        match token_val {
                            Ok(LexerRule::UuidValue(id)) => {
                                token_values.push(SerachTerm::User(UserIdentifer::Uuid(id)));
                            }
                            Ok(LexerRule::Generic(word)) => {
                                token_values.push(SerachTerm::User(UserIdentifer::Name(word.to_string())));
                            }
                            _ => {}
                        }
                    }
                }
                Ok(LexerRule::QuoteStart) => {
                    if let Some(token_val) = self.rules.next() {
                        match token_val {
                            Ok(LexerRule::QuotedString(word)) => {  // Changed from Word to QuotedString
                                token_values.push(SerachTerm::Description(word));  // No need for to_string()
                            }
                            _ => {}
                        }
                    }
                }
                Ok(LexerRule::TagStart) => {
                    if let Some(token_val) = self.rules.next() {
                        if let Ok(LexerRule::Generic(word)) = token_val {  // Changed from Word to Generic
                            token_values.push(SerachTerm::Tag(word));  // No need for to_string()
                        }
                    }
                }
                _ => {}
            }
        }
        token_values
    }
}
