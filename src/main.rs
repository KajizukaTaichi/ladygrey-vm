fn main() {
    println!("LadyGray VM");
    let mut larsa = Machine {
        stack: Vec::new(),
        heap: vec![Type::Null; RAM_SPEC].try_into().unwrap(),
        code: vec![
            Instruction::Store(Type::Integer(1)),
            Instruction::Store(Type::Integer(2)),
            Instruction::Mul,
            Instruction::Push(2),
            Instruction::Ar,
            Instruction::Push(1),
            Instruction::Store(Type::Bool(true)),
            Instruction::Jump,
        ],
        ar: 0,
        pc: 0,
    };
    larsa.run();
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
        while self.code.len() > self.pc {
            let instruction = self.code[self.pc].clone();
            match instruction {
                Instruction::Copy => {
                    let address = self.pop();
                    self.heap[self.ar] = self.heap[address].clone();
                    self.stack.push(self.ar);
                    self.ar += 1;
                }
                Instruction::Ar => {
                    let ar = self.pop();
                    self.ar = ar;
                }
                Instruction::Move => {
                    let address = self.pop();
                    self.heap[self.ar] = self.heap[address].clone();
                    self.heap[address] = Type::Null;
                    self.stack.push(self.ar);
                    self.ar += 1;
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
                Instruction::Store(value) => {
                    self.heap[self.ar] = value;
                    self.stack.push(self.ar);
                    self.ar += 1;
                }
                Instruction::Push(value) => {
                    self.stack.push(value);
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
                Instruction::Add => {
                    let address = self.pop();
                    if let Type::Integer(a) = self.heap[address].clone() {
                        let base = self.pop();
                        if let Type::Integer(b) = self.heap[base].clone() {
                            self.heap[base] = Type::Integer(a + b);
                            self.stack.push(base);
                        }
                    }
                }
                Instruction::Sub => {
                    let address = self.pop();
                    if let Type::Integer(a) = self.heap[address].clone() {
                        let base = self.pop();
                        if let Type::Integer(b) = self.heap[base].clone() {
                            self.heap[base] = Type::Integer(a - b);
                            self.stack.push(base);
                        }
                    }
                }
                Instruction::Mul => {
                    let address = self.pop();
                    if let Type::Integer(a) = self.heap[address].clone() {
                        let base = self.pop();
                        if let Type::Integer(b) = self.heap[base].clone() {
                            self.heap[base] = Type::Integer(a * b);
                            self.stack.push(base);
                        }
                    }
                }
                Instruction::Div => {
                    let address = self.pop();
                    if let Type::Integer(a) = self.heap[address].clone() {
                        let base = self.pop();
                        if let Type::Integer(b) = self.heap[base].clone() {
                            self.heap[base] = Type::Integer(a / b);
                            self.stack.push(base);
                        }
                    }
                }
            }
            dbg!(&self);
            self.pc += 1;
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
