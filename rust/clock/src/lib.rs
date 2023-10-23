use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut minutes: i32 = hours * 60 + minutes;
        let mut hours: i32 = minutes / 60;
        minutes %= 60;

        if minutes < 0 {
            minutes += 60;
            hours -= 1;
        }
        hours = (hours % 24 + 24) % 24;
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes: i32 = self.minutes + minutes;
        Clock::new(self.hours, minutes)
    }
}
