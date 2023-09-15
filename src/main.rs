extern crate ndarray;

use ndarray::prelude::*;
use std::f64::consts::PI;

const G_RED: f64 = 0.01; // m s^-2
const PHI: f64 = 35.*PI/180.; // deg N -> rad
const OMEGA: f64 = 7.29e-5; // rad s^-1
const U_0: f64 = 1.; // m s^-1
const L: f64 = 50.; // km 
 
fn sech2(y: Array1<f64>) -> Array1<f64> {
    y.mapv_into(|y| y.cosh().recip().powi(2))
}

fn thermo_depth(y: Array1<f64>) -> Array1<f64> {
   y.mapv_into(|y| 1. - y.tanh())
}

fn main() {
    let f = 2.*OMEGA*PHI.sin();
    let h_0 = U_0*f*L*1_000./G_RED;
    let y: Array1<f64> = Array::range(-150., 151., 1.);

    let u_bick = U_0*sech2(&y/L);
    let h = h_0*thermo_depth(&y/L);

    println!("\nf = {} rad s^-1\n", f);
    println!("ho = {:.2} m\n", h_0);

    println!("u_bick = {:?}\n", &u_bick);
    println!("h = {:?}", &h);
}
