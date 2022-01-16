use std::cell::RefCell;
use std::error::Error;
use std::io::{self, BufRead, Write};
use std::rc::Rc;

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

#[derive(Debug)]
struct Element {
    v: usize,
    next: Option<Rc<RefCell<Element>>>,
    prev: Option<Rc<RefCell<Element>>>,
}

impl Element {
    fn next(&self) -> Option<Rc<RefCell<Element>>> {
        self.next.clone()
    }

    fn prev(&self) -> Option<Rc<RefCell<Element>>> {
        self.prev.clone()
    }
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut input = input.lines();
    let _ = input.next().unwrap();

    let values = input.next().unwrap()?;
    let mut values = values.split(' ').map(|x| x.parse::<usize>().unwrap()).rev();

    // Build simple linked list
    let mut list = Rc::new(RefCell::new(Element {
        v: values.next().unwrap(),
        next: None,
        prev: None,
    }));

    let mut total = 1;

    while let Some(v) = values.next() {
        (*list.borrow_mut()).prev = Some(Rc::new(RefCell::new(Element {
            v,
            next: Some(Rc::clone(&list)),
            prev: None,
        })));
        let new = Rc::clone((*list.borrow()).prev.as_ref().unwrap());
        list = new;
        total += 1;
    }

    let mut cursor = list;

    loop {
        let next = cursor.borrow().next(); // Create here such that cursor isn't borrowed in if let statement

        if let Some(next) = next {
            if (cursor.borrow().v + next.borrow().v) % 2 == 0 {
                total -= 2;

                // Remove cursor and next from list
                let prev = cursor.borrow().prev(); // Create here such that cursor isn't borrowed in if let statement

                if let Some(prev) = prev {
                    let next_next = next.borrow().next();

                    if let Some(next_next) = next_next {
                        (*prev.borrow_mut()).next = Some(next_next.clone());
                        (*next_next.borrow_mut()).prev = Some(prev.clone());

                        cursor = prev;
                    } else {
                        (*prev.borrow_mut()).next = None;
                        cursor = prev;
                    }
                } else {
                    if let Some(next_next) = next.borrow().next() {
                        (*next_next.borrow_mut()).prev = None;
                        cursor = next_next;
                    } else {
                        // List is empty
                        break;
                    }
                }
            } else {
                cursor = next;
            }
        } else {
            // We have no elements to compare with, break.
            break;
        }
    }
    
    writeln!(&mut w, "{}", total).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evenup_sampleinputs() {
        for mut file in std::fs::read_dir("input/evenup")
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
                "for file: {:?}",
                file
            );
        }
    }
}
