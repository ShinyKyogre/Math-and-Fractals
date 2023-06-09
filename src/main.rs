use plotters::prelude::*;
use num_complex::*;
use std::*;

// When plotting complex numbers, they are represented lkke this: a + bi
// where a is the real part of the number and bi is the imaginary part.
// When plotting, the x axis represents the real part and y is the imaginary.
// So for example, (3, 5) on the graph is just the number  3 + 5i ; it's not really two seperate coordinates.

fn main() {
    let MAX_ITERATIONS = 100;
    let mut input = num_complex::Complex{re: 0.0, im: 0.0};

    let drawing_area = BitMapBackend::new("C:/Users/sgupt/Desktop/CS 22-23/Shaun Projects/mandelbrot-set/drawings/mandelbrotset.png", (1000, 1000))
    .into_drawing_area();

 
    drawing_area.fill(&WHITE).unwrap();
 
    let (x, y) = drawing_area.get_pixel_range();

    //Plot the points for each point on the graph
    for i in x.start..x.end { 
        for j in y.start..y.end {
            input = num_complex::Complex{re: ((i -500) as f64)/250.0, im: ((j-500) as f64)/250.0}; //This line is pretty finicky - it adjusts how much of the set you can see
            if mandelbrot(input, MAX_ITERATIONS) != 100 {
                drawing_area.draw_pixel((i, j), &HSLColor(mandelbrot(input, MAX_ITERATIONS) as f64 /100.0, 1.0, 0.5)).unwrap();
            } else {
                drawing_area.draw_pixel((i, j), &BLACK).unwrap();
            }
        }
    }

    drawing_area.present().unwrap();
}


fn mandelbrot(c : num_complex::Complex<f64>, I:i32) -> i32 { //This function should be correct
    let mut i = 0;
    let bound = 2;
    let mut z: Complex<f64> = num_complex::Complex{re: 0.0, im: 0.0 };
    while i < I &&  z.norm_sqr() <= (bound*bound) as f64 {
        z = z*z + c;
        i+=1;
    }
    return i; //Return number of iterations to diverge
}

