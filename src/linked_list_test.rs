use std::collections::HashMap;
use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use std::fmt;

#[derive(Eq, PartialEq)]
struct LinkedEdge {
    default: bool,
    next: Option<Rc<RefCell<LinkedEdge>>>,
    patient: Patient
}

impl Ord for LinkedEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.patient.cmp(&other.patient)
    }
}

impl PartialOrd for LinkedEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for LinkedEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.next.is_some() {
            write!(f, "{:?} -> {:?}", self.patient, self.next.as_ref().unwrap().borrow())
        } else {
            write!(f, "{:?}", self.patient)
        }
    }
}

impl LinkedEdge {
    fn add(&mut self, ins: Rc<RefCell<LinkedEdge>>) {
        if self.next.is_none() {
            self.next = Some(ins);
        } else {
            if &ins > self.next.as_ref().unwrap() {
                let mut ins = Some(ins);
                std::mem::swap(&mut ins, &mut self.next);
                self.next.as_ref().unwrap().borrow_mut().add(ins.unwrap());
            } else {
                self.next.as_ref().unwrap().borrow_mut().add(ins);
            }
        }
    }

    fn remove(&mut self) -> Result<Patient, Rc<RefCell<LinkedEdge>>> {
        std::mem::swap(&mut self.next.as_ref().unwrap().borrow_mut().patient, &mut self.patient);
        let mut next = self.next.as_ref().unwrap().borrow().next.clone();
        std::mem::swap(&mut self.next, &mut next);
        Ok(Rc::try_unwrap(next.unwrap())?.into_inner().patient)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Patient {
    name: String,
    infection_level: u8,
    arr_time: u32
}

impl Ord for Patient {
    fn cmp(&self, other: &Self) -> Ordering {
        self.infection_level.cmp(&other.infection_level).then(other.arr_time.cmp(&self.arr_time))
    }
}

impl PartialOrd for Patient {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {

    let mut patients = HashMap::new();
    patients.insert("Test", Rc::new(RefCell::new(LinkedEdge {
        default: false,
        next: None,
        patient: Patient {
            name: "Test".to_string(),
            infection_level: 5,
            arr_time: 1
        }
    })));

    patients.insert("Test2", Rc::new(RefCell::new(LinkedEdge {
        default: false,
        next: None,
        patient: Patient {
            name: "Test".to_string(),
            infection_level: 5,
            arr_time: 0
        }
    })));

    let mut linked_list = LinkedEdge {
        default: true,
        next: Some(Rc::new(RefCell::new(LinkedEdge {
            default: true,
            next: None,
            patient: Patient {
                name: "Final Node".to_string(),
                infection_level: 0,
                arr_time: u32::MAX
            }
        }))),
        patient: Patient {
            name: "Initial Node".to_string(),
            infection_level: u8::MAX,
            arr_time: 0
        }
    };

    linked_list.add(patients.get("Test").unwrap().clone());
    linked_list.add(patients.get("Test2").unwrap().clone());
    patients.remove("Test").unwrap().borrow_mut().remove().unwrap();

    println!("{:?}", linked_list);

}