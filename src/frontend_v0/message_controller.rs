use crate::frontend_v0::errors::ast_errors::AstBuilderError;
use crate::frontend_v0::errors::goto_table_errors::GotoTableError;
use crate::frontend_v0::errors::lexer_errors::LexerError;
use crate::frontend_v0::errors::parser_errors::ParserError;
use crate::frontend_v0::lexer::common::lexer::Position;
use std::io;

pub enum CompilationStage {
    // ?
    PrimaryFileReading,
}

impl Clone for CompilationStage {
    fn clone(&self) -> CompilationStage {
        match self {
            CompilationStage::PrimaryFileReading => CompilationStage::PrimaryFileReading,
        }
    }
}

impl Copy for CompilationStage {}

pub enum CompilationMessage {
    IOError(io::Error),
    LexingError(LexerError, Position),
    ParsingError(ParserError, Position),
    GotoTableError(GotoTableError),
    AstBuildingError(AstBuilderError, Position),
}

pub struct MessageController {
    current_stage: CompilationStage,
    message_list: Vec<(CompilationStage, CompilationMessage)>,
}

impl MessageController {
    pub fn new() -> MessageController {
        MessageController {
            current_stage: CompilationStage::PrimaryFileReading,
            message_list: Vec::new(),
        }
    }

    pub fn set_stage(&mut self, stage: CompilationStage) {
        self.current_stage = stage;
    }

    pub fn accept_io_error(&mut self, error: io::Error) {
        self.message_list
            .push((self.current_stage, CompilationMessage::IOError(error)));
    }
    pub fn accept_lexer_error(&mut self, error: LexerError, position: Position) {
        self.message_list.push((
            self.current_stage,
            CompilationMessage::LexingError(error, position),
        ));
    }
    pub fn accept_goto_table_error(&mut self, error: GotoTableError) {
        self.message_list.push((
            self.current_stage,
            CompilationMessage::GotoTableError(error),
        ));
    }
    pub fn accept_parser_error(&mut self, error: ParserError, position: Position) {
        self.message_list.push((
            self.current_stage,
            CompilationMessage::ParsingError(error, position),
        ));
    }
    pub fn accept_ast_error(&mut self, error: AstBuilderError, position: Position) {
        self.message_list.push((
            self.current_stage,
            CompilationMessage::AstBuildingError(error, position),
        ));
    }
}
