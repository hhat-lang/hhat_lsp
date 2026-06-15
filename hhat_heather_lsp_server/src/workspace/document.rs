use std::path::PathBuf;
use std::sync::RwLock;
use dashmap::DashMap;
use ropey::Rope;
use tower_lsp::lsp_types::Url;
use crate::parser::ast::ModuleAST;


pub struct ProjectWorkspace {
    modules: DashMap<Url, Document>,
    branches: ProjectBranches,
}

impl ProjectWorkspace {
    pub fn new() -> Self {
        Self {
            modules: DashMap::new(),
            branches: ProjectBranches::new(),
        }
    }
}


pub struct ProjectBranches {
    /// The `.../<project>/` path
    root: RwLock<Option<PathBuf>>,
    /// The `.../<project>/src/` path
    source: RwLock<Option<PathBuf>>,
    /// The `.../<project>/src/hhat_types/` path
    types: RwLock<Option<PathBuf>>,
    /// The `.../<project>/docs/` path
    docs: RwLock<Option<PathBuf>>,
    // TODO: include proofs in some future version
    // The `.../<project>/proofs/` path
    // proofs: RwLock<Option<PathBuf>>,
}

impl ProjectBranches {
    pub fn new() -> Self {
        Self {
            root: RwLock::new(None),
            source: RwLock::new(None),
            types: RwLock::new(None),
            docs: RwLock::new(None),
        }
    }
}


pub struct Document {
    /// Document content
    pub rope: Rope,
    /// version of the document, from [`lsp_types::TextDocumentItem`]
    pub version: i32,
    /// the AST structure for the given document
    pub ast: ModuleAST,
}

