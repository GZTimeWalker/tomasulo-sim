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

    /// Add instructions to the executer.
    #[inline]
    pub fn add_insts(&mut self, inst: &[Instruction]) {
        self.insts.extend(inst.iter().cloned());
        self.inst_count = inst.len();
    }

    /// Clear the executer.
    pub fn clear(&mut self) {
        self.rs.clear();
        self.fu.clear();
        self.insts.clear();
        self.cycle = 0;
        self.finished = false;
    }

    /// Run the simulation.
    pub fn run(&mut self) {
        while !self.finished {
            println!("{:=^60}", style("=").bold());
            self.cycle += 1;

            // issue new instructions
            self.issue();
            // execute instructions
            let comp = self.exec();
            // write back the result
            self.write(&comp);

            // check if all instructions are completed
            self.finished = self.insts_comp.len() == self.inst_count;

            // print the current state
            print!("{self:?}");
            self.print_insts();
            println!("{:=^60}\n", style("=").bold());

            // clear the reservation station which has completed instructions
            self.clear_rs(&comp);

            // avoid infinite loop
            if self.cycle > 1000 {
                panic!("Cycle limit exceeded. (1000 cycles)");
            }
        }
    }

    /// Print the completed instructions.
    fn print_insts(&mut self) {
        println!("{}", style("Instructions:").yellow().bold());
        self.insts_comp.sort_by_key(|i| i.emit_cycle.unwrap_or(0));
        for inst in self.insts_comp.iter_mut() {
            println!("{inst}");
        }
    }

    /// Issue instructions to the reservation station.
    fn issue(&mut self) {
        if let Some(inst) = self.insts.pop_front() {
            if let Some(rs_id) = self.rs.get_free(inst.op.into()) {
                if let Some(rs) = self.rs.get_mut(rs_id) {
                    match inst.op {
                        Type::SD => {
                            // we do not need to mark the FU as busy when storing
                            rs.apply(inst, &self.fu, self.cycle);
                        }
                        _ => {
                            if let Unit::Fu(id) = inst.dest {
                                self.fu.mark_busy(id, rs_id);
                                rs.apply(inst, &self.fu, self.cycle);
                            } else {
                                panic!("Destination of instruction is not a register.")
                            }
                        }
                    }
                    // only issue one instruction per cycle
                    return;
                }
            }
            self.insts.push_front(inst);
        }
    }

    /// Execute instructions in the reservation station.
    #[inline]
    fn exec(&mut self) -> Vec<RsId> {
        self.rs.exec(self.cycle)
    }

    /// Write the result back from the reservation station to the FU.
    ///
    /// This will also broadcast the result to the other reservation stations.
    fn write(&mut self, comp: &Vec<RsId>) {
        let mut boardcast = Vec::new();
        for rs_id in comp {
            if let Some(rs) = self.rs.get_mut(*rs_id) {
                if let Some(Unit::Fu(fu_id)) = rs.dest() {
                    let value = rs.result().unwrap();
                    boardcast.push((*rs_id, value.clone()));
                    self.fu.mark_ready(*fu_id, value);
                    let mut inst = rs.take().unwrap();
                    inst.write(self.cycle);
                    self.insts_comp.push(inst);
                }
            }
        }
        for (rs_id, value) in boardcast {
            self.rs.flush(rs_id, value);
        }
    }

    /// Clear the reservation station.
    ///
    /// Clear the reservation station after print it to the screen
    /// thus we can see the result of the previous cycle.
    fn clear_rs(&mut self, comp: &Vec<RsId>) {
        for rs_id in comp {
            if let Some(rs) = self.rs.get_mut(*rs_id) {
                rs.clear();
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
