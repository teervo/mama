use crate::task_priority::PriorityError;
use crate::TaskId;

#[derive(Debug, PartialEq)]
pub enum Error {
    InsufficientArguments,
    InvalidArgument(String),
    IdNotFound(TaskId),
    ZeroId,
    NonnumericId,
    InvalidPriority,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InsufficientArguments => write!(f, "Not enough arguments provided."),
            Error::InvalidArgument(explanation) => {
                write!(f, "Invalid argument: {}", explanation)
            }
            Error::IdNotFound(id) => write!(f, "ID '{}' does not exist.", id),
            Error::ZeroId => write!(f, "IDs must be non-zero."),
            Error::NonnumericId => write!(f, "IDs must be numeric."),
            Error::InvalidPriority => write!(f, "Task priority must be an uppercase letter (A-Z)."),
        }
    }
}

impl From<PriorityError> for Error {
    fn from(_e: PriorityError) -> Self {
        Self::InvalidPriority
    }
}
