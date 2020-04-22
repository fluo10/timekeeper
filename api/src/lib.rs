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
    lasttime: i32,
    currenttime: DateTime<Local>
}
impl Timekeeper{
    pub fn new() -> Timekeeper{
        Timekeeper{currenttime: Utc::now()}
    }
    pub fn getHour(&self) -> u32{
        self.currenttime.year()      
    }
    pub fn getWeek(&self) -> u32{
        self.currenttime.weekday()
    }
    pub fn getMinute(&self) -> u32{
        self.currenttime.minute()
    }
    pub fn getSecond(&self) -> u32{
        self.currenttime.second()
    }
    pub fn update(){}
    fn setTime(&self){
        self.currenttime = Utc::now()
    }
    fn CheckAlerm(){}
    fn CheckTimer(){}
}

impl Alert{
    pub fn check(&self) -> bool{}
    pub fn alert(&self){}
}