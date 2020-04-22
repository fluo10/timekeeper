extern crate chrono;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub struct TimeKeeper{
    time: i32,

}
impl Timekeeper{
    pub fn getHour(){}
    pub fn getWeek(){}
    pub fn getMinute(){}
    pub fn Update(){}
    fn SetCurrentTime(){}
    fn CheckAlerm(){}
    fn CheckTimer(){}
}

impl Alert{
    pub fn check(&self) -> bool{}
    pub fn alert(&self){}
}