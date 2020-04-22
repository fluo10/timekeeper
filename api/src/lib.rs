extern crate chrono;
use chrono::{TimeZone, Weekday,ParseResult};
use chrono::prelude::{DateTime, Utc, Local, Datelike, Timelike};
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
pub struct Timekeeper{
    lasttime: DateTime<Local>,
    currenttime: DateTime<Local>
}
impl Timekeeper{
    pub fn new() -> Timekeeper{
        Timekeeper{
            lasttime: Local::now(),
            currenttime: Local::now()
        }
    }
    pub fn getHour(&self) -> u32{
        return self.currenttime.hour(); 
    }
    pub fn getWeek(&self) -> chrono::Weekday{
        return self.currenttime.weekday();
    }
    pub fn getMinute(&self) -> u32{
        return self.currenttime.minute();
    }
    pub fn getSecond(&self) -> u32{
        return self.currenttime.second();
    }
    pub fn update(){}
    fn setTime(&mut self){
        self.lasttime = self.currenttime;
        self.currenttime = Local::now();
    }
    fn CheckAlerm(){}
    fn CheckTimer(){}
}

pub trait Alert{
    fn check(&self) -> bool;
    fn alert(&self);
}