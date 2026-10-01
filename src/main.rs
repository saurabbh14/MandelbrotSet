use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

mod lib;

struct GridParams {
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    xpt: usize,
    ypt: usize,
}

impl GridParams {
    fn grid_spacings(&self) -> (f64, f64) {
        let dx = (self.xmax - self.xmin) / (self.xpt as f64 - 1.0);
        let dy = (self.ymax - self.ymin) / (self.ypt as f64 - 1.0);
        (dx, dy)
    }
}

fn main() {
    let gridparams = GridParams {
        xmin : -2.0,
        xmax : 0.5,
        ymin : -1.25,
        ymax : 1.25,
        xpt : 800,
        ypt : 800
    };
    let (dx, dy) = gridparams.grid_spacings();
    let mut grid_re:Vec<f64> = vec![0.0; gridparams.xpt];
    let mut grid_im:Vec<f64> = vec![0.0; gridparams.ypt];
    for i in 0..gridparams.xpt {
        grid_re[i] = gridparams.xmin + (i as f64) * dx;
    }
    for i in 0..gridparams.ypt {
        grid_im[i] = gridparams.ymin + (i as f64) * dy;
    }
    let c_vec = crate::lib::grid_func(&grid_re, &grid_im);

    // File write
    let path = Path::new("Madelbrot_points.out");
    let file = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut file = BufWriter::new(file);
    for i in 0..grid_re.len() {
        for j in 0..grid_im.len() {
            writeln!(file, "{} {} {}", grid_re[i], grid_im[j], c_vec[i][j]).unwrap();
        }
        writeln!(file, "").unwrap(); // pm3d blank line
    }
}
