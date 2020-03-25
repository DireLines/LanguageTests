use derive_more::Add;
use rand::Rng;
use rayon::prelude::*;
use std::ops::Mul;
use std::time::Instant;

#[derive(Debug, Add, Copy, Clone)]
struct Complex {
    re: f64,
    im: f64,
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Complex {
    fn sqr_magnitude(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
    fn magnitude(self) -> f64 {
        self.sqr_magnitude().sqrt()
    }
}

#[derive(Debug, Copy, Clone)]
struct Pixel {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct MandelbrotInput {
    pix: Pixel,
    c: Complex,
}

#[derive(Debug)]
struct MandelbrotOutput {
    pix: Pixel,
    iterations: i32,
}

fn main() {
    let compute_time = Instant::now();
    let arr = mandelbrot(
        500,
        Complex {
            re: -0.25,
            im: 0.65,
        },
        0.025,
        1000,
    );
    println!(
        "computation done in {:.3} seconds",
        compute_time.elapsed().as_secs_f64()
    );
}

fn mandelbrot(
    resolution: i32,
    center: Complex,
    width: f64,
    max_iters: i32,
) -> Vec<MandelbrotOutput> {
    let points = mandelbrot_points(resolution, center, width);
    points
        .par_iter()
        .map(|p| mandelbrot_iterate(p, max_iters))
        .collect()
}

fn mandelbrot_points(resolution: i32, center: Complex, width: f64) -> Vec<MandelbrotInput> {
    let half = width / 2.0;
    let step = width / (resolution as f64);
    (0..resolution * resolution)
        .into_par_iter()
        .map(|ind| {
            let x = ind / resolution;
            let y = ind % resolution;
            MandelbrotInput {
                pix: Pixel { x: x, y: y },
                c: Complex {
                    re: x as f64 * step + center.re - half,
                    im: y as f64 * step + center.im - half,
                },
            }
        })
        .collect()
}

fn mandelbrot_iterate(p: &MandelbrotInput, max_iters: i32) -> MandelbrotOutput {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..max_iters {
        if z.sqr_magnitude() >= 4.0 {
            return MandelbrotOutput {
                pix: p.pix.clone(),
                iterations: i,
            };
        }
        z = mandelbrot_step(z, p.c)
    }
    return MandelbrotOutput {
        pix: p.pix.clone(),
        iterations: max_iters,
    };
}

fn mandelbrot_step(z: Complex, c: Complex) -> Complex {
    z * z + c
}
