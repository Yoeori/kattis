use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

// Hashing constants
const P: i64 = 31;
const M: i64 = 1_000_000_009;

fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

fn calc(i: usize, b: u8) -> i64 {
    ((mod_pow(P, i as i64, M) % M) * (b as i64)) % M
}

fn hash(word: &str) -> i64 {
    word.bytes()
        .enumerate()
        .fold(0, |acc, (i, b)| ((acc + calc(i, b)) % M))
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let words: HashMap<i64, (String, usize)> = input
        .lines()
        .skip(1)
        .enumerate()
        .map(|(i, w)| (w.unwrap(), i))
        .map(|item| (hash(&item.0), item))
        .collect();
    let mut res: Vec<(&str, usize)> = vec![];

    for (key, (word, i)) in words.iter() {
        if check_word(*key, word, &words) {
            res.push((word, *i));
        }
    }

    res.sort_by(|(_, i), (_, o)| i.cmp(o));
    for word in &res {
        writeln!(&mut w, "{}", word.0)?;
    }

    if res.is_empty() {
        writeln!(&mut w, "NO TYPOS")?;
    }

    Ok(())
}

fn check_word(mut hash: i64, word: &str, list: &HashMap<i64, (String, usize)>) -> bool {
    let bytes = word.as_bytes();

    for i in (0..word.len()).rev() {
        // update hash
        hash -= calc(i, bytes[i]);
        hash %= M;

        if list.contains_key(&(((hash % M) + M) % M))
            && list.get(&(((hash % M) + M) % M)).unwrap().0
               == format!("{}{}", &word[0..i], &word[(i + 1)..])
        {
            return true;
        }

        if i != 0 {
            hash += calc(i - 1, bytes[i]);
            hash %= M;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typo_sampleinputs() {
        for mut file in std::fs::read_dir("input/typo")
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
                &file
            );
        }
    }
}
