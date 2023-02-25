use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct StickCalibration {
    pub h_center: u32,
    pub v_center: u32,
    pub h_max_above_center: u32,
    pub v_max_above_center: u32,
    pub h_max_below_center: u32,
    pub v_max_below_center: u32,
}

impl Display for StickCalibration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "h_center:{} v_center:{} h_max_above_center:{} v_max_above_center:{} h_max_below_center:{} v_max_below_center:{}", self.h_center, self.v_center, self.h_max_above_center, self.v_max_above_center, self.h_max_below_center, self.v_max_below_center)
    }
}

impl StickCalibration {
    pub fn l_from_bytes(bytes: &[u8; 9]) -> Self {
        let h_max_above_center: u32 = ((bytes[1] as u32) << 8) & 0xF00 | (bytes[0] as u32);
        let v_max_above_center: u32 = ((bytes[2] as u32) << 4) | ((bytes[1] as u32) >> 4);
        let h_center: u32 = ((bytes[4] as u32) << 8) & 0xF00 | (bytes[3] as u32);
        let v_center: u32 = ((bytes[5] as u32) << 4) | ((bytes[4] as u32) >> 4);
        let h_max_below_center: u32 = ((bytes[7] as u32) << 8) & 0xF00 | bytes[6] as u32;
        let v_max_below_center: u32 = ((bytes[8] as u32) << 4) | ((bytes[7] as u32) >> 4);

        Self {
            h_center,
            v_center,
            h_max_above_center,
            v_max_above_center,
            h_max_below_center,
            v_max_below_center,
        }
    }

    pub fn r_from_bytes(bytes: &[u8; 9]) -> Self {
        let h_center: u32 = ((bytes[1] as u32) << 8) & 0xF00 | (bytes[0] as u32);
        let v_center: u32 = ((bytes[2] as u32) << 4) | ((bytes[1] as u32) >> 4);
        let h_max_below_center: u32 = ((bytes[4] as u32) << 8) & 0xF00 | (bytes[3] as u32);
        let v_max_below_center: u32 = ((bytes[5] as u32) << 4) | ((bytes[4] as u32) >> 4);
        let h_max_above_center: u32 = ((bytes[7] as u32) << 8) & 0xF00 | bytes[6] as u32;
        let v_max_above_center: u32 = ((bytes[8] as u32) << 4) | ((bytes[7] as u32) >> 4);

        Self {
            h_center,
            v_center,
            h_max_above_center,
            v_max_above_center,
            h_max_below_center,
            v_max_below_center,
        }
    }
}
