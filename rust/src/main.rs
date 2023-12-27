mod functions;
use crate::functions::*;

fn main() {
    let x = linspace(-5.0,5.0,1000);
    let y: Vec<f64> = x.into_iter()
        .map(|x| x*28.0)
        .collect();
    for i in y{
        println!("{:?}", i);
    }
}
