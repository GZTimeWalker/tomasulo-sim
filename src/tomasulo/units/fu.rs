use console::style;

use crate::tomasulo::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuId(u8);

impl FuId {
    pub fn new(id: u8) -> FuId {
        assert!(id % 2 == 0 && id < 2 * FU_SIZE as u8);
        FuId(id)
    }
}

pub const FU_SIZE: usize = 16;

#[derive(Clone, PartialEq)]
pub struct FloatingUnit {
    pub inner: [FloatingUnitInner; FU_SIZE],
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FloatingUnitInner {
    pub qi: Option<RsId>,
    pub value: Option<Value>,
}

impl FloatingUnit {
    pub fn new() -> FloatingUnit {
        FloatingUnit {
            inner: (0..FU_SIZE)
                .map(|v| FloatingUnitInner::default().with_value(2f64 * v as f64))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    /// Mark the floating unit as busy.
    pub fn mark_busy(&mut self, id: FuId, qi: RsId) {
        let fu = &mut self.inner[id.0 as usize / 2];
        fu.qi = Some(qi);
        fu.value = None;
    }

    /// Mark the floating unit as ready.
    pub fn mark_ready(&mut self, id: FuId, qi: RsId, value: Value) {
        let fu = &mut self.inner[id.0 as usize / 2];
        if fu.qi == Some(qi) {
            fu.value = Some(value);
        }
    }

    pub fn get(&self, id: FuId) -> &FloatingUnitInner {
        &self.inner[id.0 as usize / 2]
    }

    pub fn clear(&mut self) {
        for fu in self.inner.iter_mut() {
            fu.qi.take();
        }
    }
}

impl FloatingUnitInner {
    pub fn with_value(mut self, val: f64) -> Self {
        self.value.replace(value::new(ValueInner::Float(val)));
        self
    }
}

impl std::fmt::Debug for FloatingUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, fu) in self.inner.iter().enumerate() {
            if fu.qi.is_none() {
                continue;
            }

            let qi = match fu.qi {
                Some(qi) => format!("{qi}"),
                None => "None  ".to_string(),
            };
            let value = match &fu.value {
                Some(value) => style(format!("{value}")).cyan().underlined(),
                None => style("None".to_string()).white(),
            };
            let fuid = FuId::new(i as u8 * 2);
            writeln!(
                f,
                "{} : {} -> {}",
                style(fuid).magenta().underlined(),
                qi,
                value
            )?;
        }
        Ok(())
    }
}

impl std::fmt::Display for FuId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "F{:02}", self.0)
    }
}
