
use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(mut hours: i32,mut minutes: i32) -> Self {

        if minutes < 0 {
            let diff = minutes / 60 -1;
            dbg!(diff);
            hours = hours + diff;
            minutes = 60 + (minutes % 60);
        }

        if hours < 0 {
            hours = 24 + (hours % 24);
        }

        if minutes >= 60  {
            hours += minutes / 60;
            minutes = minutes % 60;
        }

        Self {
            hours: hours % 24,
            minutes:  minutes
        }

    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        
        let mut diff_minutes = self.minutes + minutes;
        let mut hours = self.hours;

        if diff_minutes >= 60 {
            hours += diff_minutes / 60;
            diff_minutes = diff_minutes % 60;
        }

        if diff_minutes < 0 {
            let diff = diff_minutes / 60 -1;
            hours += diff;
            diff_minutes = 60 + (diff_minutes % 60);
        }



        if hours < 0 {

            hours = 24 + (hours % 24);
        }

        if hours >= 24 {
            hours = hours % 24;
        }

        Self {
            minutes: diff_minutes,
            hours: hours
        } 
    
    }
}


impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", format!("{:0>2}",self.hours), format!("{:0>2}",self.minutes))
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}