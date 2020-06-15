use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
impl Clock {
    pub fn new(hour: i32, minute: i32) -> Clock {
        let (mut hr, mut mt) = (hour, minute);

        while mt < 0 {
            hr -= 1;
            mt += 60;
        }

        hr += mt / 60;
        while hr < 0 {
            hr += 24;
        }

        Clock {
            hours: hr % 24,
            minutes: mt % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Clock {
        Clock::new(self.hours, self.minutes + minutes)
    }
}