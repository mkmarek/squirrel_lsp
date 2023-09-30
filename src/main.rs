use std::collections::HashMap;

use difference::{Changeset, Difference};
use squirrel_printer::{IndentationType, Printer, PrinterSettings};
use tokio::fs;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::request::{GotoDeclarationParams, GotoDeclarationResponse};
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod ast_visitor;
mod scope_table;
mod squirrel_ast;
mod squirrel_lexer;
mod squirrel_parser;
mod squirrel_printer;

#[derive(Debug)]
struct Backend {
    documents: Mutex<HashMap<Url, String>>,
    client: Client,
}

impl Backend {
    async fn on_change(&self, params: TextDocumentItem) {
        {
            let mut documents = self.documents.lock().await;
            documents.insert(params.uri.clone(), params.text.clone());
        }

        let mut parser = squirrel_parser::Parser::new(&params.text);

        let result = parser.parse();

        let mut diagnostics = Vec::new();
        if result.is_err() {
            let error = result.err().unwrap();

            diagnostics.push(Diagnostic::new_simple(
                Range::new(
                    Position::new(error.from.line as u32, error.from.linechar as u32),
                    Position::new(error.to.line as u32, error.to.linechar as u32),
                ),
                error.details,
            ));
        } else {
            let result = result.unwrap();
            let scope_table = scope_table::ScopeTable::new(&result);

            let mut errors = scope_table.validate_variables();

            diagnostics.append(&mut errors);
        }
        self.client
            .publish_diagnostics(params.uri.clone(), diagnostics, Some(params.version))
            .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn goto_declaration(
        &self,
        params: GotoDeclarationParams,
    ) -> Result<Option<GotoDeclarationResponse>> {
        let path = params
            .text_document_position_params
            .text_document
            .uri
            .path()
            .strip_prefix("/")
            .ok_or(tower_lsp::jsonrpc::Error::invalid_params(
                "failed to strip prefix",
            ))?;

        let contents = fs::read_to_string(path).await.map_err(|_| {
            tower_lsp::jsonrpc::Error::invalid_params("failed to read file contents")
        })?;

        let mut parser = squirrel_parser::Parser::new(&contents);

        let result = parser.parse();

        if result.is_err() {
            return Ok(None);
        }

        let result = result.unwrap();
        let scope_table = scope_table::ScopeTable::new(&result);

        let usage = scope_table.find_variable_usage_by_location(
            params.text_document_position_params.position.line as usize,
            params.text_document_position_params.position.character as usize,
        );

        if usage.is_none() {
            return Ok(None);
        }

        let usage = usage.unwrap();

        let declaration = &usage.declaration;

        if declaration.is_none() {
            return Ok(None);
        }

        let declaration = declaration.as_ref().unwrap();

        let declaration = scope_table.get_declaration(declaration);

        if declaration.is_none() {
            return Ok(None);
        }

        let declaration = declaration.unwrap();

        Ok(Some(GotoDeclarationResponse::Scalar(Location::new(
            params.text_document_position_params.text_document.uri,
            Range::new(
                Position::new(
                    declaration.from.line as u32,
                    declaration.from.linechar as u32,
                ),
                Position::new(declaration.to.line as u32, declaration.to.linechar as u32),
            ),
        ))))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let path = params
            .text_document_position_params
            .text_document
            .uri
            .path()
            .strip_prefix("/")
            .ok_or(tower_lsp::jsonrpc::Error::invalid_params(
                "failed to strip prefix",
            ))?;

        let contents = fs::read_to_string(path).await.map_err(|_| {
            tower_lsp::jsonrpc::Error::invalid_params("failed to read file contents")
        })?;

        let mut parser = squirrel_parser::Parser::new(&contents);

        let result = parser.parse();

        if result.is_err() {
            return Ok(None);
        }

        let result = result.unwrap();
        let scope_table = scope_table::ScopeTable::new(&result);

        let usage = scope_table.find_variable_usage_by_location(
            params.text_document_position_params.position.line as usize,
            params.text_document_position_params.position.character as usize,
        );

        if usage.is_none() {
            return Ok(None);
        }

        let usage = usage.unwrap();

        let declaration = &usage.declaration;

        if declaration.is_none() {
            return Ok(None);
        }

        let declaration = declaration.as_ref().unwrap();

        let declaration = scope_table.get_declaration(declaration);

        if declaration.is_none() {
            return Ok(None);
        }

        let declaration = declaration.unwrap();

        Ok(Some(GotoDefinitionResponse::Scalar(Location::new(
            params.text_document_position_params.text_document.uri,
            Range::new(
                Position::new(
                    declaration.from.line as u32,
                    declaration.from.linechar as u32,
                ),
                Position::new(declaration.to.line as u32, declaration.to.linechar as u32),
            ),
        ))))
    }

    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                inlay_hint_provider: Some(OneOf::Left(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    ..WorkspaceServerCapabilities::default()
                }),
                definition_provider: Some(OneOf::Left(true)),
                declaration_provider: Some(DeclarationCapability::Simple(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                //hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()
        })
    }

