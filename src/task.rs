use gregorian::Date;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Task {
    pub description: String,
    pub completed: bool,
    pub priority: Option<crate::TaskPriority>,
    pub completion_date: Option<Date>,
    pub creation_date: Option<Date>,
}

impl Task {
    pub fn complete(&mut self) {
        self.completed = true;
        self.completion_date = Some(Date::today());
        // todo.txt must have a creation date whenever it has a completion date
        if self.creation_date.is_none() {
            self.creation_date = Date::new(1970, 1, 1).ok();
        }
    }
    pub fn uncomplete(&mut self) {
        self.completed = false;
        self.completion_date = None;
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.completed {
            write!(f, "x ")?;
        }
        if let Some(pri) = self.priority {
            write!(f, "({}) ", pri)?;
        }
        if let Some(date) = self.completion_date {
            write!(f, "{} ", date)?;
        }
        if let Some(date) = self.creation_date {
            write!(f, "{} ", date)?;
        }

        write!(f, "{}", self.description)
    }
}

#[derive(Debug, PartialEq)]
pub enum TaskParsingError {
    EmptyLine,
}

impl std::str::FromStr for Task {
    type Err = TaskParsingError;

    // Using regular expressions might make this a lot cleaner,
    // but it would also increase the binary size by a couple of MB
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let mut cur = iter.next();

        if cur.is_none() {
            return Err(TaskParsingError::EmptyLine);
        }

        // First token is optional and marks task completion
        let completed = match cur {
            Some("x") => {
                cur = iter.next();
                true
            }
            _ => false,
        };

        // Second token is optional and marks task priority. The priority
        // is a single uppercase ASCII character enclosed in parentheses
        let priority = match cur.unwrap_or("").parse::<crate::TaskPriority>() {
            Ok(priority) => {
                cur = iter.next();
                Some(priority)
            }
            _ => None,
        };

        // Next tokens are 0, 1 or 2 dates (YYYY-MM-DD).
        // If only one date is available, it is the creation date of the task.
        // In case of two dates, they are the completion dates and the creation
        // date, in that order.
        let mut creation_date = match cur.unwrap_or("").parse::<Date>() {
            Ok(dt) => {
                cur = iter.next();
                Some(dt)
            }
            _ => None,
        };
        let completion_date = match cur.unwrap_or("").parse::<Date>() {
            Ok(dt) => {
                cur = iter.next();
                let tmp = creation_date;
                creation_date = Some(dt);
                tmp
            }
            _ => None,
        };

        // remainder is the description:
        let description = std::iter::once(cur.unwrap())
            .chain(iter)
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ");

        Ok(Task {
            description,
            completed,
            priority,
            completion_date,
            creation_date,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TaskPriority;

    #[test]
    fn parse_empty_string() {
        assert_eq!("".parse::<Task>(), Err(TaskParsingError::EmptyLine));
    }

    #[test]
    fn parse_whitespace_only() {
        assert_eq!("\t".parse::<Task>(), Err(TaskParsingError::EmptyLine));
        assert_eq!(" ".parse::<Task>(), Err(TaskParsingError::EmptyLine));
    }

    #[test]
    fn description_only() {
        assert_eq!("one two".parse::<Task>().unwrap().description, "one two");
    }

    #[test]
    fn task_completion() {
        assert!("x description".parse::<Task>().unwrap().completed);
        assert!(!"description".parse::<Task>().unwrap().completed);
    }

    #[test]
    fn valid_priority_is_parsed() {
        assert_eq!(
            "(A) description".parse::<Task>().unwrap().priority,
            Some(TaskPriority('A'))
        );
        assert_eq!(
            "x (A) description".parse::<Task>().unwrap().priority,
            Some(TaskPriority('A'))
        );
        assert_eq!(
            "x (A) description".parse::<Task>().unwrap().description,
            "description"
        );
    }

    #[test]
    fn invalid_priority_not_parsed() {
        assert_eq!("(A description".parse::<Task>().unwrap().priority, None);
    }

    #[test]
    fn parse_creation_date() {
        assert_eq!(
            "2019-11-01 abc".parse::<Task>().unwrap().creation_date,
            Date::new(2019, 11, 01).ok()
        );
    }

    #[test]
    fn parse_completion_date() {
        let task = "2019-11-02 2019-11-01 abc".parse::<Task>().unwrap();
        assert_eq!(task.creation_date, Date::new(2019, 11, 01).ok());
        assert_eq!(task.completion_date, Date::new(2019, 11, 02).ok());
    }

    #[test]
    fn initial_x_not_parsed_as_description() {
        assert_eq!(
            "x description".parse::<Task>().unwrap().description,
            "description"
        );
    }
    #[test]
    fn test_formatting() {
        assert_eq!(
            format!("{}", "description only".parse::<Task>().unwrap()),
            "description only"
        );
        assert_eq!(
            format!(
                "{}",
                "x (C) 2019-11-02 2019-11-01 abc".parse::<Task>().unwrap()
            ),
            "x (C) 2019-11-02 2019-11-01 abc"
        );
    }

    #[test]
    fn completed_task_has_completion_date() {
        let mut task = "description".parse::<Task>().unwrap();
        task.complete();
        assert_eq!(task.completion_date, Some(Date::today()));
    }

    #[test]
    fn uncompleted_task_has_no_completion_date() {
        let mut task = "x 2019-11-01 2019-11-01 description"
            .parse::<Task>()
            .unwrap();
        task.uncomplete();
        assert_eq!(task.completion_date, None);
    }

    #[test]
    fn completed_task_must_also_have_creation_date() {
        let mut task = "description".parse::<Task>().unwrap();
        task.complete();
        assert_eq!(task.creation_date, Date::new(1970, 1, 1).ok());
    }
}
