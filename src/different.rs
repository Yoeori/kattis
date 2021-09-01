use std::io::{self, BufRead};

// Should not be seen as a 'safe' program since it will just panic whenever unexpected behaviour occurs
pub fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match &line.unwrap()[..] {
            "\n" => {
                // End of input, gracefully shutdown
                break;
            },
            inp => {
                let mut vals = inp.split(' ').map(|v| v.parse::<i64>().unwrap());
                println!("{}",  (vals.next().unwrap() - vals.next().unwrap()).abs());
            } 
        }
        
    }
}