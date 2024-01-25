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
            Ordering::Greater => if minutes%60 == 0 {(minutes/60,minutes%60)} else {(minutes/60-1,60+minutes%60)},
            Ordering::Less => (minutes/60,minutes%60),
            Ordering::Equal => (0,minutes)
        };
        let temp_hours = hours + tup.0;
        let h_new = match 0.cmp(&temp_hours) {
            Ordering::Greater => temp_hours%24 +24,
            Ordering::Less => temp_hours%24,
            Ordering::Equal => temp_hours
        };
        
        Clock {
            hours: if h_new==24 {0} else {h_new},
            minutes: tup.1
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {

        Clock::new(self.hours, minutes+self.minutes)

    }
}
