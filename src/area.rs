pub trait Area {
    fn area(&self) -> f64;
    
}

pub struct Circle{
 pub   radius:f64,

}
 impl Area for Circle {
    fn area(&self) -> f64{
        self.radius * self.radius * std::f64::consts::PI
    }

 }

 pub struct Rectangle {
  pub  width:f64,
  pub  lenght: f64,
 }
impl Area for Rectangle  {
    fn area(&self) -> f64 {
        self.lenght * self.width 
    
  }
}
