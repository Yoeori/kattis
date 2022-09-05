use std::error::Error;
use std::io::{self, BufRead, Write};
use std::cmp;

/// This file is based on the Handbook of geometry for competitive programmers by Victor Lecomte

use std::ops::{Div, Mul, Sub, Add, Neg};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output=T> + Copy> Add for &Point<T> {
    type Output = Point<T>;

    fn add(self, other: Self) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output=T> + Copy> Sub for &Point<T> {
    type Output = Point<T>;

    fn sub(self, other: Self) -> Point<T> {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Mul<Output=T> + Copy> Mul<T> for &Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: T) -> Point<T> {
        Point {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl<T: Div<Output=T> + Copy> Div<T> for &Point<T> {
    type Output = Point<T>;

    fn div(self, rhs: T) -> Point<T> {
        Point {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl<T: Neg<Output=T> + Copy> Neg for &Point<T> {
    type Output = Point<T>;

    fn neg(self) -> Point<T> {
        Point {
            x: -self.x,
            y: -self.y
        }
    }
}

impl<T: Add<Output=T> + Mul<Output=T> + Sub<Output=T> + Copy> Point<T> {
    pub fn square(&self) -> T {
        return self.x * self.x + self.y * self.y;
    }
}

impl Point<f64> {
    pub fn abs(&self) -> f64 {
        self.square().sqrt()
    }
}

pub struct Circle<'a, T> {
    pub center: &'a Point<T>,
    pub radius: T
}

impl<> Circle<'_, f64> {
    pub fn in_circle(&self, p: &Point<f64>) -> bool {
        (p - self.center).abs() < self.radius
    }
}

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines();
    let testcases = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    for _ in 0..testcases {
        lines.next();
        let line = lines.next().unwrap().unwrap();
        let mut line_iter = line.split(' ');
        let n = line_iter.next().unwrap().parse::<usize>().unwrap();

        let radius = line_iter.next().unwrap().parse::<f64>().unwrap() / 2.0;

        let mosquitoes: Vec<Point<f64>> = (&mut lines).take(n).map(|l| l.unwrap()).map(|l| {
            let mut pathiter = l.split(' ').map(|x| x.parse::<f64>().unwrap());
            Point { x: pathiter.next().unwrap(), y: pathiter.next().unwrap() }
        }).collect();

        let mut max = 1;

        for m1 in mosquitoes.iter() {
            for m2 in mosquitoes.iter() {
                if m1 != m2 {
                    let q = (m1 - m2).abs();
                    let p = Point { x: (m1.x + m2.x) / 2.0, y: (m1.y + m2.y) / 2.0 };

                    let c1 = Circle {
                        center: &Point {
                            x: p.x + (radius.powi(2) - (q / 2.0).powi(2)).sqrt() * (m1.y - m2.y) / q,
                            y: p.y + (radius.powi(2) - (q / 2.0).powi(2)).sqrt() * (m2.x - m1.x) / q
                        },
                        radius: radius + (10.0f64.powi(-5) / 2.0)
                    };

                    let c2 = Circle {
                        center: &Point {
                            x: p.x - (radius.powi(2) - (q / 2.0).powi(2)).sqrt() * (m1.y - m2.y) / q,
                            y: p.y - (radius.powi(2) - (q / 2.0).powi(2)).sqrt() * (m2.x - m1.x) / q
                        },
                        radius: radius + (10.0f64.powi(-5) / 2.0)
                    };

                    max = cmp::max(cmp::max(max, count_in_circle(&c1, &mosquitoes)), count_in_circle(&c2, &mosquitoes));
                }
            }
        }

        writeln!(&mut w, "{}", max).unwrap();
    }

    Ok(())
}

fn count_in_circle(c: &Circle<f64>, mosquitoes: &Vec<Point<f64>>) -> usize {
    let mut total = 0;
    for p in mosquitoes {
        if c.in_circle(&p) {
            total += 1;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mosquitoes_sampleinputs() {
        for mut file in std::fs::read_dir("input/mosquitoes")
            .unwrap()
            .filter(|f| f.is_ok() && f.as_ref().unwrap().path().extension().unwrap() == "in")
            .map(|f| f.unwrap().path())
        {
            let mut output_writer: Vec<u8> = Vec::new();
            solve(
                std::fs::read_to_string(&file).unwrap().as_bytes(),
                &mut output_writer,
            )
            .unwrap();
            file.set_extension("ans");
            assert_eq!(
                std::str::from_utf8(&output_writer).unwrap().trim(),
                std::fs::read_to_string(&file).unwrap().trim(),
                "file: {:?}",
                file
            );
        }
    }
}
