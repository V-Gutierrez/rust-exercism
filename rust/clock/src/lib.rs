use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            hours,
            minutes,
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        if self.minutes + minutes >= 60 {
            if self.hours + 1 == 24 {
                self.hours = self.minutes % 60;
                self.minutes = self.minutes % 60;
            } else {
                self.hours += 1;
                self.minutes = self.minutes + minutes - 60;
            }
        }

        Self {
            hours: self.hours,
            minutes: self.minutes,
        }
    }
}