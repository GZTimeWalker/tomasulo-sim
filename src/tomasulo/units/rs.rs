use std::collections::BTreeMap;

use console::style;

use crate::tomasulo::*;

const ADD_RS_COUNT: usize = 3;
const MULT_RS_COUNT: usize = 2;
const LOAD_BUFFER_COUNT: usize = 3;
const STORE_BUFFER_COUNT: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RsId(RsType, u8);

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RsType {
    ADD,
    MULT,
    LOAD,
    STORE,
}

#[derive(Debug, Eq, PartialEq)]
pub enum RsState {
    Busy,
    Free,
    Calculating,
    Ready,
}

#[derive(Debug)]
pub struct RsInner {
    pub id: RsId,
    pub state: RsState,
    inst: Option<Instruction>,

    vj: Option<Value>,
    vk: Option<Value>,
    qj: Option<RsId>,
    qk: Option<RsId>,

    pub addr: Option<Value>,
    pub result: Option<Value>,
}

pub struct ReservationStation {
    inner: BTreeMap<RsId, RsInner>,
}

impl ReservationStation {
    pub fn new() -> ReservationStation {
        let mut inner = BTreeMap::new();
        for i in 0..ADD_RS_COUNT {
            inner.insert(
                RsId(RsType::ADD, i as u8),
                RsInner::new(RsType::ADD, i as u8),
            );
        }
        for i in 0..MULT_RS_COUNT {
            inner.insert(
                RsId(RsType::MULT, i as u8),
                RsInner::new(RsType::MULT, i as u8),
            );
        }
        for i in 0..LOAD_BUFFER_COUNT {
            inner.insert(
                RsId(RsType::LOAD, i as u8),
                RsInner::new(RsType::LOAD, i as u8),
            );
        }
        for i in 0..STORE_BUFFER_COUNT {
            inner.insert(
                RsId(RsType::STORE, i as u8),
                RsInner::new(RsType::STORE, i as u8),
            );
        }
        ReservationStation { inner }
    }

    pub fn get_free(&self, rs_type: RsType) -> Option<RsId> {
        for inner in self.inner.values() {
            if inner.id.0 == rs_type && inner.state == RsState::Free {
                return Some(inner.id);
            }
        }
        None
    }

    #[inline]
    pub fn get_mut(&mut self, id: RsId) -> Option<&mut RsInner> {
        self.inner.get_mut(&id)
    }

    pub fn clear(&mut self) {
        for inner in self.inner.values_mut() {
            inner.clear();
        }
    }

    pub fn exec(&mut self, fu: &FloatingUnit, cycle: u64) -> Vec<RsId> {
        let mut ready = Vec::new();
        for inner in self.inner.values_mut() {
            if inner.state == RsState::Busy {
                if inner.is_ready() {
                    inner.state = RsState::Calculating;
                } else {
                    if let Some(qj) = inner.qj {
                        if let Some(value) = fu.try_get_value(qj) {
                            inner.vj.replace(value);
                            inner.qj = None;
                        }
                    }

                    if let Some(qk) = inner.qk {
                        if let Some(value) = fu.try_get_value(qk) {
                            inner.vk.replace(value);
                            inner.qk = None;
                        }
                    }
                }
            } else if inner.state == RsState::Calculating {
                match inner.exec(cycle) {
                    RsState::Ready => {
                        ready.push(inner.id);
                        inner.state = RsState::Ready;
                    }
                    _ => {}
                }
            }
        }
        ready
    }
}

impl RsInner {
    pub fn new(rs_type: RsType, id: u8) -> RsInner {
        RsInner {
            id: RsId(rs_type, id),
            inst: None,
            state: RsState::Free,
            vj: None,
            vk: None,
            qj: None,
            qk: None,
            addr: None,
            result: None,
        }
    }

