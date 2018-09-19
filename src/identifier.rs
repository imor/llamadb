use std::fmt;
use std::ops::Deref;

/// An identifier is the name for a database object.
/// Table names, column names, constraint names are identifiers.
///
/// Allowed characters:
///
/// * `a-z`
/// * `A-Z`
/// * `0-9`
/// * `_`
/// * Space (allowed in SQL with quoted identifiers)
///
/// Other rules:
///
/// * Identifiers must have a minimum length of 1.
/// * Identifiers cannot start with a number (0-9) or space.
/// * Identifiers are case insensitive.
///
/// When stored and compared, identifiers must be folded into a canonical,
/// lower-case representation. This process is known as normalization.
#[derive(PartialEq, Eq, Clone)]
pub struct Identifier {
    value: String
}

impl Identifier {
    pub fn new(value: &str) -> Option<Identifier>
    {
        match normalize(value) {
            Some(s) => Some(Identifier {
                value: s
            }),
            None => None
        }
    }
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &str { &self.value }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.value)
    }
}

impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.value)
    }
}

fn normalize(value: &str) -> Option<String> {

    fn is_valid(value: &str) -> bool {
        if let Some(c) = value.chars().nth(0) {
            // Test if the first character is not a digit or space
            match c {
                '0'...'9' | ' ' => false,
                _ => {
                    value.chars().all(|c| {
                        match c {
                            'a'...'z' | 'A'...'Z' | '0'...'9' | '_' | ' ' => true,
                            _ => false
                        }
                    })
                }
            }
        } else {
            false
        }
    }

    if is_valid(value) {
        Some(value.chars().map(|c| {
            c.to_ascii_lowercase()
        }).collect())
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::Identifier;

    #[test]
    fn test_identifier() {
        fn cmp(a: &'static str, b: &'static str) -> bool {
            (&Identifier::new(a).unwrap() as &str) == b
        }

        fn cmp_none(a: &'static str) -> bool {
            Identifier::new(a).is_none()
        }

        assert!(cmp("AbCdEfG", "abcdefg"));
        assert!(cmp("a0123456789", "a0123456789"));
        assert!(cmp("Hello World", "hello world"));
        assert!(cmp_none(""));
        assert!(cmp_none("1a"));
        assert!(cmp_none(" abc "));
        assert!(cmp("_1a", "_1a"));
    }
}
