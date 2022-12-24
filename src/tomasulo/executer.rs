use console::style;
use std::collections::VecDeque;

use super::*;

pub struct Executer {
    pub rs: ReservationStation,
    pub fu: FloatingUnit,
    pub insts: VecDeque<Instruction>,
    pub insts_comp: Vec<Instruction>,
    pub inst_count: usize,
    pub cycle: u64,
    pub finished: bool,
}

impl Executer {
    pub fn new() -> Executer {
        Executer {
            rs: ReservationStation::new(),
            fu: FloatingUnit::new(),
            insts: VecDeque::new(),
            insts_comp: Vec::new(),
            inst_count: 0,
            cycle: 0,
            finished: false,
        }
    }

    #[inline]
    pub fn add_insts(&mut self, inst: &[Instruction]) {
        self.insts.extend(inst.iter().cloned());
        self.inst_count = inst.len();
    }

    pub fn clear(&mut self) {
        self.rs.clear();
        self.fu.clear();
        self.insts.clear();
        self.cycle = 0;
        self.finished = false;
    }

    pub fn run(&mut self) {
        while !self.finished {
            self.cycle += 1;

            self.issue();
            let comp = self.execute();
            self.finished = self.insts_comp.len() == self.inst_count;
            println!("{self:?}");
            self.write(comp);

            self.print_insts();
        }
    }

    fn print_insts(&mut self) {
        for inst in self.insts_comp.iter_mut() {
            println!("{inst}");
        }
    }

    fn issue(&mut self) {
        if let Some(inst) = self.insts.pop_front() {
            if let Some(rs_id) = self.rs.get_free(inst.op.into()) {
                if let Some(rs) = self.rs.get_mut(rs_id) {
                    if let Unit::Fu(id) = inst.dest {
                        self.fu.mark_busy(id, rs_id);
                        rs.apply(inst, &self.fu, self.cycle);
                    } else {
                        panic!("Destination of instruction is not a register.")
                    }
                    return;
                }
            }
            self.insts.push_front(inst);
        }
    }

    #[inline]
    fn execute(&mut self) -> Vec<RsId> {
        self.rs.exec(&self.fu, self.cycle)
    }

    fn write(&mut self, comp: Vec<RsId>) {
        for rs_id in comp {
            if let Some(rs) = self.rs.get_mut(rs_id) {
                if let Some(Unit::Fu(fu_id)) = rs.dest() {
                    let value = rs.result().unwrap();
                    self.fu.mark_ready(*fu_id, value);
                    let mut inst = rs.take().unwrap();
                    inst.write(self.cycle);
                    self.insts_comp.push(inst);
                    rs.clear();
                }
            }
        }
    }
}

impl std::fmt::Debug for Executer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let finished = if self.finished {
            style("> Finished").green().bold()
        } else {
            style("> Running").red().bold()
        };

        writeln!(
            f,
            "{} {} {}",
            style("Cycle:").yellow().bold(),
            style(self.cycle).cyan().bold(),
            finished
        )?;
        writeln!(f, "{}", style("Reservation Stations:").yellow().bold())?;
        writeln!(f, "{:?}", self.rs)?;
        writeln!(f, "{}", style("Floating Units:").yellow().bold())?;
        writeln!(f, "{:?}", self.fu)?;
        Ok(())
    }
}
