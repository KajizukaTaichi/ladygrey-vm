fn main() {
    let mut calc = Machine {
        stack: Vec::new(),
        heap: vec![Type::Null; RAM_SPEC].try_into().unwrap(),
        code: calc_compiler("1 + 2 + 3 + 4".to_string()),
        ar: 0,
        pc: 0,
    };
    calc.start();
    let result = calc.heap[0].clone();
    dbg!(result);
}

/// Compiler of simple calculation expression
fn calc_compiler(source: String) -> Vec<Instruction> {
    let tokens: Vec<&str> = source.split_whitespace().collect();

    let mut index = 0;
    let mut code = vec![];

    while tokens.len() > index {
        match tokens[index] {
            "+" | "-" | "*" | "/" => {
                let value = tokens[index + 1].parse().unwrap();
                code.push(Instruction::Store(Type::Integer(value)));
                code.push(Instruction::Push(1));
                code.push(Instruction::Ar);
                code.push(match tokens[index] {
                    "+" => Instruction::Add,
                    "-" => Instruction::Sub,
                    "*" => Instruction::Mul,
                    "/" => Instruction::Div,
                    _ => panic!("Invalid operator"),
                });
                index += 2;
            }
            _ => {
                let value = tokens[index].parse().unwrap();
                code.push(Instruction::Store(Type::Integer(value)));
                index += 1;
            }
        }
    }
    code
}

/// Spec of heap area in RAM
const RAM_SPEC: usize = 2;

/// Core machine body of LadyGrey VM
#[derive(Debug, Clone)]
struct Machine {
    /// Area that have address
    stack: Vec<usize>,
    /// Area that objects are stored
    heap: [Type; RAM_SPEC],
    /// Program code to be executed
    code: Vec<Instruction>,

    /// Program Counter
    pc: usize,
    /// Address Register
    ar: usize,
}


/// Data type system
#[derive(Debug, Clone)]
enum Type {
    Integer(i64),
    Bool(bool),
    Null,
}

/// ISA (Instruction Set Architecture)
#[allow(warnings)]
#[derive(Debug, Clone)]
enum Instruction {    
    /// Copy value from heap to other area allocated by AR
    Copy,
    
    /// Move value from heap to other area allocated by AR
    Move,

    /// Store value on heap and push that address
    Store(Type),

    /// Set value of AR　(Address Register)
    Ar,
    
    /// Jump to the address if condition is true
    Jump,

    /// Push value to the stack
    Push(usize),
    
    /// Duplicate top value on the stack
    Dup,
    
    /// Swap top two values on the stack
    Swap,
    
    /// Compare two value and push result to stack
    Equal,

    /// Invert boolean value
    Not,

    /// Increment integer value
    Inc,

    /// Decrement integer value
    Dec,

    /// Addition two number
    Add,
    
    /// Subtraction two number
    Sub,
    
    /// Multiplication two number
    Mul,
    
    /// Division two number
    Div,
}

impl Machine {
    /// Start running program
    fn start(&mut self) {
        eprintln!("LadyGrey VM is starting up...");
        while self.code.len() > self.pc {
            let instruction = self.code[self.pc].clone();
            match instruction {
                // About memory and register
                Instruction::Copy => {
                    let address = self.pop();
                    self.heap[self.ar] = self.heap[address].clone();
                    self.stack.push(self.ar);
                    self.ar += 1;
                }
                Instruction::Move => {
                    let address = self.pop();
                    self.heap[self.ar] = self.heap[address].clone();
                    self.heap[address] = Type::Null;
                    self.stack.push(self.ar);
                    self.ar += 1;
                }
                Instruction::Store(value) => {
                    self.heap[self.ar] = value;
                    self.stack.push(self.ar);
                    self.ar += 1;
                }
                Instruction::Ar => {
                    let ar = self.pop();
                    self.ar = ar;
                }
                Instruction::Jump => {
                    let address = self.pop();
                    let condition = if let Type::Bool(b) = self.heap[address].clone() {
                        b
                    } else {
                        false
                    };
                    let jump_to = self.pop();
                    if condition {
                        self.pc = jump_to;
                        continue;
                    }
                }

                // About stack
                Instruction::Dup => {
                    let value = self.pop();
                    self.stack.push(value);
                    self.stack.push(value);
                }
                Instruction::Swap => {
                    let a = self.pop();
                    let b = self.pop();
                    self.stack.push(a);
                    self.stack.push(b);
                }
                Instruction::Push(value) => {
                    self.stack.push(value);
                }

                // About logical processing
                Instruction::Equal => {
                    let address = self.pop();
                    if let Type::Integer(a) = self.heap[address].clone() {
                        let base = self.pop();
                        if let Type::Integer(b) = self.heap[base].clone() {
                            self.heap[base] = Type::Bool(format!("{a:?}") == format!("{b:?}"));
                            self.stack.push(base);
                        }
                    }
                }
                Instruction::Not => {
                    let address = self.pop();
                    if let Type::Bool(b) = self.heap[address] {
                        self.heap[address] = Type::Bool(!b);
                    }
                    self.stack.push(address);
                }

                // About arithmetic processing
                Instruction::Inc => {
                    let address = self.pop();
                    if let Type::Integer(i) = self.heap[address] {
                        self.heap[address] = Type::Integer(i + 1);
                    }
                    self.stack.push(address);
                }
                Instruction::Dec => {
                    let address = self.pop();
                    if let Type::Integer(i) = self.heap[address] {
                        self.heap[address] = Type::Integer(i - 1);
                    }
                    self.stack.push(address);
                }
                Instruction::Add => {
                    let address = self.pop();
                    if let Type::Integer(a) = self.heap[address].clone() {
                        let base = self.pop();
                        if let Type::Integer(b) = self.heap[base].clone() {
                            self.heap[base] = Type::Integer(b + a);
                            self.stack.push(base);
                        }
                    }
                }
                Instruction::Sub => {
                    let address = self.pop();
                    if let Type::Integer(a) = self.heap[address].clone() {
                        let base = self.pop();
                        if let Type::Integer(b) = self.heap[base].clone() {
                            self.heap[base] = Type::Integer(b - a);
                            self.stack.push(base);
                        }
                    }
                }
                Instruction::Mul => {
                    let address = self.pop();
                    if let Type::Integer(a) = self.heap[address].clone() {
                        let base = self.pop();
                        if let Type::Integer(b) = self.heap[base].clone() {
                            self.heap[base] = Type::Integer(b * a);
                            self.stack.push(base);
                        }
                    }
                }
                Instruction::Div => {
                    let address = self.pop();
                    if let Type::Integer(a) = self.heap[address].clone() {
                        let base = self.pop();
                        if let Type::Integer(b) = self.heap[base].clone() {
                            self.heap[base] = Type::Integer(b / a);
                            self.stack.push(base);
                        }
                    }
                }
            }

            // Move to next instruction
            self.pc += 1;
        }

        // Show dump
        dbg!(self);
    }

    /// Pop the stack's top value
    fn pop(&mut self) -> usize {
        self.stack.pop().expect("Stack underflow")
    }
}