    async fn workspace_diagnostic(
        &self,
        _: WorkspaceDiagnosticParams,
    ) -> Result<WorkspaceDiagnosticReportResult> {
        self.client
            .log_message(MessageType::INFO, "workspace diagnostic!")
            .await;

        Ok(WorkspaceDiagnosticReportResult::Report(
            WorkspaceDiagnosticReport { items: vec![] },
        ))
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

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let contents = {
            self.documents
                .lock()
                .await
                .get(&params.text_document.uri)
                .ok_or(tower_lsp::jsonrpc::Error::invalid_params(
                    "failed to get document",
                ))?
                .clone()
        };

        let mut parser = squirrel_parser::Parser::new(&contents);
        let ast = parser.parse().map_err(|_| {
            tower_lsp::jsonrpc::Error::invalid_params("failed to parse squirrel file")
        })?;

        let mut printer = Printer::new(
            contents.len(),
            PrinterSettings {
                indentation: if params.options.insert_spaces {
                    IndentationType::Spaces
                } else {
                    IndentationType::Tabs
                },
                indentation_size: params.options.tab_size as usize,
            },
        );

        let mut printed = printer.print(&ast);

        if let Some(insert_final_newline) = params.options.insert_final_newline {
            if insert_final_newline && !printed.ends_with('\n') {
                printed.push('\n');
            }
        }

        if let Some(trim_final_newlines) = params.options.trim_final_newlines {
            if trim_final_newlines {
                while printed.ends_with('\n') {
                    printed.pop();
                }
            }
        }

        let sanitized_contents = contents.replace("\r\n", "\n");
        let changeset = Changeset::new(&sanitized_contents, &printed, "");

        let mut edits = Vec::new();

        let mut current_line_number = 0;
        let mut current_character = 0;

        for change in changeset.diffs {
            self.client
                .log_message(MessageType::ERROR, format!("{:?}", change))
                .await;
            match change {
                Difference::Same(str) => {
                    let line_count = str.matches('\n').count();
                    current_line_number += line_count;
                    current_character = if line_count > 0 {
                        str.split("\n").last().unwrap_or("").len()
                    } else {
                        current_character + str.len()
                    }
                }
                Difference::Rem(str) => {
                    let text_end_line_number = current_line_number + str.matches('\n').count();
                    let text_end_character = if text_end_line_number > current_line_number {
                        str.split("\n").last().unwrap_or("").len()
                    } else {
                        current_character + str.len()
                    };

                    edits.push(TextEdit {
                        range: Range::new(
                            Position::new(current_line_number as u32, current_character as u32),
                            Position::new(text_end_line_number as u32, text_end_character as u32),
                        ),
                        new_text: String::new(),
                    });

                    current_line_number = text_end_line_number;
                    current_character = text_end_character;
                }
                Difference::Add(str) => {
                    let line_number_start = current_line_number;
                    let character_start = current_character;

                    edits.push(TextEdit {
                        range: Range::new(
                            Position::new(line_number_start as u32, character_start as u32),
                            Position::new(line_number_start as u32, character_start as u32),
                        ),
                        new_text: str,
                    });
                }
            }
        }

        self.client
            .log_message(MessageType::ERROR, format!("{:?}", edits))
            .await;
        Ok(Some(edits))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let _ = params;
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("Hello world".to_string())),
            range: None,
        }))
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: Mutex::new(HashMap::new()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
