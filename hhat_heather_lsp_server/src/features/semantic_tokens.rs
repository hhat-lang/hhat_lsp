use tower_lsp::lsp_types::{SemanticTokenType, SemanticTokenModifier};
use crate::workspace::document::Document;

pub const TOKEN_TYPES: &[SemanticTokenType] = &[
    SemanticTokenType::KEYWORD,
    SemanticTokenType::TYPE,
    SemanticTokenType::FUNCTION,
    SemanticTokenType::VARIABLE,
    SemanticTokenType::STRING,
    SemanticTokenType::NUMBER,
    SemanticTokenType::OPERATOR,
    SemanticTokenType::COMMENT,
    SemanticTokenType::PARAMETER,
];

pub const TOKEN_MODIFIERS: &[SemanticTokenModifier] = &[
    SemanticTokenModifier::DEFINITION,
    SemanticTokenModifier::DECLARATION,
];


pub struct TokenCollector<'a> {
    doc: &'a Document,
    /// tokens as tuple vector: (line, column, length, type, modifiers)
    tokens: Vec<(u32, u32, u32, u32, u32)>,
}

impl<'a> TokenCollector<'a> {
    fn new(doc: &'a Document) -> Self {
        Self {
            doc,
            tokens: Vec::new(),
        }
    }
}

