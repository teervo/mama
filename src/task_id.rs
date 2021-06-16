use std::cmp::Ordering;
use std::ops::Add;

use crate::error::Error;

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// TaskId is the numerical identifier of a task in todo.txt.
/// Wrt. input from and output to the user, the tasklist is 1-indexed.
/// Internally, it is 0-indexed.
pub struct TaskId(pub usize);

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.add(1).fmt(f)
    }
}

impl std::str::FromStr for TaskId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<usize>() {
            Ok(n) if n < 1 => Err(Error::ZeroId),
            Ok(n) => Ok(TaskId(n - 1)),
            _ => Err(Error::NonnumericId),
        }
    }
}

impl std::cmp::PartialEq<usize> for TaskId {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        self.0.eq(&other)
    }
}

impl std::cmp::PartialOrd<usize> for TaskId {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatted_ids_are_1_indexed() {
        assert_eq!(format!("{}", TaskId(0)), "1");
    }

    #[test]
    fn internal_ids_are_0_indexed() {
        assert_eq!(Ok(TaskId(0)), "1".parse::<TaskId>());
    }
}
