use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::io::{self, BufRead};
use std::rc::Rc;

#[derive(Eq, PartialEq, Debug)]
struct Patient {
    name: String,
    infection_level: u8,
    arr_time: u32,
}

impl Ord for Patient {
    fn cmp(&self, other: &Self) -> Ordering {
        other.infection_level.cmp(&self.infection_level).then(self.arr_time.cmp(&other.arr_time))
    }
}

impl PartialOrd for Patient {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut patients_map: HashMap<String, Rc<Patient>> = HashMap::new();
    let mut patients: BTreeSet<Rc<Patient>> = BTreeSet::new();

    let mut cur = 0;

    let stdin = io::stdin();
    for l in stdin.lock().lines().map(|l| l.unwrap()).skip(1) {
        let mut line = l.split(" ");

        match line.next().unwrap() {
            "0" => {
                let name = line.next().unwrap();
              
                patients_map.insert(name.to_string(), Rc::new(Patient {
                    name: name.to_string(),
                    infection_level: line.next().unwrap().parse().unwrap(),
                    arr_time: cur,
                }));

                patients.insert(patients_map.get(name).unwrap().clone());

                cur += 1;
            },
            "1" => {
                let name = line.next().unwrap();
                
                let mut patient = patients_map.get_mut(name).unwrap();
                patients.remove(patient);

                Rc::get_mut(&mut patient).unwrap().infection_level += line.next().unwrap().parse::<u8>().unwrap();
                patients.insert(patient.clone());
            },
            "2" => {
                let patient = patients_map.remove(line.next().unwrap()).unwrap();
                patients.remove(&patient);
            },
            "3" => {
                if patients.is_empty() {
                    println!("The clinic is empty");
                } else {
                    println!("{}", patients.iter().next().unwrap().name);
                }
            },
            _ => panic!()
        }
    }


}