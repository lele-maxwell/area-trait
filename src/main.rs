
fn main() {
// PI variable removed

println!("enter the shape you want to calculate the area for");
let mut shape = String::new();
std::io::stdin().read_line(&mut shape).expect("failed to read line");
let shape = shape.trim();
 
if shape == "circle" {
    println!("enter the radius of the circle");
    let mut radius = String::new();
    std::io::stdin().read_line(&mut radius).expect("failed to read line");
    let radius: f64 = radius.trim().parse().expect("please enter a number");

    let circle = Circle{radius};

    println!("the area of the circle is {}", circle.area());
}
else if shape == "rectangle" {

    println!("enter the width of the rectangle");
    let mut width = String::new();
    std::io::stdin().read_line(&mut width).expect("failed to read line");
    let width: f64 = width.trim().parse().expect("please enter a number");

    println!("enter the lenght of the rectangle");
    let mut lenght = String::new();
    std::io::stdin().read_line(&mut lenght).expect("failed to read line");
    let lenght: f64 = lenght.trim().parse().expect("please enter a number");

    let rectangle = Rectangle{width, lenght};
    println!("the area of the rectangle is {}", rectangle.area());
}
else {
    println!("please enter a valid shape");
 

}
}
mod area;
use area::{Area, Circle, Rectangle};
