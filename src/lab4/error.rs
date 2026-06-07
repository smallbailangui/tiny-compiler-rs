#![allow(dead_code)]

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("IO错误: {0}")]
    Io(String),

    #[error("词法错误: {0}")]
    Lex(String),

    #[error("语法错误: {0}")]
    Parse(String),

    #[error("语义/类型错误: {0}")]
    Semantic(String),

    #[error("代码生成错误: {0}")]
    Codegen(String),
}