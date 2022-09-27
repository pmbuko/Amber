use heraclitus_compiler::prelude::*;
use crate::{utils::{metadata::ParserMetadata, TranslateMetadata}};
use crate::translate::module::TranslateModule;
use super::statement::stmt::Statement;

#[derive(Debug, Clone)]
pub struct Block {
    statements: Vec<Statement>,
    is_scope: bool
}

impl Block {
    // Get whether this block is empty
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }

    pub fn set_scopeless(&mut self) {
        self.is_scope = false;
    }

    // Push a parsed statement into the block
    pub fn push_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}

impl SyntaxModule<ParserMetadata> for Block {
    syntax_name!("Block");

    fn new() -> Self {
        Block {
            statements: vec![],
            is_scope: true
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        meta.mem.push_scope();
        while let Some(token) = meta.get_current_token() {
            // Handle the end of line or command
            if ["\n", ";"].contains(&token.word.as_str()) {
                meta.increment_index();
                continue;
            }
            // Handle comments
            if token.word.starts_with('#') {
                meta.increment_index();
                continue
            }
            // Handle block end
            else if token.word == "}" {
                break;
            }
            let mut statemant = Statement::new();
            if let Err(failure) = statemant.parse(meta) {
                return match failure {
                    Failure::Quiet(pos) => error_pos!(meta, pos, "Unexpected token"),
                    Failure::Loud(err) => return Err(Failure::Loud(err))
                }
            }
            self.statements.push(statemant);
        }
        meta.mem.pop_scope();
        Ok(())
    }
}

impl TranslateModule for Block {
    fn translate(&self, meta: &mut TranslateMetadata) -> String {
        if self.is_scope { meta.increase_indent(); }
        let result = if self.is_empty() {
            ":".to_string()
        }
        else {
            self.statements.iter()
                .map(|module| meta.gen_indent() + &module.translate(meta))
                .collect::<Vec<_>>().join(";\n")
        };
        if self.is_scope { meta.decrease_indent(); }
        result
    }
}