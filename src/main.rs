use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod squirrel_ast;
mod squirrel_lexer;
mod squirrel_parser;

#[derive(Debug)]
struct Backend {
    client: Client,
}

impl Backend {
    async fn on_change(&self, params: TextDocumentItem) {
        let mut parser = squirrel_parser::Parser::new(&params.text);

        let result = parser.parse();

        if result.is_err() {
            let error = result.err().unwrap();

            let mut diagnostics = Vec::new();

            diagnostics.push(Diagnostic::new_simple(
                Range::new(
                    Position::new(error.from.line as u32, error.from.linechar as u32),
                    Position::new(error.to.line as u32, error.to.linechar as u32),
                ),
                error.details,
            ));

            self.client
                .publish_diagnostics(params.uri.clone(), diagnostics, Some(params.version))
                .await;
        } else {
            self.client
                .publish_diagnostics(params.uri.clone(), vec![], Some(params.version))
                .await;
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                inlay_hint_provider: Some(OneOf::Left(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                //document_formatting_provider: Some(OneOf::Left(true)),
                //hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()
        })
    }
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file opened!")
            .await;
        self.on_change(TextDocumentItem {
            language_id: params.text_document.language_id,
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
        })
        .await
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        self.on_change(TextDocumentItem {
            language_id: "squirrel".to_string(),
            uri: params.text_document.uri,
            text: std::mem::take(&mut params.content_changes[0].text),
            version: params.text_document.version,
        })
        .await
    }
    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    // async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
    //     let path = params.text_document.uri.path().strip_prefix("/").ok_or(
    //         tower_lsp::jsonrpc::Error::invalid_params("failed to strip prefix"),
    //     )?;

    //     let contents = fs::read_to_string(path).await.map_err(|_| {
    //         tower_lsp::jsonrpc::Error::invalid_params("failed to read file contents")
    //     })?;

    //     let edits = Vec::new();

    //     // edits.push(TextEdit {
    //     //     range: Range {
    //     //         start: Position {
    //     //             line: 0,
    //     //             character: 0,
    //     //         },
    //     //         end: Position {
    //     //             line: 0,
    //     //             character: 1,
    //     //         },
    //     //     },
    //     //     new_text: "Hello world".to_string(),
    //     // });

    //     Ok(Some(edits))
    // }

    //     async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
    //         let _ = params;
    //         Ok(Some(Hover {
    //             contents: HoverContents::Scalar(MarkedString::String("Hello world".to_string())),
    //             range: None,
    //         }))
    //     }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}

