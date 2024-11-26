use logos::Logos;
use uuid::Uuid;


#[derive(Logos, Debug)]
pub enum LexerRule {
    #[token("bucket:")]
    #[token("/")]
    Bucket,
    #[token("user:")]
    #[token("@")]
    User,
    #[token("desc:")]
    #[token("description:")]
    #[token("!")]
    Description,
    #[token("tag:")]
    #[token("#")]
    TagStart,
    #[token("[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}",callback = parse_uuid)]
    UuidValue(Uuid), // Will be converted to a bucket or user id
    // For quoted strings - matches anything in double quotes
    #[regex(r#""#)]
    QuoteStart,
    
    // The actual string content between quotes
    #[token(r#"[^"]*"#, callback = parse_quoted_string)]
    QuotedString(String),
    
    #[token(r#"\""#)]
    QuoteEnd,
    #[token("[a-zA-Z0-9_-]+", callback = parse_word)]
    Generic(String),
}

// Callback functions for UUID
fn is_uuid(lex: &mut logos::Lexer<LexerRule>) -> bool {
    let slice = lex.slice();
    // UUID format: 8-4-4-4-12 hex digits
    if slice.len() != 36 {
        return false;
    }
    // Quick format check before attempting parse
    slice.chars().all(|c| c.is_ascii_hexdigit() || c == '-') 
        && slice.matches('-').count() == 4 
        && Uuid::parse_str(slice).is_ok()
}

fn parse_uuid(lex: &mut logos::Lexer<LexerRule>) -> Uuid {
    Uuid::parse_str(lex.slice()).unwrap_or_default()
}

// Callback functions for words
fn is_word(lex: &mut logos::Lexer<LexerRule>) -> bool {
    let slice = lex.slice();
    !slice.is_empty() 
        && slice.chars().all(|c| {
            c.is_alphanumeric() || c == '-' || c == '_'
        })
}

fn parse_word(lex: &mut logos::Lexer<LexerRule>) -> String {
    lex.slice().to_string()
}

// Callback functions for quoted strings
fn is_quoted_string(lex: &mut logos::Lexer<LexerRule>) -> bool {
    let slice = lex.slice();
    // Must start and end with quotes and have content
    slice.starts_with('"') 
        && slice.ends_with('"') 
        && slice.len() > 2
}

fn parse_quoted_string(lex: &mut logos::Lexer<LexerRule>) -> String {
    let slice = lex.slice();
    // Remove the surrounding quotes
    slice[1..slice.len()-1].to_string()
}

