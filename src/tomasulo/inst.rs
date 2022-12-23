use super::*;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Type {
    ADDD,
    SUBD,
    MULTD,
    DIVD,
    LD,
    SD,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Instruction {
    pub op: Type,
    pub dest: Unit,
    pub src1: Option<Value>,
    pub src2: Option<Value>,

    emit_cycle: Option<u64>,
    exec_cycle: Option<u64>,
    write_cycle: Option<u64>,

    left_cycle: Option<u64>,
}

impl Instruction {
    pub fn new(op: Type, dest: Unit) -> Instruction {
        Instruction {
            op,
            dest,
            src1: None,
            src2: None,
            emit_cycle: None,
            exec_cycle: None,
            write_cycle: None,
            left_cycle: None,
        }
    }

    fn latency(&self) -> u64 {
        match self.op {
            Type::ADDD | Type::SUBD | Type::LD | Type::SD => 2,
            Type::MULTD => 10,
            Type::DIVD => 20,
        }
    }

    pub fn emit(&mut self, cycle: u64) {
        self.emit_cycle.replace(cycle);
        self.left_cycle.replace(self.latency());
    }

    pub fn exec(&mut self, cycle: u64) -> bool {
        if let Some(left) = self.left_cycle {
            if left == 0 {
                self.exec_cycle.replace(cycle);
                self.left_cycle.take();
                true
            } else {
                self.left_cycle.replace(left - 1);
                false
            }
        } else {
            false
        }
    }

    pub fn write(&mut self, cycle: u64) {
        self.write_cycle.replace(cycle);
    }

    pub fn emit_cycle(&self) -> Option<u64> {
        self.emit_cycle
    }

    pub fn exec_cycle(&self) -> Option<u64> {
        self.exec_cycle
    }

    pub fn write_cycle(&self) -> Option<u64> {
        self.write_cycle
    }

    pub fn parse(line: &str) -> Option<Instruction> {
        let mut iter = line.split_whitespace();
        let op = Type::parse(iter.next()?)?;

        let dest = Unit::parse(iter.next()?)?;

        let mut src1 = None;
        let mut src2 = None;

        let next = iter.next()?;
        src1.replace(value::new(match Unit::parse(next) {
            Some(u) => u.into(),
            None => next.trim_end_matches('+').parse::<i64>().ok()?.into()
        }));

        let next = iter.next()?;
        src2.replace(value::new(match Unit::parse(next) {
            Some(u) => u.into(),
            None => next.trim_end_matches('+').parse::<i64>().ok()?.into()
        }));

        Some(Instruction {
            op,
            dest,
            src1,
            src2,
            emit_cycle: None,
            exec_cycle: None,
            write_cycle: None,
            left_cycle: None,
        })
    }
}

impl Type {
    pub fn parse(line: &str) -> Option<Type> {
        match line {
            "ADDD" => Some(Type::ADDD),
            "SUBD" => Some(Type::SUBD),
            "MULTD" => Some(Type::MULTD),
            "DIVD" => Some(Type::DIVD),
            "LD" => Some(Type::LD),
            "SD" => Some(Type::SD),
            _ => None,
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::ADDD => write!(f, "+"),
            Type::SUBD => write!(f, "-"),
            Type::MULTD => write!(f, "*"),
            Type::DIVD => write!(f, "/"),
            _ => Ok(()),
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} {} ", self.op, self.dest)?;
        if let Some(src1) = self.src1.as_ref() {
            write!(f, "{src1} ")?;
        }
        if let Some(src2) = self.src2.as_ref() {
            write!(f, "{src2} ")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let insts = [r"
        LD F2 0 R2
        LD F4 0 R3
        DIVD F0 F4 F2
        MULTD F6 F0 F2
        ADDD F0 F4 F2
        SD F6 0 R3
        MULTD F6 F0 F2
        SD F6 0 R1", r"
        LD F6 34+ R2
        LD F2 45+ R3
        MULTD 0 F2 F4
        SUBD F8 F6 F2
        DIVD F10 F0 F6
        ADDD F6 F8 F2"];

        for inst in insts.iter() {
            for line in inst.lines() {
                if line.is_empty() {
                    continue;
                }
                if let Some(inst) = Instruction::parse(line.trim()) {
                    println!("{inst}");
                }
            }
            println!();
        }
    }
}
