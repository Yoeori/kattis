use std::collections::{HashSet, HashMap};
use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

#[derive(Debug)]
struct WordSet<'a> {
    correct: usize,
    incorrect: usize,
    translation: &'a str
}

fn has_one_translation(sentence: &Vec<&str>, wordset_map: &HashMap<&str, WordSet>) -> bool {
    for word in sentence {
        let wordset = wordset_map.get(word).unwrap();
        if wordset.correct + wordset.incorrect > 1 {
            return false;
        }
    }
    return true;
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());

    let _: usize = lines.next().unwrap().parse().unwrap();
    let sentence = lines.next().unwrap();
    let sentence: Vec<&str> = sentence.split(' ').collect();
    let sentence_map: HashSet<&str> = sentence.iter().map(|&x| x).collect();

    let m: usize = lines.next().unwrap().parse().unwrap();
    let lines: Vec<String> = lines.take(m).collect();
    let mut wordset_map: HashMap<&str, WordSet> = HashMap::new();

    for line in &lines {
        let (word, rest) = line.split_once(' ').unwrap();
        let (translation, correct) = rest.split_once(' ').unwrap();
        let correct = correct == "correct";

        if sentence_map.contains(word) {
            if let Some(v) = wordset_map.get_mut(&word) {
                if correct {
                    v.correct += 1;
                } else {
                    v.incorrect += 1;
                }
            } else {
                wordset_map.insert(word, WordSet {
                    correct: if correct { 1 } else { 0 },
                    incorrect: if !correct { 1 } else { 0 },
                    translation
                });
            }
        }
    }

    if has_one_translation(&sentence, &wordset_map) {
        // Print sentence / and correct/incorrect
        let mut is_correct = true;
        for word in sentence {
            let wordset = wordset_map.get(word).unwrap();
            write!(&mut w, "{} ", wordset.translation).unwrap();
            is_correct = is_correct && wordset.correct == 1
        }
        
        if is_correct {
            writeln!(&mut w, "\ncorrect").unwrap();
        } else {
            writeln!(&mut w, "\nincorrect").unwrap();
        }
    } else {
        // Print poss
        let mut correct = wordset_map.get(sentence[0]).unwrap().correct;
        let mut total = wordset_map.get(sentence[0]).unwrap().incorrect + correct;

        for word in &sentence[1..] {
            let wordset = wordset_map.get(word).unwrap();
            correct = correct * wordset.correct;
            total = total * (wordset.correct + wordset.incorrect)
        }

        writeln!(&mut w, "{} correct", correct).unwrap();
        writeln!(&mut w, "{} incorrect", total - correct).unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dunglish_sampleinputs() {
        for mut file in std::fs::read_dir("input/dunglish")
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
