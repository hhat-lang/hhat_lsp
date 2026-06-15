use std::future::Future;
use std::pin::Pin;
use tower_lsp::{Client, LanguageServer};
use tower_lsp::lsp_types::{CompletionParams, CompletionResponse, DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverParams, InitializeParams, InitializeResult, InitializedParams, RenameParams, WorkspaceEdit};
use crate::workspace::document::ProjectWorkspace;


pub struct HeatherLanguageServer {
    client: Client,
    workspace: ProjectWorkspace,
}

impl HeatherLanguageServer {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            workspace: ProjectWorkspace::new(),
        }
    }
}


impl LanguageServer for HeatherLanguageServer {
    fn initialize<'life0, 'async_trait>(&'life0 self, params: InitializeParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<InitializeResult>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait
    {
        todo!()
    }

    fn initialized<'life0, 'async_trait>(&'life0 self, params: InitializedParams) -> Pin<Box<dyn Future<Output=()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait
    {
        todo!()
    }

    fn shutdown<'life0, 'async_trait>(&'life0 self) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<()>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait
    {
        todo!()
    }

    fn did_open<'life0, 'async_trait>(&'life0 self, params: DidOpenTextDocumentParams) -> Pin<Box<dyn Future<Output=()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait
    {
        todo!()
    }

    fn did_change<'life0, 'async_trait>(&'life0 self, params: DidChangeTextDocumentParams) -> Pin<Box<dyn Future<Output=()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait
    {
        todo!()
    }

    fn did_close<'life0, 'async_trait>(&'life0 self, params: DidCloseTextDocumentParams) -> Pin<Box<dyn Future<Output=()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    fn goto_definition<'life0, 'async_trait>(&'life0 self, params: GotoDefinitionParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<GotoDefinitionResponse>>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait
    {
        todo!()
    }

    fn hover<'life0, 'async_trait>(&'life0 self, params: HoverParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Hover>>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait
    {
        todo!()
    }

    fn completion<'life0, 'async_trait>(&'life0 self, params: CompletionParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<CompletionResponse>>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait
    {
        todo!()
    }

    fn rename<'life0, 'async_trait>(&'life0 self, params: RenameParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<WorkspaceEdit>>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait
    {
        todo!()
    }
}
