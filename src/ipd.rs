tonic::include_proto!("ipd");
use std::fmt;

impl From<i32> for Action {
    fn from(i: i32) -> Self {
        match i {
            1 => Action::Cooperate,
            2 => Action::Defect,
            _ => Action::Null,
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Action::Null => "NULL",
            Action::Cooperate => "COOPERATE",
            Action::Defect => "DEFECT",
        };
        write!(f, "{}", text)
    }
}
