pub mod errors;
pub mod instructions;
pub mod state;

use crate::errors::InterpreterError;
use crate::instructions::ByteCode;
use crate::state::Variable;

pub struct Interpreter {
    pub instruction_pointer: u32,
    pub stack: Vec<Variable>,
}

impl Interpreter {
    pub fn new() -> Self {
        return Self {
            instruction_pointer: 0,
            stack: vec![],
        };
    }

    pub fn clear_stack(&mut self) {
        self.stack = vec![];
    }

    /// Push a variable to stack. 3 Possibilities:
    /// - Variable has no identifier, it gets pushed to stack
    /// - Variable has an identifier and identifier is unused: push to stack
    /// - Variable has an identifier and identifier is already taken: mutate existing variable
    fn push_to_stack(
        &mut self,
        identifier: Option<char>,
        value: i32,
    ) -> Result<(), InterpreterError> {
        // check if var has identifier
        match identifier {
            Some(id) => {
                //if identifier exists, modify value
                if self
                    .stack
                    .iter()
                    .find(|elem| elem.identifier == Some(id))
                    .is_some()
                {
                    self.stack
                        .iter_mut()
                        .filter(|elem| elem.identifier == Some(id))
                        .for_each(|mut elem| elem.value = value);
                } else {
                    self.stack.push(Variable {
                        identifier: identifier,
                        value: value,
                    });
                }
            }
            // variable has no identifier, can just push
            None => self.stack.push(Variable {
                identifier: identifier,
                value: value,
            }),
        }

        return Ok(());
    }

    /// Pop k values from the stack and return them as an array
    fn get_from_stack(&mut self, nb_items: u32) -> Result<Vec<Variable>, InterpreterError> {
        let mut values: Vec<Variable> = vec![];

        for _ in 0..nb_items {
            let temp = self.stack.pop();

            match temp {
                None => return Err(InterpreterError::StackUnderflow),
                Some(val) => values.push(val),
            }
        }

        return Ok(values);
    }

    /// Advance instruction pointer by one. Checks if new position is valid.
    fn advance_ip(&mut self, bytecode_len: u32) -> Result<(), InterpreterError> {
        self.instruction_pointer += 1;

        if self.instruction_pointer >= bytecode_len {
            return Err(InterpreterError::InvalidPointerAddress);
        } else {
            return Ok(());
        }
    }

    /// Set instruction pointer to given position, used by jump instructions. Checks if new position is valid
    fn set_ip(&mut self, destination: u32, bytecode_len: u32) -> Result<(), InterpreterError> {
        self.instruction_pointer = destination;

        if self.instruction_pointer >= bytecode_len {
            return Err(InterpreterError::InvalidPointerAddress);
        } else {
            return Ok(());
        }
    }

    /// Take bytecode as input and run it. At the the end returns topmost value in stack
    pub fn run(&mut self, bytecode: Vec<ByteCode>) -> Result<i32, InterpreterError> {
        self.instruction_pointer = 0;
        let bytecode_len = bytecode.len() as u32;

        let mut advance_ip: bool;
        loop {
            advance_ip = true;
            match bytecode[self.instruction_pointer as usize] {
                ByteCode::Return => break,
                ByteCode::LoadVal(val) => self.push_to_stack(None, val)?,
                ByteCode::WriteVar(id) => {
                    let val = self.stack.pop();
                    match val {
                        None => return Err(InterpreterError::StackUnderflow),
                        Some(v) => self.push_to_stack(Some(id), v.value)?,
                    }
                }
                ByteCode::ReadVar(id) => {
                    // find var
                    let var = self
                        .stack
                        .iter()
                        .find(|x| x.identifier == Some(id))
                        .map(|elem| elem.to_owned());

                    match var {
                        None => return Err(InterpreterError::UnknownVariable { identifier: id }),
                        Some(v) => {
                            let temp = v.to_owned().clone();
                            self.push_to_stack(None, temp.value)?;
                        }
                    }
                }
                ByteCode::Add => {
                    let vals = self.get_from_stack(2)?;
                    self.push_to_stack(None, vals[0].value + vals[1].value)?;
                }
                ByteCode::Sub => {
                    let vals = self.get_from_stack(2)?;
                    self.push_to_stack(None, vals[0].value - vals[1].value)?;
                }
                ByteCode::Mul => {
                    let vals = self.get_from_stack(2)?;
                    self.push_to_stack(None, vals[0].value * vals[1].value)?;
                }
                ByteCode::Div => {
                    let vals = self.get_from_stack(2)?;
                    self.push_to_stack(None, vals[0].value / vals[1].value)?;
                }

                // comparisons
                ByteCode::Greater => {
                    let vals = self.get_from_stack(2)?;
                    self.push_to_stack(None, (vals[0].value > vals[1].value) as i32)?;
                }
                ByteCode::Lower => {
                    let vals = self.get_from_stack(2)?;
                    self.push_to_stack(None, (vals[0].value < vals[1].value) as i32)?;
                }
                ByteCode::Eq => {
                    let vals = self.get_from_stack(2)?;
                    self.push_to_stack(None, (vals[0].value == vals[1].value) as i32)?;
                }
                ByteCode::GreaterEq => {
                    let vals = self.get_from_stack(2)?;
                    self.push_to_stack(None, (vals[0].value >= vals[1].value) as i32)?;
                }
                ByteCode::LowerEq => {
                    let vals = self.get_from_stack(2)?;
                    self.push_to_stack(None, (vals[0].value <= vals[1].value) as i32)?;
                }

                // jumps
                ByteCode::Jump(destination) => {
                    self.set_ip(destination, bytecode_len)?;
                    advance_ip = false;
                }
                ByteCode::JumpI(destination) => {
                    if self.get_from_stack(1)?[0].value == 1 {
                        self.set_ip(destination, bytecode_len)?;
                        advance_ip = false;
                    }
                }
            }

            if advance_ip {
                self.advance_ip(bytecode_len)?;
            }
        }

        let return_val = self.stack.pop();

        match return_val {
            Some(val) => return Ok(val.value),
            None => return Err(InterpreterError::StackUnderflow),
        }
    }
}
