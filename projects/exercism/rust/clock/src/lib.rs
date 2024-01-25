use std::{fmt::Display, cmp::Ordering};

#[derive(Debug,PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        println!(
            "Construct a new Clock from {} hours and {} minutes",
            hours,
            minutes
        );
        let tup = match 0.cmp(&minutes) {
            Ordering::Equal => (0,minutes),
            _ => (minutes.div_euclid(60),minutes.rem_euclid(60)),
        };
        let temp_hours = hours + tup.0;
        let h_new = match 0.cmp(&temp_hours) {
            Ordering::Equal => temp_hours,
            _ => temp_hours.rem_euclid(24),
        };
        
        Clock {
            hours: h_new,
            minutes: tup.1
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {

        Clock::new(self.hours, minutes+self.minutes)

    }
}
