use crate::tomasulo::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RsId(RsType, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RsType {
    ADD,
    MULT,
    LOAD,
    STORE,
}

pub struct ReservationStation {
    pub id: RsId,
    pub busy: bool,
    pub inst: Instruction,

    pub vj: Option<Value>,
    pub vk: Option<Value>,
    pub qj: Option<RsId>,
    pub qk: Option<RsId>,

    pub addr: Option<Value>,
}

impl ReservationStation {
    pub fn new(id: RsId, inst: Instruction) -> ReservationStation {
        ReservationStation {
            id,
            inst,
            busy: false,
            vj: None,
            vk: None,
            qj: None,
            qk: None,
            addr: None,
        }
    }
}

impl std::fmt::Display for RsId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}{}", self.0, self.1)
    }
}
