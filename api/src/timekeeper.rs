use chrono::{TimeZone, Weekday,ParseResult};
use chrono::prelude::{DateTime, Utc, Local, Datelike, Timelike};

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
/*
impl application{
  pub fn update(&self){}
  fn alert($self){}
  pub fn starttimer(&self){}
  pub fn stopTimer(&self){}
  pub fn resetTimer(&self){}
}
*/