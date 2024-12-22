mod handle_assignment;
mod handle_conditionals;
mod handle_for;
mod handle_identifier;
mod handle_while;
mod parse_expression;
mod parse_token;
mod parse_tokens;
mod process_precedence;

use handle_conditionals::*;
use handle_for::*;
use handle_identifier::*;
use handle_while::*;
use parse_expression::*;
use parse_token::*;
pub use parse_tokens::*;
