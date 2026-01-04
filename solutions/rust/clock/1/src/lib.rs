use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        Self {
            minutes: Self::normalize(total_minutes)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minutes: Self::normalize(self.minutes + minutes)
        }
    }

    fn normalize(minutes: i32) -> i32 {
        let day = 24 * 60;
        ((minutes % day) + day) % day
    }
    
    fn hours(&self) -> i32{
        self.minutes / 60
    }

    fn mins(&self) -> i32 {
        self.minutes % 60
    }
}


impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours(), self.mins())
    }
}