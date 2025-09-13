use std::collections::{HashSet};
use std::hash::Hash;

/// Convert a slice into a [`HashSet`] for fast lookup.
pub fn lookupify<T>(items: &[T]) -> HashSet<T>
where
    T: Eq + Hash + Clone,
{
    items.iter().cloned().collect()
}

/// Convert a [`HashSet`] back into a [`Vec`].
pub fn unlookupify<T>(set: &HashSet<T>) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    set.iter().cloned().collect()
}

/// Escape a string using Lua style escape sequences.
pub fn escape(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '\\' => "\\\\".to_string(),
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            '\t' => "\\t".to_string(),
            '\u{07}' => "\\a".to_string(),
            '\u{08}' => "\\b".to_string(),
            '\u{0B}' => "\\v".to_string(),
            '\"' => "\\\"".to_string(),
            '\'' => "\\'".to_string(),
            c if !c.is_ascii() || (c.is_control() && c != '\n' && c != '\t') => {
                format!("\\{:03}", c as u8)
            }
            c => c.to_string(),
        })
        .collect()
}

/// Split a string into a vector of characters.
pub fn chararray(input: &str) -> Vec<char> {
    input.chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_roundtrip() {
        let data = ["a", "b"]; // slice of &str
        let set = lookupify(&data);
        let vec = unlookupify(&set);
        assert_eq!(set.len(), 2);
        assert!(vec.contains(&"a"));
        assert!(vec.contains(&"b"));
    }

    #[test]
    fn escape_basic() {
        assert_eq!(escape("a\n"), "a\\n");
    }

    #[test]
    fn chararray_basic() {
        assert_eq!(chararray("ab"), vec!['a', 'b']);
    }
}

