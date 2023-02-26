use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Controller {
    JoyconL = 0x01,
    JoyconR = 0x02,
    ProController = 0x03,
}

impl Display for Controller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::JoyconL => "Joy-Con (L)",
                Self::JoyconR => "Joy-Con (R)",
                Self::ProController => "Pro Controller",
            }
        )
    }
}

impl FromStr for Controller {
    type Err = UnknownController;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JOYCON_R" => Ok(Self::JoyconR),
            "JOYCON_L" => Ok(Self::JoyconL),
            "PRO_CONTROLLER" => Ok(Self::ProController),
            _ => Err(UnknownController(s.into())),
        }
    }
}

#[derive(Debug, Clone, Error)]
pub struct UnknownController(String);

impl Display for UnknownController {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown controller: {}", self.0)
    }
}
