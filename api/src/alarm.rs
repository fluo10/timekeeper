use crate::alert::Alert;

pub struct Alarm{
    name: String,
    hour: i32
}
impl Alarm {
    pub fn Get(){}
    pub fn Set(){}
}
impl Alert for Alarm {
    fn check(&self) -> bool{
        return true;
    }
    fn alert(&self){}
}