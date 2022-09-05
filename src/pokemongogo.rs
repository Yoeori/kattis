use std::error::Error;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn dist(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());
    let n = lines.next().unwrap().parse::<usize>().unwrap();

    let mut pokemon_names: HashMap<String, usize> = HashMap::new();

    let mut pokemons: Vec<(usize, Point)> = vec![];
    let mut pokemon_map: Vec<Vec<usize>> = vec![];

    let mut pokemon_count: usize = 0;

    for line in lines.take(n) {
        let mut sp = line.split(' ');
        let p = Point {
            x: sp.next().unwrap().parse().unwrap(),
            y: sp.next().unwrap().parse().unwrap()
        };
        let name = sp.next().unwrap();

        if let Some(&i) = pokemon_names.get(name) {
            pokemon_map[i].push(pokemon_count);
            pokemons.push((i, p));
        } else {
            pokemon_names.insert(name.to_string(), pokemon_map.len());
            pokemons.push((pokemon_map.len(), p));
            pokemon_map.push(vec![pokemon_count]);
        }

        pokemon_count += 1;
    }
    
    let start_end = Point { x: 0, y: 0 };
    let mut dp: Vec<Vec<i32>> = vec![vec![std::i32::MAX; pokemon_count]; 1 << pokemon_map.len()];

    for i in 1..dp.len() {
        for (id, (pokemon, point)) in pokemons.iter().enumerate() {
            if (i >> pokemon) & 1 == 1 {
                // Check basecase
                if i & !(1usize << pokemon) == 0 {
                    dp[i][id] = start_end.dist(point);
                } else {
                    dp[i][id] = pokemons.iter().enumerate().filter(|(x, _)| dp[i & !(1usize << pokemon)][*x] != std::i32::MAX).map(|(x, (_, p2))| dp[i & !(1usize << pokemon)][x] + p2.dist(point)).min().unwrap();
                }
            }
            if i == (dp.len() - 1) {
                dp[i][id] += point.dist(&start_end);
            }
        }
    }

    writeln!(&mut w, "{}", dp[dp.len() - 1].iter().min().unwrap()).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pokemongogo_sampleinputs() {
        for mut file in std::fs::read_dir("input/pokemongogo")
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
