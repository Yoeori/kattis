use std::io::{self, BufRead};

const MOD: u64 = 1000000007;

fn main() {
    let stdin = io::stdin();
    for l in stdin.lock().lines().map(|l| l.unwrap()).skip(1) {
        let mut seq = l.split(' ');
        let res = add(&mut seq);
        println!("{:?}", res % MOD);

    }
}

fn mul<'a>(seq: &mut impl Iterator<Item = &'a str>) -> u64 {
    let mut total = 1;

    while let Some(c) = seq.next() {
        match c {
            "(" => {
                total *= add(seq);
                total %= MOD;
            },
            ")" => {
                break;
            }
            n => {
                total *= n.parse::<u64>().unwrap();
                total %= MOD;
            }
        }

    }

    total
}

fn add<'a>(seq: &mut impl Iterator<Item = &'a str>) -> u64 {
    let mut total = 0;

    while let Some(c) = seq.next() {
        match c {
            "(" => {
                total += mul(seq);
                total %= MOD;
            },
            ")" => {
                break;
            }
            n => {
                total += n.parse::<u64>().unwrap();
                total %= MOD;
            }
        }

    }
    
    total
}