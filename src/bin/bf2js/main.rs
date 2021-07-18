use std::io;

mod translator;
use bfi::BfProgram;
use translator::bf_to_js;
fn main(){
    let mut usr_input = String::new();
    println!("code here : ");

    io::stdin()
        .read_line(&mut usr_input)
        .expect("smth really bad happened");

        //here is print0to99 program for testing purposes: 
        // usr_input = r#"++++++++++
        // >++++++++++++++++++++++++++++++++++++++++++++++++
        // >++++++++++
        // [
        //   >++++++++++++++++++++++++++++++++++++++++++++++++
        //   >++++++++++
        //   [
        //     <<<. >>.+ <<<.
        //     >>>>-
        //   ]
        //   <----------------------------------------------------------
        //   <<+
        //   >
        //   -
        // ]"#.to_string();

    if let Ok(prog) = BfProgram::new(usr_input){
        let code = bf_to_js(prog);
        std::fs::write("bf.js", code).expect("failed writing to main.js");
        println!("looks like we are good, now check the 'bf.js' file...")
    } else {
        eprintln!("failed to parse the program. probably it is too big, or it has invalid UTF-8.")
    }

}