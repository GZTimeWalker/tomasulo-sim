pub mod fu;
pub mod regs;
pub mod rs;

use std::str::FromStr;

pub use fu::*;
pub use regs::*;
pub use rs::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Unit {
    /// Reservation station unit
    Rs(RsId),
    /// Floating point unit
    Fu(FuId),
    /// Register unit
    Regs(RegId),
}

impl FromStr for Unit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('R') => {
                if let Ok(id) = s[1..].parse::<u8>() {
                    Ok(Unit::Regs(RegId::new(id)))
                } else {
                    Err(())
                }
            }
            Some('F') => {
                if let Ok(id) = s[1..].parse::<u8>() {
                    Ok(Unit::Fu(FuId::new(id)))
                } else {
                    Err(())
                }
            }
            _ => Err(()),
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
            Unit::Rs(id) => write!(f, "{id}"),
            Unit::Fu(id) => write!(f, "{id}"),
            Unit::Regs(id) => write!(f, "{id}"),
        }
    }
}
