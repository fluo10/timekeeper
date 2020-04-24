pub trait Alert{
    fn check(&self) -> bool;
    fn alert(&self);
}
