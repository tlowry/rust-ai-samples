extern crate dataplotlib;
extern crate ndarray;

use dataplotlib::util::{linspace, zip2};
use dataplotlib::plotbuilder::PlotBuilder2D;
use dataplotlib::plotter::Plotter;
use std::f64::consts::E;

use ndarray::Array;

fn main() {
    // Vector of 500 evenly spaced floats ranging from -10 to 10
    let x = linspace(-10, 10, 500);

    let input = Array::from_vec(x.clone());
    let weights = Array::from_vec(x.clone());
    let bb = input.dot(&weights);

    println!("XX{:?}",bb);

    draw(&x, &tanh);
}

// Sigmoid function
fn sigmoid(input: f64) -> f64 {
    return 1.0 / (1.0 + E.powf(-input));
}

// tanh function just use standard library
fn tanh(input: f64) -> f64 {
    return  input.tanh();
}

// Take a vector of floats and transform function and create a visual 2d graph of the result
fn draw(x: &Vec<f64>, f: &Fn(f64) -> f64) {
    let fx = x.iter().map(|x| f(*x)).collect();
    let results = zip2(&x, &fx);

    // Creates a new plot builder
    let mut pb = PlotBuilder2D::new();

    // Colour the plot line
    pb.add_color_xy(results, [1.0, 1.0, 0.0, 1.0]);

    let mut plt = Plotter::new();
    plt.plot2d(pb);
    plt.join();
}