    pub fn apply(&mut self, mut inst: Instruction, fu: &FloatingUnit, cycle: u64) {
        inst.emit(cycle);

        match inst.op {
            Type::LD | Type::SD => {
                // assume that the address is always ready
                self.addr.replace(inst.src1.clone().unwrap());
                self.vk.replace(inst.src2.clone().unwrap());
            }
            _ => {
                // use fu to calculate the value
                if let Some(src1) = inst.src1.clone() {
                    match *src1 {
                        ValueInner::Unit(Unit::Fu(fuid)) => {
                            let fu = fu.get(fuid);
                            println!(
                                "{} {}",
                                style("fu").bold().dim(),
                                style(format!("{:?}", fu)).bold().dim()
                            );
                            match &fu.value {
                                Some(value) => {
                                    self.vj.replace(value.clone());
                                }
                                None => {
                                    self.qj = fu.qi;
                                }
                            }
                        }
                        _ => {
                            panic!("src1 is not a fu: {:?}", src1)
                        }
                    }
                }

                if let Some(src2) = inst.src2.clone() {
                    match *src2 {
                        ValueInner::Unit(Unit::Fu(fuid)) => {
                            let fu = fu.get(fuid);
                            println!(
                                "{} {}",
                                style("fu").bold().dim(),
                                style(format!("{:?}", fu)).bold().dim()
                            );
                            match &fu.value {
                                Some(value) => {
                                    self.vk.replace(value.clone());
                                }
                                None => {
                                    self.qk = fu.qi;
                                }
                            }
                        }
                        _ => {
                            panic!("src2 is not a fu: {:?}", src2)
                        }
                    }
                }
            }
        }

        self.inst.replace(inst);
        self.state = RsState::Busy;
    }

    pub fn clear(&mut self) {
        self.inst = None;
        self.state = RsState::Free;
        self.vj = None;
        self.vk = None;
        self.qj = None;
        self.qk = None;
        self.addr = None;
    }

    pub fn is_ready(&self) -> bool {
        match self.inst.as_ref().unwrap().op {
            Type::LD | Type::SD => self.vk.is_some(),
            _ => self.vj.is_some() && self.vk.is_some(),
        }
    }

    #[inline]
    pub fn dest(&self) -> Option<&Unit> {
        self.inst.as_ref().map(|inst| &inst.dest)
    }

    #[inline]
    pub fn result(&self) -> Option<Value> {
        self.result.clone()
    }

    pub fn exec(&mut self, cycle: u64) -> RsState {
        if let Some(inst) = self.inst.as_mut() {
            let op = inst.op;
            if inst.exec(cycle) {
                self.result.replace(match op {
                    Type::LD | Type::SD => {
                        let addr = self.addr.as_ref().unwrap();
                        let vk = self.vk.as_ref().unwrap();
                        let addr = value::apply_op(Type::ADDD, addr.clone(), vk.clone());
                        value::new(ValueInner::MemAddr(addr))
                    }
                    _ => {
                        let lhs = self.vj.as_ref().unwrap();
                        let rhs = self.vk.as_ref().unwrap();
                        value::apply_op(op, lhs.clone(), rhs.clone())
                    }
                });
                RsState::Ready
            } else {
                RsState::Calculating
            }
        } else {
            RsState::Free
        }
    }

    pub fn take(&mut self) -> Option<Instruction> {
        self.inst.take()
    }
}

impl From<Type> for RsType {
    fn from(t: Type) -> RsType {
        match t {
            Type::ADDD | Type::SUBD => RsType::ADD,
            Type::MULTD | Type::DIVD => RsType::MULT,
            Type::LD => RsType::LOAD,
            Type::SD => RsType::STORE,
        }
    }
}

impl std::fmt::Debug for ReservationStation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for inner in self.inner.values() {
            writeln!(f, "{}", inner)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for RsInner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let op = match self.inst.as_ref() {
            Some(inst) => format!("{}", inst.op),
            None => String::from("None "),
        };
        let vj = match self.vj.as_ref() {
            Some(v) => style(v.brief()).cyan().bold(),
            None => style(String::from("None ")).white(),
        };
        let vk = match self.vk.as_ref() {
            Some(v) => style(v.brief()).cyan().bold(),
            None => style(String::from("None ")).white(),
        };
        let qj = match self.qj.as_ref() {
            Some(v) => format!("{}", v),
            None => String::from("None  "),
        };
        let qk = match self.qk.as_ref() {
            Some(v) => format!("{}", v),
            None => String::from("None  "),
        };
        let addr = match self.addr.as_ref() {
            Some(v) => style(format!("{}", v)).blue(),
            None => style(String::from("None ")).white(),
        };

        write!(
            f,
            "{} : {},{},{:<5},{:<5},{},{},{}",
            self.id, self.state, op, vj, vk, qj, qk, addr
        )
    }
}

impl std::fmt::Display for RsId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:<6}",
            style(format!("{:?}{}", self.0, self.1)).green().bold()
        )
    }
}

impl std::fmt::Display for RsState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RsState::Free => write!(f, "{:<6}", style("Free").yellow().bold()),
            RsState::Busy => write!(f, "{:<6}", style("Busy").red().bold()),
            RsState::Calculating => write!(f, "{:<6}", style("Calc").blue().bold()),
            RsState::Ready => write!(f, "{:<6}", style("Ready").green().bold()),
        }
    }
}
