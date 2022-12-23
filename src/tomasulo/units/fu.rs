use crate::tomasulo::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuId(u8);

impl FuId {
    pub fn new(id: u8) -> FuId {
        assert!(id % 2 == 0 && id < FU_SIZE as u8);
        FuId(id)
    }
}

pub const FU_SIZE: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FloatingUnit {
    pub inner: [FloatingUnitInner; FU_SIZE],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Default)]
pub struct FloatingUnitInner {
    pub qi: Option<RsId>,
}



impl FloatingUnit {
    pub fn new() -> FloatingUnit {
        FloatingUnit {
            inner: [FloatingUnitInner::default(); FU_SIZE],
        }
    }

    pub fn get(&self, id: FuId) -> &FloatingUnitInner {
        &self.inner[id.0 as usize / 2]
    }

    pub fn get_mut(&mut self, id: FuId) -> &mut FloatingUnitInner {
        &mut self.inner[id.0 as usize / 2]
    }
}

impl std::fmt::Display for FuId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "F{}", self.0)
    }
}
