use std::fmt::Display;

use strum::EnumString;
use thiserror::Error;

use crate::stick_calibration::StickCalibration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum StickDirection {
    Center,
    Up,
    Down,
    Left,
    Right,
    #[strum(to_string = "v")]
    Vertical,
    #[strum(to_string = "h")]
    Horizontal,
}

#[derive(Debug, Clone)]
pub struct StickState {
    h_stick: u32,
    v_stick: u32,
    calibration: Option<StickCalibration>,
}

impl StickState {
    pub fn new(
        h: Option<u32>,
        v: Option<u32>,
        calibration: Option<StickCalibration>,
    ) -> Result<Self, InvalidStickValue> {
        let h = h.unwrap_or(0);
        let v = v.unwrap_or(0);

        if h >= 0x1000 || v >= 0x1000 {
            Err(InvalidStickValue)
        } else {
            Ok(Self {
                h_stick: h,
                v_stick: v,
                calibration,
            })
        }
    }

    pub fn set_h(&mut self, val: u32) -> Result<(), InvalidStickValue> {
        if val >= 0x1000 {
            Err(InvalidStickValue)
        } else {
            self.h_stick = val;
            Ok(())
        }
    }

    #[inline]
    pub fn get_h(&self) -> u32 {
        self.h_stick
    }

    pub fn set_v(&mut self, val: u32) -> Result<(), InvalidStickValue> {
        if val >= 0x1000 {
            Err(InvalidStickValue)
        } else {
            self.v_stick = val;
            Ok(())
        }
    }

    #[inline]
    pub fn get_v(&self) -> u32 {
        self.v_stick
    }

    pub fn set_center(&mut self) -> Result<(), NoCalibrationDataAvailable> {
        if let Some(calib_data) = &self.calibration {
            self.h_stick = calib_data.h_center;
            self.v_stick = calib_data.v_center;
            Ok(())
        } else {
            Err(NoCalibrationDataAvailable)
        }
    }

    pub fn is_center(&self, radius: Option<u32>) -> Option<bool> {
        let calibration = self.calibration.as_ref()?;
        let radius = radius.unwrap_or(0);

        let h_is_center = ((calibration.h_center - radius)..=(calibration.h_center + radius))
            .contains(&self.h_stick);
        let v_is_center = ((calibration.v_center - radius)..=(calibration.v_center + radius))
            .contains(&self.v_stick);
        Some(h_is_center && v_is_center)
    }

    pub fn set_up(&mut self) -> Result<(), NoCalibrationDataAvailable> {
        if let Some(calib_data) = &self.calibration {
            self.h_stick = calib_data.h_center;
            self.v_stick = calib_data.v_center + calib_data.v_max_above_center;
            Ok(())
        } else {
            Err(NoCalibrationDataAvailable)
        }
    }

    pub fn set_down(&mut self) -> Result<(), NoCalibrationDataAvailable> {
        if let Some(calib_data) = &self.calibration {
            self.h_stick = calib_data.h_center;
            self.v_stick = calib_data.v_center - calib_data.v_max_below_center;
            Ok(())
        } else {
            Err(NoCalibrationDataAvailable)
        }
    }

    pub fn set_left(&mut self) -> Result<(), NoCalibrationDataAvailable> {
        if let Some(calib_data) = &self.calibration {
            self.h_stick = calib_data.h_center - calib_data.h_max_below_center;
            self.v_stick = calib_data.v_center;
            Ok(())
        } else {
            Err(NoCalibrationDataAvailable)
        }
    }

    pub fn set_right(&mut self) -> Result<(), NoCalibrationDataAvailable> {
        if let Some(calib_data) = &self.calibration {
            self.h_stick = calib_data.h_center - calib_data.h_max_above_center;
            self.v_stick = calib_data.v_center;
            Ok(())
        } else {
            Err(NoCalibrationDataAvailable)
        }
    }

    #[inline]
    pub fn set_calibration(&mut self, calibration: StickCalibration) {
        self.calibration = Some(calibration)
    }

    #[inline]
    pub fn get_calibration(&self) -> Result<&StickCalibration, NoCalibrationDataAvailable> {
        self.calibration.as_ref().ok_or(NoCalibrationDataAvailable)
    }

    #[inline]
    pub fn as_bytes(&self) -> [u8; 3] {
        let byte_0 = self.h_stick as u8;
        let byte_1 = ((self.v_stick as u8) & 0xF) << 4;
        let byte_2 = (self.v_stick as u8) >> 4;
        [byte_0, byte_1, byte_2]
    }
}

impl From<&[u8; 3]> for StickState {
    fn from(value: &[u8; 3]) -> Self {
        let h_stick = (value[0] as u32) | (((value[1] as u32) & 0xF) << 8);
        let v_stick = ((value[1] as u32) >> 4) | ((value[2] as u32) << 4);

        Self {
            h_stick,
            v_stick,
            calibration: None,
        }
    }
}

#[derive(Debug, Clone, Error)]
pub struct InvalidStickValue;

impl Display for InvalidStickValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stick values must be in [0,{}}}", 0x1000)
    }
}

#[derive(Debug, Clone, Error)]
pub struct NoCalibrationDataAvailable;

impl Display for NoCalibrationDataAvailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No calibration data available")
    }
}
