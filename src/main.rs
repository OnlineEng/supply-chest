use std::fs;
use std::io::ErrorKind;

// Reads chest.txt one item per line
// `?` gives errors back to caller
// Promises caller one of two outcomes: `Ok` with list of items, or `Err` with the reason it failed
fn load_items() -> Result<Vec<String>, std::io::Error> {
    let contents = fs::read_to_string("chest.txt")?; // Reads whole file as one String, reading returns a Result and `?` unpacks it
    let mut items = Vec::new();
    for line in contents.lines() { // `.lines` splits the text at each newline
        items.push(line.to_string());
    }
    Ok(items) // `Ok` needed because the function returns a Result
}

// Borrows list
fn check_slot(items: &Vec<String>, index: usize) -> Option<&String> {
    items.get(index)
}

// Game state that should never happen, so there's nothing to recover to
#[allow(dead_code)]
fn impossible_state() {
    panic!("Chest is both open AND locked: impossible game state!");
}

fn main() {
    let items = match load_items() {
        Ok(items) => items, // File read fine, use it
        Err(error) => match error.kind() { // `.kind` returns an ErrorKind that says which error
            ErrorKind::NotFound => {
                println!("chest.txt not found! Using default items");
                vec![String::from("Rope"), String::from("Torch"), String::from("Potion")]
            }
            _ => { // `_` catches every other kind of error
                println!("Could not read chest.txt: {error}");
                return; // Safe program end without panicking
            }
        }
    };

    println!("The chest contains:");
    for item in &items {
        println!("- {item}");
    }

    match check_slot(&items, 1) { // Match both cases, example found in vectors.rs
        Some(item) => println!("Slot 1 contains {item}."),
        None => println!("There is no item in slot 1."),
    }
    match check_slot(&items, 99) {
        Some(item) => println!("Slot 99 contains {item}."),
        None => println!("There is no item in slot 99."),
    }
    // impossible_state(); // Forces an error to cause a crash with panic!
    // RUST_BACKTRACE=1 listed the function calls that led to the panic:
    // main (line 54) called impossible_state, which panicked at line 23.

    // let missing = &items[99]; // `[]` promises an item so when it fails (slot 99 doesn't exist)
    // .. there's no correct value to give back, so it stops. `.get` would return None instead.
}
