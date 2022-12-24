use std::str::FromStr;

use console::style;

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

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub op: Type,
    pub dest: Unit,
    pub src1: Option<Value>,
    pub src2: Option<Value>,

    pub emit_cycle: Option<u64>,
    pub start_cycle: Option<u64>,
    pub exec_cycle: Option<u64>,
    pub write_cycle: Option<u64>,

    // The number of cycles left to finish the instruction.
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
            start_cycle: None,
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

    /// Emit the instruction.
    pub fn emit(&mut self, cycle: u64) {
        self.emit_cycle.replace(cycle);
        self.left_cycle.replace(self.latency());
    }

    /// Execute the instruction.
    pub fn exec(&mut self, cycle: u64) -> bool {
        if let Some(left) = self.left_cycle {
            if left == 0 {
                self.left_cycle.take();
                self.exec_cycle.replace(cycle - 1);
                true
            } else if left == self.latency() {
                self.start_cycle.replace(cycle);
                self.left_cycle.replace(left - 1);
                false
            } else {
                self.left_cycle.replace(left - 1);
                false
            }
        } else {
            false
        }
    }

    #[inline]
    pub fn write(&mut self, cycle: u64) {
        self.write_cycle.replace(cycle);
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let op = iter.next().expect("no op").parse()?;

        let dest = iter.next().expect("no dest").parse()?;

        let mut src1 = None;
        let mut src2 = None;

        let next = iter.next().expect("no src1");
        src1.replace(value::new(match next.parse::<Unit>() {
            Ok(u) => u.into(),
            Err(_) => next
                .trim_end_matches('+')
                .parse::<i64>()
                .map_err(|_| ())?
                .into(),
        }));

        let next = iter.next().expect("no src2");
        src2.replace(value::new(match next.parse::<Unit>() {
            Ok(u) => u.into(),
            Err(_) => next
                .trim_end_matches('+')
                .parse::<i64>()
                .map_err(|_| ())?
                .into(),
        }));

        Ok(Instruction {
            op,
            dest,
            src1,
            src2,
            emit_cycle: None,
            exec_cycle: None,
            start_cycle: None,
            write_cycle: None,
            left_cycle: None,
        })
    }
}

impl FromStr for Type {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADDD" => Ok(Type::ADDD),
            "SUBD" => Ok(Type::SUBD),
            "MULTD" => Ok(Type::MULTD),
            "DIVD" => Ok(Type::DIVD),
            "LD" => Ok(Type::LD),
            "SD" => Ok(Type::SD),
            _ => Err(()),
        }
    }
}

impl Type {
    pub fn op_str(&self) -> &'static str {
        match self {
            Type::ADDD => "+",
            Type::SUBD => "-",
            Type::MULTD => "*",
            Type::DIVD => "/",
            _ => "",
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Type::ADDD => style(format!("{self:?}")).green(),
            Type::SUBD => style(format!("{self:?}")).red(),
            Type::MULTD => style(format!("{self:?}")).yellow(),
            Type::DIVD => style(format!("{self:?}")).blue(),
            Type::LD => style(format!("{self:?}")).cyan(),
            Type::SD => style(format!("{self:?}")).magenta(),
        };
        write!(f, "{s:<5}")
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let inst = format!(
            "{:?} {} {} {}",
            self.op,
            self.dest,
            self.src1.as_ref().unwrap(),
            self.src2.as_ref().unwrap()
        );
        let emit = match self.emit_cycle {
            Some(c) => c.to_string(),
            None => " ".to_string(),
        };
        let start = match self.start_cycle {
            Some(c) => c.to_string(),
            None => " ".to_string(),
        };
        let exec = match self.exec_cycle {
            Some(c) => c.to_string(),
            None => " ".to_string(),
        };
        let write = match self.write_cycle {
            Some(c) => c.to_string(),
            None => " ".to_string(),
        };
        write!(
            f,
            "{:<20}: {:>3}, {:>3}, {:>3}, {:>3}",
            style(inst).white().bold(),
            style(emit).red(),
            style(start).blue(),
            style(exec).yellow(),
            style(write).green()
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let insts = [
            r"
        LD F2 0 R2
        LD F4 0 R3
        DIVD F0 F4 F2
        MULTD F6 F0 F2
        ADDD F0 F4 F2
        SD F6 0 R3
        MULTD F6 F0 F2
        SD F6 0 R1",
            r"
        LD F6 34+ R2
        LD F2 45+ R3
        MULTD F0 F2 F4
        SUBD F8 F6 F2
        DIVD F10 F0 F6
        ADDD F6 F8 F2",
        ];

        for inst in insts.iter() {
            for line in inst.lines() {
                if line.is_empty() {
                    continue;
                }
                if let Ok(inst) = line.trim().parse::<Instruction>() {
                    println!("{inst}, {inst:?}");
                }
            }
            println!();
        }
    }
}
