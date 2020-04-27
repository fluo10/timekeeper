extern crate chrono;

use chrono::{TimeZone, Weekday,ParseResult};
use chrono::prelude::{DateTime, Utc, Local, Datelike, Timelike};
mod alert;
mod timekeeper;
mod alarm;
mod timer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    fn test_now(){
        
    }
}
