use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol(String);

impl TryFrom<String> for Symbol {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // TODO: There should be more checks here.
        if value.is_empty() {
            return Err("Symbol cannot be empty".to_string());
        }
        Ok(Symbol(value))
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
