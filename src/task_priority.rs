#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TaskPriority(pub char);

impl std::fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq)]
pub enum PriorityError {
    InvalidLength,
    NoParentheses,
    InvalidCharacter,
}

impl std::str::FromStr for TaskPriority {
    type Err = PriorityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() != 3 {
            return Err(PriorityError::InvalidLength);
        }
        if !s.starts_with('(') || !s.ends_with(')') {
            return Err(PriorityError::NoParentheses);
        }
        match s.chars().nth(1) {
            Some(c) if c.is_ascii_uppercase() => Ok(TaskPriority(c)),
            _ => Err(PriorityError::InvalidCharacter),
        }
    }
}

impl std::cmp::PartialEq<char> for TaskPriority {
    fn eq(&self, other: &char) -> bool {
        self.0.eq(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert!("".parse::<TaskPriority>().is_err());
    }
    #[test]
    fn valid_priority() {
        assert_eq!("(A)".parse::<TaskPriority>().ok(), Some(TaskPriority('A')));
        assert_eq!("(Z)".parse::<TaskPriority>().ok(), Some(TaskPriority('Z')));
    }
    #[test]
    fn too_long_priority() {
        assert_eq!(
            "(AB)".parse::<TaskPriority>(),
            Err(PriorityError::InvalidLength)
        );
    }
    #[test]
    fn no_parentheses() {
        assert_eq!(
            "(AX".parse::<TaskPriority>(),
            Err(PriorityError::NoParentheses)
        );
        assert_eq!(
            "XA)".parse::<TaskPriority>(),
            Err(PriorityError::NoParentheses)
        );
    }
    #[test]
    fn invalid_character() {
        assert_eq!(
            "(a)".parse::<TaskPriority>(),
            Err(PriorityError::InvalidCharacter)
        );
        assert_eq!(
            "(ß)".parse::<TaskPriority>(),
            Err(PriorityError::InvalidCharacter)
        );
        assert_eq!(
            "(§)".parse::<TaskPriority>(),
            Err(PriorityError::InvalidCharacter)
        );
        assert_eq!(
            "(_)".parse::<TaskPriority>(),
            Err(PriorityError::InvalidCharacter)
        );
        assert_eq!(
            "(Ö)".parse::<TaskPriority>(),
            Err(PriorityError::InvalidCharacter)
        );
    }
}
