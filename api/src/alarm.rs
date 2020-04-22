use trait::Alert;

pub struct Alarm{
    name: String,
    hour: i32
}
impl Alarm {
    pub fn check(){}
    pub fn Get(){}
    pub fn Set(){}
}
impl Alert for Alarm {
    fn check() -> bool{
        return true;
    }
    fn alert(){}
}