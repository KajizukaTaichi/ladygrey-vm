fn main() {
    let mut calc = Machine {
        stack: Vec::new(),
        heap: vec![Type::Null; RAM_SPEC].try_into().unwrap(),
        code: calc_compiler("1 + 2 * 4 / 3 - 5".to_string()),
        ar: 0,
        pc: 0,
    };
    calc.run();
    let result = calc.heap[0].clone();
    dbg!(result);
}

fn calc_compiler(source: String) -> Vec<Instruction> {
    let tokens: Vec<&str> = source.split_whitespace().collect();

    let mut index = 0;
    let mut code = vec![];

    while tokens.len() > index {
        match tokens[index] {
            "+" | "-" | "*" | "/" => {
                let value = tokens[index + 1].parse().unwrap();
                code.push(Instruction::Store(Type::Integer(value)));
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
const RAM_SPEC: usize = 5;

#[derive(Debug, Clone)]
struct Machine {
    stack: Vec<usize>,
    heap: [Type; RAM_SPEC],
    code: Vec<Instruction>,

    /// Program Counter
    pc: usize,
    /// Address Register
    ar: usize,
}

impl Machine {
    fn run(&mut self) {
        eprintln!("LadyGrey VM is starting up...");
        while self.code.len() > self.pc {
            let instruction = self.code[self.pc].clone();
            match instruction {
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

            self.pc += 1;
            dbg!(&self);
        }
    }
    fn pop(&mut self) -> usize {
        self.stack.pop().expect("Stack underflow")
    }
}

#[derive(Debug, Clone)]
enum Type {
    Integer(i64),
    Bool(bool),
    Null,
}

#[allow(warnings)]
#[derive(Debug, Clone)]
enum Instruction {
    Push(usize),
    Dup,
    Swap,
    Store(Type),
    Copy,
    Move,
    Inc,
    Dec,
    Add,
    Sub,
    Mul,
    Div,
    Ar,
    Jump,
    Equal,
    Not,
}
