use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}
fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines();

    let first_line: Vec<usize> = lines.next().unwrap().unwrap().split(' ').map(|c| c.parse::<usize>().unwrap()).collect();
    let (n, m, d) = (first_line[0], first_line[1], first_line[2]);

    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    let mut skepticism: HashMap<String, usize> = HashMap::new();

    for _ in 0..n {
        let person = lines.next().unwrap().unwrap();
        let mut person = person.split(' ');
        let name = person.next().unwrap().to_owned();

        connections.insert(name.clone(), vec![]);
        skepticism.insert(name, person.next().unwrap().parse().unwrap());
    }

    for _ in 0..m {
        let connection = lines.next().unwrap().unwrap();
        let mut connection = connection.split(' ');
        let p1 = connection.next().unwrap();
        let p2 = connection.next().unwrap();

        (*connections.get_mut(p1).unwrap()).push(p2.to_owned());
        (*connections.get_mut(p2).unwrap()).push(p1.to_owned());
    }

    let origin = lines.next().unwrap().unwrap();
    let mut visited: HashSet<&str> = HashSet::new();
    let mut spreading: HashSet<&str> = HashSet::new();
    let mut heard: HashSet<&str> = HashSet::new();
    spreading.insert(&origin);

    for _ in 0..d {
        let mut new_spreading: HashSet<&str> = HashSet::new();

        for name in spreading {
            if !visited.contains(name) {
                visited.insert(name);
                for neighbour in connections.get(name).unwrap().iter() {
                    if !visited.contains(&neighbour[..]) {
                        heard.insert(&neighbour[..]);
                        let skepticism = skepticism.get_mut(neighbour).unwrap();
                        if *skepticism > 0 {
                            *skepticism -= 1;
                        }

                        if *skepticism == 0 {
                            new_spreading.insert(&neighbour[..]);
                        }
                    }
                }
            }
        }

        spreading = new_spreading;
    }

    writeln!(&mut w, "{}", heard.len()).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grapevine_sampleinputs() {
        for mut file in std::fs::read_dir("input/grapevine")
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
