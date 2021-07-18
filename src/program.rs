use std::slice::Iter;

use super::MEM;

pub struct BfProgram {
    commands: [BfCommand; MEM],
}

impl BfProgram {
    pub fn new<T: ToString>(string: T) -> Result<Self, BfParseError> {
        //TODO error handling
        let commands = Self::parse::<T>(string)?;

        Ok(BfProgram { commands })
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> &BfCommand {
        self.commands.get_unchecked(index)
    }

    fn parse<T: ToString>(string: T) -> Result<[BfCommand; MEM], BfParseError> {
        use BfCommand::*;

        let s = string.to_string();
        let mut commands = [Hlt; MEM];
        let mut i = 0;

        for c in s.chars() {
            if i >= MEM {
                return Err(BfParseError::TooLong);
            }
            commands[i] = match c {
                '>' => Right,
                '<' => Left,
                '+' => Add,
                '-' => Sub,
                '.' => Get,
                //',' => Set,
                '[' => JumpRight,
                ']' => JumpLeft,
                _ => continue,
            };
            #[cfg(feature = "debug")]
            {
                println!("{} parsed: {:?}", i, commands[i]);
            }
            i += 1;
        }

        Ok(commands)
    }
}

impl<'a> IntoIterator for &'a BfProgram{
    type Item = &'a BfCommand;

    type IntoIter = Iter<'a, BfCommand>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.iter()
    }
}


#[derive(Debug, Clone, Copy)]
pub enum BfCommand {
    Right, //move data pointer to the right.
    Left,  //move data pointer to the left.
    Add,
    Sub,
    Get,
    //Set,
    JumpRight, //jump rightwise after matching JumpLeft if data behind the ptr is zero.
    JumpLeft,  //jump leftwise before matching JumpRight if data behind the ptr is nonzero.
    Hlt,
}

#[derive(Debug)]
pub enum BfParseError {
    TooLong,
}
