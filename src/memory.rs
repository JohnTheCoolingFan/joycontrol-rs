use std::{fmt::Display, ops::Deref};

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct FlashMemory {
    pub data: Vec<u8>,
}

impl FlashMemory {
    pub fn new(
        spi_flash_memory_data: Option<&[u8]>,
        default_stick_cal: Option<bool>,
        size: Option<usize>,
    ) -> Result<Self, SizeMismatch> {
        let size = size.unwrap_or(0x80000);
        let (mut spi_flash_memory_data, default_stick_cal) =
            if let Some(data) = spi_flash_memory_data {
                (data.into(), default_stick_cal.unwrap_or(false))
            } else {
                (vec![0xFF; size], true)
            };

        if spi_flash_memory_data.len() != size {
            return Err(SizeMismatch(spi_flash_memory_data.len(), size));
        }
        if default_stick_cal {
            spi_flash_memory_data[0x603D..0x6046]
                .copy_from_slice([0x00, 0x07, 0x70, 0x00, 0x08, 0x80, 0x00, 0x07, 0x70].as_slice());
            spi_flash_memory_data[0x6046..0x604F]
                .copy_from_slice([0x00, 0x08, 0x80, 0x00, 0x07, 0x70, 0x00, 0x07, 0x70].as_slice());
        }

        Ok(Self {
            data: spi_flash_memory_data,
        })
    }

    pub fn get_factory_l_stick_calibration(&self) -> &[u8] {
        &self.data[0x603D..0x6046]
    }

    pub fn get_factory_r_stick_calibration(&self) -> &[u8] {
        &self.data[0x6046..0x604F]
    }

    pub fn get_user_l_stick_calibration(&self) -> Option<&[u8]> {
        if self.data[0x8010..=0x8011] == [0xB2, 0xA1] {
            Some(&self.data[0x8012..0x801B])
        } else {
            None
        }
    }

    pub fn get_user_r_stick_calibration(&self) -> Option<&[u8]> {
        if self.data[0x801B..=0x801C] == [0xB2, 0xA1] {
            Some(&self.data[0x801D..0x8026])
        } else {
            None
        }
    }
}

impl Deref for FlashMemory {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Debug, Clone, Error)]
pub struct SizeMismatch(usize, usize);

impl Display for SizeMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Given data size {} does not match size {}",
            self.0, self.1
        )
    }
}
