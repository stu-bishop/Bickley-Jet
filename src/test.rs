use std::f32::consts::E;
use std::f32::consts::PI;

pub fn run() {

    println!("exp(1) = {}", E.powi(1));
    println!("exp(pi) = {}", E.powf(PI));

    let r: f32 = 10.0; 
    // println!("The area of a circle of radius {:.1} = {:.2}", r, pi*r.powf(2f32));
    println!("The area of a circle of radius {:.1} = {:.2}", r, PI*r.powf(2f32));

    println!("{}", f64::sqrt(1001.3 - 1001.0 - 0.3));
    println!("{}", f64::sqrt(-1.0));

}
