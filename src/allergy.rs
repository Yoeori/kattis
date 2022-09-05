use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Res {
    last_measure: usize,
    total: usize
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap()).map(|x| x.parse::<usize>().unwrap());
    let k = lines.next().unwrap();
    let allergen_days: Vec<usize> = lines.take(k).collect();

    let mut dp: Vec<Vec<Res>> = vec![vec![Res::default(); k]; 1 << k];
    for i in 1..dp.len() {
        for (idx, allergen_day) in allergen_days.iter().enumerate() {
            if (i >> idx) & 1 == 1 {

                if i & !(1usize << idx) == 0 {
                    dp[i][idx] = Res { last_measure: 0, total: *allergen_day }
                } else {
                    dp[i][idx] = dp[i & !(1usize << idx)].iter().enumerate().filter(|(x, _)| dp[i & !(1usize << idx)][*x] != Res::default()).map(|(_, res)| {
                        if res.last_measure + allergen_day + 1 > res.total {
                            Res {
                                last_measure: res.total,
                                total: res.last_measure + allergen_day + 1
                            }
                        } else {
                            Res {
                                last_measure: res.total,
                                total: res.total + 1,
                            }
                        }
                    }).min_by_key(|x| x.total).unwrap();
                }
            }
        }
    }

    writeln!(&mut w, "{}", dp[dp.len() - 1].iter().min_by_key(|x| x.total).unwrap().total).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allergy_sampleinputs() {
        for mut file in std::fs::read_dir("input/allergy")
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
