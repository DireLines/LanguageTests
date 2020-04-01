use image::ImageBuffer;
use num_complex::Complex;
use rayon::prelude::*;
use std::env;
use std::time::Instant;

#[derive(Debug, Copy, Clone)]
struct Pixel {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct MandelbrotInput {
    pix: Pixel,
    c: Complex<f64>,
}

#[derive(Debug)]
struct MandelbrotOutput {
    pix: Pixel,
    iterations: u32,
}

fn main() {
    let resolution = env::args()
        .nth(1)
        .expect("you need to supply a resolution")
        .parse()
        .unwrap_or(5000);
    let max_iters = env::args()
        .nth(2)
        .expect("you need to supply a max # of iterations")
        .parse()
        .unwrap_or(1000);
    let compute_time = Instant::now();
    let arr = mandelbrot(
        resolution,
        Complex::<f64> {
            re: -0.25,
            im: 0.65,
        },
        0.025,
        max_iters,
    );
    println!(
        "computation done in {:.3} seconds",
        compute_time.elapsed().as_secs_f64()
    );
    let image_time = Instant::now();
    let mut imgbuf = ImageBuffer::new(resolution, resolution);
    arr.iter().for_each(|p| {
        let x = p.pix.x;
        let y = p.pix.y;
        let i = {
            if p.iterations == max_iters {
                0
            } else {
                (p.iterations % 255) as u8
            }
        };
        let pixel = imgbuf.get_pixel_mut(y, x);
        *pixel = image::Rgb([i, i, i]);
    });
    imgbuf.save("mandelbrot.png").unwrap();
    println!(
        "image writing done in {:.3} seconds",
        image_time.elapsed().as_secs_f64()
    );
}

fn mandelbrot(
    resolution: u32,
    center: Complex<f64>,
    width: f64,
    max_iters: u32,
) -> Vec<MandelbrotOutput> {
    let points = mandelbrot_points(resolution, center, width);
    points
        .par_iter()
        .map(|p| mandelbrot_iterate(p, max_iters))
        .collect()
}

fn mandelbrot_points(resolution: u32, center: Complex<f64>, width: f64) -> Vec<MandelbrotInput> {
    let half = width / 2.0;
    let step = width / (resolution as f64);
    (0..resolution * resolution)
        .into_par_iter()
        .map(|ind| {
            let x = ind / resolution;
            let y = ind % resolution;
            MandelbrotInput {
                pix: Pixel { x, y },
                c: Complex::<f64> {
                    re: x as f64 * step + center.re - half,
                    im: y as f64 * step + center.im - half,
                },
            }
        })
        .collect()
}

fn mandelbrot_iterate(p: &MandelbrotInput, max_iters: u32) -> MandelbrotOutput {
    let mut z = Complex::<f64> { re: 0.0, im: 0.0 };
    for i in 0..max_iters {
        if z.norm_sqr() >= 4.0 {
            return MandelbrotOutput {
                pix: p.pix,
                iterations: i,
            };
        }
        z = mandelbrot_step(z, p.c)
    }
    return MandelbrotOutput {
        pix: p.pix,
        iterations: max_iters,
    };
}

#[inline(always)]
fn mandelbrot_step(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    z * z + c
}
