use num::complex::Complex;
use std::f64;


// function to replace z -> z^2 + c
fn mapping_function(z: &Complex<f64>, c: &Complex<f64>) -> Option<Complex<f64>> {
    if z.norm() > 2.0 {
        return None
    }
    let r = *z * *z + c;
    Some(r)
}

fn map_iter(c: &Complex<f64>) -> usize {
    let mut z_clone = Complex::new(0.0, 0.0);
    for i in 0..100 {
        let x = mapping_function(&z_clone, c);
        match x {
            Some(x) => z_clone = x,
            None => return i,
        }
    }
    100
}

pub fn grid_func(grid_re: &Vec<f64>, grid_im: &Vec<f64>) -> Vec<Vec<usize>> {
    let mut c:Complex<f64> = Complex { re: 0.0, im: 0.0 };
    let mut c_vec: Vec<Vec<usize>> = vec![vec![0; grid_im.len()]; grid_re.len()];
    for (i, re) in grid_re.iter().enumerate() {
        c.re = *re;
        for (j, im) in grid_im.iter().enumerate() {
            c.im = *im;
            let x = map_iter(&c);
            c_vec[i][j] = x
        }
    }
    c_vec
}

fn main() {}