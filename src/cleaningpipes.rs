use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

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

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &Self) -> T {
        self.x * other.y - self.y * other.x
    }
}

impl Point<f64> {
    pub fn abs(&self) -> f64 {
        self.square().sqrt()
    }

    pub fn is_perp(&self, other: &Self) -> bool {
        self.dot(other) == 0.0
    }

    pub fn angle(&self, other: &Self) -> f64 {
        let cos_theta = self.dot(other) / self.abs() / other.abs();
        f64::max(-1.0, f64::min(1.0, cos_theta)).acos()
    }

    pub fn orient(a: &Point<f64>, b: &Point<f64>, c: &Point<f64>) -> f64 {
        (b - a).cross(&(c - a))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LineSegment<'a, 'b, T>(pub &'a Point<T>, pub &'b Point<T>);

impl LineSegment<'_, '_, f64> {
    pub fn proper_intersect(&self, other: &LineSegment<f64>) -> Option<Point<f64>> {
        if self.0 == other.0 {
            Some(self.0.clone())
        } else if self.0 == other.1 {
            Some(self.0.clone())
        } else if self.1 == other.0 {
            Some(self.1.clone())
        } else if self.1 == other.1 {
            Some(self.1.clone())
        } else {
            let oa = Point::orient(other.0, other.1, self.0);
            let ob = Point::orient(other.0, other.1, self.1);
            let oc = Point::orient(self.0, self.1, other.0);
            let od = Point::orient(self.0, self.1, other.1);

            if oa * ob < 0.0 && oc * od < 0.0 {
                Some(&(&(self.0 * ob) - &(self.1 * oa)) / (ob - oa))
            } else {
                None
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Color {
    Red,
    Black,
    None,
    Both,
}

impl Color {
    fn next(&self) -> Self {
        match self {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
            Color::None => panic!("No next known"),
            Color::Both => Color::Both,
        }
    }
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines();

    // Technique:
    // Map all intersections, create graph where pipe is vertice, and all edges are the connections between the pipes through the intersections
    // Red/black color graph, if not possible => "impossible", else possible (check if bipartite)

    let line = lines.next().unwrap().unwrap();
    let mut line_iter = line.split(' ');

    let amount_of_wells = line_iter.next().unwrap().parse::<usize>().unwrap();
    let p = line_iter.next().unwrap().parse::<usize>().unwrap();

    let wells: Vec<Point<f64>> = (&mut lines).take(amount_of_wells).map(|l| l.unwrap()).map(|l| {
        let mut pathiter = l.split(' ').map(|x| x.parse::<f64>().unwrap());
        Point { x: pathiter.next().unwrap(), y: pathiter.next().unwrap() }
    }).collect();

    let pipes: Vec<(usize, Point<f64>)> = (&mut lines).take(p).map(|l| l.unwrap()).map(|l| {
        let mut pathiter = l.split(' ').map(|x| x.parse::<isize>().unwrap());
        (pathiter.next().unwrap() as usize, Point { x: pathiter.next().unwrap() as f64, y: pathiter.next().unwrap() as f64 })
    }).collect();

    let mut edges: Vec<Vec<usize>> = vec![vec![]; p];

    for (i1, (w1, p1)) in pipes.iter().enumerate() {
        for (i2, (w2, p2)) in pipes.iter().enumerate() {
            if i2 > i1 && w1 != w2 { // Pipes from the same well only intersect at the well itself.
                let s1 = LineSegment(&wells[w1 - 1], p1);
                let s2 = LineSegment(&wells[w2 - 1], p2);

                if let Some(_) = s1.proper_intersect(&s2) {
                    edges[i1].push(i2);
                    edges[i2].push(i1);
                }
            }
        }
    }

    let mut visited = vec![false; p];
    let mut color: Vec<Color> = vec![Color::None; p];
    let mut found_both_colors = false;

    for i in 0..p {
        if !visited[i] {
            color[i] = Color::Black;
            if dfs(i, &edges, &mut visited, &mut color) {
                found_both_colors = true;
            }
        }
    }

    if found_both_colors {
        writeln!(&mut w, "impossible").unwrap();
    } else {
        writeln!(&mut w, "possible").unwrap();
    }

    Ok(())
}

fn dfs(node: usize, edges: &Vec<Vec<usize>>, visited: &mut Vec<bool>, color: &mut Vec<Color>) -> bool {
    let mut to_visit = vec![node];
    let mut found_both = false;

    while let Some(cur) = to_visit.pop() {
        if !visited[cur] {
            visited[cur] = true;
            for &neighbour in edges[cur].iter() {
                color[neighbour] = if color[cur] == color[neighbour] {
                    found_both = true;
                    Color::Both
                } else {
                    color[cur].next()
                };

                if !visited[neighbour] {
                    to_visit.push(neighbour);
                }
            }
        }
    }

    found_both
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleaningpipes_sampleinputs() {
        for mut file in std::fs::read_dir("input/cleaningpipes")
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
