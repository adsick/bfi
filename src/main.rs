use std::io;

use bfi::{Bf, BfProgram};

fn main() {
    let mut usr_input = String::new();
    println!("code here : ");

    io::stdin()
        .read_line(&mut usr_input)
        .expect("smth really bad happened");
    if let Ok(program) = BfProgram::new(usr_input) {
        let mut bf = Bf::new(program);
        bf.execute();
    } else {
        eprintln!("parse error (too long)")
    }
    println!("programm halted.");
}
