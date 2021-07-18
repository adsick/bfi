pub mod program;

pub use program::*;


use std::ops::{AddAssign, SubAssign};

//fixed size of program memory and data memory
const MEM: usize = 1024;
const LIMIT: u16 = u16::MAX;

type BfResult = Result<(), BfError>;
pub struct Bf {
    data: [u8; MEM],
    program: BfProgram,
    cmd_indx: usize,
    data_ptr: usize,
    limit: usize,
}

impl Bf {
    pub fn new(program: BfProgram) -> Self {
        Bf {
            data: [0; MEM],
            program,
            cmd_indx: 0,
            data_ptr: 0,
            limit: 0,
        }
    }
    fn tick(&mut self) -> BfResult {
        if let Some(ix) = self.cmd_indx.checked_add(1) {
            if ix < MEM {
                self.cmd_indx = ix;
                return Ok(());
            }
        }
        Err(BfError::Runaway)
    }
    pub fn execute(&mut self) -> String {
        use program::BfCommand::*;

        let res = String::new();
        //let mut i = 0;

        loop {
            let c = unsafe { self.program.get_unchecked(self.cmd_indx) };
            #[cfg(feature = "debug")]
            {
                println!("command: {:?}", c);
                println!("cmd_idx: {}", self.cmd_indx);
            }

            match c {
                Right => {
                    if let Err(e) = self.right() {
                        eprintln!("runtime error: {:?}", e);
                        break;
                    }
                }
                Left => {
                    if let Err(e) = self.left() {
                        eprintln!("runtime error: {:?}", e);
                        break;
                    }
                }
                Add => {
                    if let Err(e) = self.add() {
                        eprintln!("runtime error: {:?}", e);
                        break;
                    }
                }
                Sub => {
                    if let Err(e) = self.sub() {
                        eprintln!("runtime error: {:?}", e);
                        break;
                    }
                }
                Get => {
                    print!("{}", self.get() as char);
                    if let Err(e) = self.tick() {
                        eprintln!("runtime error: {:?}", e);
                        break;
                    }
                }
                //Set => self.set(),
                JumpRight => {
                    if let Err(e) = self.jump_right() {
                        eprintln!("runtime error: {:?}", e);
                        break;
                    }
                }
                JumpLeft => {
                    if let Err(e) = self.jump_left() {
                        println!("runtime error: {:?}", e);
                        break;
                    }
                }
                Hlt => break,
            }
            #[cfg(feature = "debug")]
            {
                println!("{:?}", self.data);
                println!("step: {}", self.limit);
            }
            self.limit+=1;
            if self.limit > LIMIT as usize{
                eprint!("time limit reached. force stop.");
                break;
            }
        }
        res
    }

    fn left(&mut self) -> BfResult {
        self.tick()?;
        if let Some(ptr) = self.data_ptr.checked_sub(1) {
            self.data_ptr = ptr;
            return Ok(());
        } else {
            return Err(BfError::Segfault);
        }
    }

    fn right(&mut self) -> BfResult {
        self.tick()?;
        self.data_ptr += 1;
        if self.data_ptr >= MEM {
            return Err(BfError::Segfault);
        } else {
            Ok(())
        }
    }

    fn add(&mut self) -> BfResult {
        //we don't care about overflow in release - it will wrap on overflows.
        self.tick()?;
        self.data[self.data_ptr].add_assign(1);
        Ok(())
    }

    fn sub(&mut self) -> BfResult {
        // same thing here
        self.tick()?;
        self.data[self.data_ptr].sub_assign(1);
        Ok(())
    }

    fn get(&self) -> u8 {
        self.data[self.data_ptr]
    }

    // fn set(&mut self, value: u8) {
    //     self.data[self.data_ptr] = value;
    // }

    fn jump_right(&mut self) -> BfResult {
        if self.data[self.data_ptr] != 0 {
            return self.tick();
        }

        let mut jc = 1;

        while jc != 0 {
            //runaway check here
            self.cmd_indx += 1;
            let ix = self.cmd_indx;
            if ix >= MEM {
                return Err(BfError::Runaway);
            }

            let c = unsafe { self.program.get_unchecked(ix) };
            if let BfCommand::JumpLeft = c {
                jc -= 1;
            } else if let BfCommand::JumpRight = c {
                jc += 1;
            }
        }
        self.cmd_indx += 1; //it has to sit after that jump left
        Ok(())
    }
    fn jump_left(&mut self) -> BfResult {
        if self.data[self.data_ptr] == 0 {
            return self.tick();
        }
        let mut jc = -1;

        while jc != 0 {
            //runaway check here
            if let Some(ix) = self.cmd_indx.checked_sub(1) {
                self.cmd_indx = ix;
            } else {
                return Err(BfError::Runaway);
            }
            let ix = self.cmd_indx;

            let c = unsafe { self.program.get_unchecked(ix) };
            if let BfCommand::JumpLeft = c {
                jc -= 1;
            } else if let BfCommand::JumpRight = c {
                jc += 1;
            }
        }
        if let Some(ix) = self.cmd_indx.checked_add(1) {
            self.cmd_indx = ix;
        } else {
            return Err(BfError::Runaway);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum BfError {
    Segfault,
    Runaway, //when cmd_index hits out of boundaries
}
