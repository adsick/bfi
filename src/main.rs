use bfi::{Bf, BfProgram};

fn main() {
    if let Ok(program) = BfProgram::new("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."){
        let mut bf = Bf::new(program);
        bf.execute();
    } else {
        eprintln!("parse error (too long)")
    }
}
