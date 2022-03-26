extern crate rayon;

use std::io::Write;
use std::ops::{Add, Mul, Sub};
use std::sync::Arc;

use rayon::prelude::*;

const MAX_ITER: usize = 50;
const VLEN: usize = 8;
const ZEROS: Vecf64 = Vecf64([0.; VLEN]);

macro_rules! for_vec {
    ( in_each [ $( $val:tt ),* ] do $from:ident $op:tt $other:ident ) => {
        $( $from.0[$val] $op $other.0[$val]; )*
    };
    ( $from:ident $op:tt $other:ident ) => {
        for_vec!(in_each [0, 1, 2, 3, 4, 5, 6, 7] do $from $op $other);
    };
}

#[derive(Clone, Copy)]
pub struct Vecf64([f64; VLEN]);

impl Mul for Vecf64 {
    type Output = Vecf64;
    fn mul(mut self, other: Vecf64) -> Vecf64 {
        for_vec!(self *= other);
        self
    }
}

impl Add for Vecf64 {
    type Output = Vecf64;
    fn add(mut self, other: Vecf64) -> Vecf64 {
        for_vec!(self += other);
        self
    }
}

impl Sub for Vecf64 {
    type Output = Vecf64;
    fn sub(mut self, other: Vecf64) -> Vecf64 {
        for_vec!(self -= other);
        self
    }
}

pub fn mbrot8(cr: Vecf64, ci: Vecf64) -> u8 {
    let mut zr = ZEROS;
    let mut zi = ZEROS;
    let mut tr = ZEROS;
    let mut ti = ZEROS;
    for _ in 0..MAX_ITER / 5 {
        for _ in 0..5 {
            zi = (zr + zr) * zi + ci;
            zr = tr - ti + cr;
            tr = zr * zr;
            ti = zi * zi;
        }
        if (tr + ti).0.iter().all(|&t| t > 4.) {
            return 0;
        }
    }
    (tr + ti).0.iter()
        .enumerate()
        .map(|(i, &t)| if t <= 4. { 0x80 >> i } else { 0 })
        .fold(0, |accu, b| accu | b)
}

fn main() -> Result<(), std::io::Error> {
    let size = std::env::args().nth(1).and_then(|n| n.parse().ok()).unwrap_or(200);
    let size = size / VLEN * VLEN;
    let inv = 2. / size as f64;
    let mut xloc = vec![ZEROS; size / VLEN];
    for i in 0..size {
        xloc[i / VLEN].0[i % VLEN] = i as f64 * inv - 1.5;
    }
    let xloc = Arc::new(xloc);

    let rows: Vec<Vec<u8>> = (0..size).into_par_iter().map(|y| {
        let xloc = xloc.clone();
        let ci = Vecf64([y as f64 * inv - 1.; VLEN]);
        (0..size / VLEN).map(|x| mbrot8(xloc[x], ci)).collect()
    }).collect();

    println!("P4\n{} {}", size, size);
    let stdout_unlocked = std::io::stdout();
    let mut stdout = stdout_unlocked.lock();
    for row in rows {
        stdout.write_all(&row)?;
    }

    Ok(())
}