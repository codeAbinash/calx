use crate::errors::Error;
use crate::lexer::token::Token;

pub struct LexOutput {
    pub tokens: Vec<Token>,
    pub errors: Vec<Error>,
}
