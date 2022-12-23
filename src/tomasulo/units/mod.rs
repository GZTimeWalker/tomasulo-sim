pub mod rs;
pub mod fu;
pub mod regs;

pub use rs::*;
pub use fu::*;
pub use regs::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Unit {
    /// Reservation station unit
    Rs(RsId),
    /// Floating point unit
    Fu(FuId),
    /// Register unit
    Regs(RegId),
}

impl Unit {
    pub fn parse(s: &str) -> Option<Unit> {
        match s.chars().next() {
            Some('R') => {
                if let Ok(id) = s[1..].parse::<u8>() {
                    Some(Unit::Regs(RegId::new(id)))
                } else {
                    None
                }
            }
            Some('F') => {
                if let Ok(id) = s[1..].parse::<u8>() {
                    Some(Unit::Fu(FuId::new(id)))
                } else {
                    None
                }
            }
            _ => None
        }
    }
}

impl From<RsId> for Unit {
    fn from(id: RsId) -> Unit {
        Unit::Rs(id)
    }
}

impl From<FuId> for Unit {
    fn from(id: FuId) -> Unit {
        Unit::Fu(id)
    }
}

impl From<RegId> for Unit {
    fn from(id: RegId) -> Unit {
        Unit::Regs(id)
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Unit::Rs(id) => write!(f, "{}", id),
            Unit::Fu(id) => write!(f, "{}", id),
            Unit::Regs(id) => write!(f, "{}", id),
        }
    }
}
