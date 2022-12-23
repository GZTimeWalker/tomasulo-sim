
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegId(u8);

impl RegId {
    pub fn new(id: u8) -> RegId {
        RegId(id)
    }
}

impl std::fmt::Display for RegId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "R{}", self.0)
    }
}
