use std::collections::HashMap;

use crate::tomasulo::*;

const ADD_RS_COUNT: usize = 3;
const MULT_RS_COUNT: usize = 2;
const LOAD_BUFFER_COUNT: usize = 3;
const STORE_BUFFER_COUNT: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RsId(RsType, u8);

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RsType {
    ADD,
    MULT,
    LOAD,
    STORE,
}

pub struct RsInner {
    pub id: RsId,
    pub busy: bool,
    pub inst: Option<Instruction>,

    pub vj: Option<Value>,
    pub vk: Option<Value>,
    pub qj: Option<RsId>,
    pub qk: Option<RsId>,

    pub addr: Option<Value>,
}

pub struct ReservationStation {
    inner: HashMap<RsId, RsInner>,
}

impl ReservationStation {
    pub fn new() -> ReservationStation {
        let mut inner = HashMap::new();
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
            if inner.id.0 == rs_type && !inner.busy {
                return Some(inner.id);
            }
        }
        None
    }

    pub fn get(&self, id: RsId) -> &RsInner {
        self.inner.get(&id).unwrap()
    }
}

impl RsInner {
    pub fn new(rs_type: RsType, id: u8) -> RsInner {
        RsInner {
            id: RsId(rs_type, id),
            inst: None,
            busy: false,
            vj: None,
            vk: None,
            qj: None,
            qk: None,
            addr: None,
        }
    }

    pub fn apply(&mut self, inst: Instruction) {
        self.inst = Some(inst);
        self.busy = true;
    }

    pub fn clear(&mut self) {
        self.inst = None;
        self.busy = false;
        self.vj = None;
        self.vk = None;
        self.qj = None;
        self.qk = None;
        self.addr = None;
    }

    pub fn is_ready(&self) -> bool {
        self.vj.is_some() && self.vk.is_some()
    }

    pub fn exec(&mut self, cycle: u64) -> Option<Value> {
        if let Some(inst) = self.inst.as_mut() {
            if inst.exec(cycle) {
                match inst.op {
                    Type::LD => {
                        let addr = self.addr.as_ref().unwrap();
                        Some(addr.clone())
                    }
                    Type::SD => {
                        let addr = self.addr.as_ref().unwrap();
                        Some(addr.clone())
                    }
                    _ => {
                        let lhs = self.vj.as_ref().unwrap();
                        let rhs = self.vk.as_ref().unwrap();
                        Some(value::apply_op(inst.op, lhs.clone(), rhs.clone()))
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
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

impl std::fmt::Display for RsId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}{}", self.0, self.1)
    }
}
