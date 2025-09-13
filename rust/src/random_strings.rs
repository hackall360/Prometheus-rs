use rand::Rng;
use rand::seq::SliceRandom;

use crate::ast::Expression;

const CHARSET: &[u8] = b"qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890";

/// Generate a random string. If `words` is provided a random element from the
/// list is returned, otherwise a string of random characters with a random
/// length between 2 and 15 is produced.
pub fn random_string(words: Option<&[&str]>) -> String {
    let mut rng = rand::thread_rng();
    if let Some(words) = words {
        words.choose(&mut rng).unwrap().to_string()
    } else {
        let len = rng.gen_range(2..=15);
        (0..len)
            .map(|_| *CHARSET.choose(&mut rng).unwrap() as char)
            .collect()
    }
}

/// Convenience wrapper returning the string as an [`Expression`].
pub fn random_string_expr(words: Option<&[&str]>) -> Expression {
    Expression::String(random_string(words))
}
