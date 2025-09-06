use core::fmt;

/// A symbol that may represent a variable or a function.
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol(String);

impl TryFrom<&str> for Symbol {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // TODO: There should be more checks here.
        if value.is_empty() {
            return Err("Symbol cannot be empty".to_string());
        }
        Ok(Symbol(value.to_string()))
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
