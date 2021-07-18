use bfi::BfCommand::*;
use bfi::BfProgram;
const MEM: usize = 128;

pub fn bf_to_js(prog: BfProgram) -> String {
    let mut res = String::new();

    res.push_str(&format!(
        r#"var data = new Uint8Array({})
    var ptr = 0
    var res = ''
    "#,
        MEM
    ));

    let mut dptr = 0;
    let mut delta = 0;
    for c in prog.into_iter() {
        match c {
            Right => {
                dptr += 1;

                if delta != 0 {
                    res.push_str(&format!("data[ptr]+={}\n", delta));
                    delta = 0;
                }
            }
            Left => {
                dptr -= 1;

                if delta != 0 {
                    res.push_str(&format!("data[ptr]+={}\n", delta));
                    delta = 0;
                }
            }
            Add => {
                delta += 1;

                if dptr != 0 {
                    res.push_str(&format!("ptr+={}\n", dptr));
                    dptr = 0;
                }
            }
            Sub => {
                delta -= 1;

                if dptr != 0 {
                    res.push_str(&format!("ptr+={}\n", dptr));
                    dptr = 0;
                }
            }
            Get => {
                if dptr != 0 {
                    res.push_str(&format!("ptr+={}\n", dptr));
                    dptr = 0;
                }

                if delta != 0 {
                    res.push_str(&format!("data[ptr]+={}\n", delta));
                    delta = 0;
                }

                res.push_str("res+=String.fromCharCode(data[ptr])\n")
            }
            JumpRight => {
                if dptr != 0 {
                    res.push_str(&format!("ptr+={}\n", dptr));
                    dptr = 0;
                }

                if delta != 0 {
                    res.push_str(&format!("data[ptr]+={}\n", delta));
                    delta = 0;
                }
                res.push_str("while(data[ptr]){\n")
            }
            JumpLeft => {
                if dptr != 0 {
                    res.push_str(&format!("ptr+={}\n", dptr));
                    dptr = 0;
                }

                if delta != 0 {
                    res.push_str(&format!("data[ptr]+={}\n", delta));
                    delta = 0;
                }
                res.push_str("}\n")
            }
            Hlt => {
                res.push_str("console.log(res);\n");
                break;
            }
        }
    }

    res
}
