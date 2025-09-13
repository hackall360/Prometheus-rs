use rand::Rng;

use crate::ast::Expression;
use crate::pipeline::Pipeline;

use crate::random_strings;

/// Create a random string literal using the pipeline's name generator.
pub fn string_literal(pipeline: &mut Pipeline) -> Expression {
    Expression::String(pipeline.name_generator.generate())
}

/// Create a random dictionary key represented as a string expression.
pub fn dictionary_literal() -> Expression {
    random_strings::random_string_expr(None)
}

/// Create a random number literal in the range used by the Lua codebase.
pub fn number_literal() -> Expression {
    let mut rng = rand::thread_rng();
    Expression::Number(rng.gen_range(-8_388_608..=8_388_607) as f64)
}

/// Return a random literal of any of the supported types.
pub fn any_literal(pipeline: &mut Pipeline) -> Expression {
    let mut rng = rand::thread_rng();
    match rng.gen_range(1..=3) {
        1 => string_literal(pipeline),
        2 => number_literal(),
        _ => dictionary_literal(),
    }
}
