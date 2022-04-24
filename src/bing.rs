use std::error::Error;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());

    let n: usize = lines.next().unwrap().parse().unwrap();
    let words: Vec<String> = lines.take(n).collect();
    let mut lookup: HashMap<&str, usize> = HashMap::new();

    for word in &words {
        writeln!(&mut w, "{}", lookup.get(&word[..]).unwrap_or(&0)).unwrap();

        for i in 1..=word.len() {
            if let Some(idx) = lookup.get_mut(&word[0..i]) {
                *idx += 1;
            } else {
                lookup.insert(&word[0..i], 1);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bing_sampleinputs() {
        for mut file in std::fs::read_dir("input/bing")
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
