use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct StickState {
    h_stick: u32,
    v_stick: u32,
}

impl StickState {
    pub fn new(h: Option<u32>, v: Option<u32>) -> Result<Self, InvalidStickValues> {
        let h = h.unwrap_or(0);
        let v = v.unwrap_or(0);

        if h >= 0x1000 {
            return Err(InvalidStickValues);
        }

        if v >= 0x1000 {
            return Err(InvalidStickValues);
        }

        Ok(Self {
            h_stick: h,
            v_stick: v,
        })
    }
}

#[derive(Debug, Clone, Error)]
pub struct InvalidStickValues;

impl Display for InvalidStickValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stick values must be in [0,{}}}", 0x1000)
    }
}
