use bytes::Bytes;
use log::{error, info, warn};
use std::{
    error::Error,
    fs,
    io::{self, Write},
};
use tokio::{fs::File, io::AsyncReadExt};

// TODO: other method impls
#[derive(Debug, Clone)]
pub struct NFCTag {
    pub data: Vec<u8>,
    tag_type: NFCTagType,
    source: Option<String>,
}

impl NFCTag {
    pub fn new(data: &[u8], tag_type: Option<NFCTagType>, source: Option<String>) -> Self {
        let tag_type = tag_type.unwrap_or(NFCTagType::Amiibo);
        if matches!(tag_type, NFCTagType::Amiibo) {
            if data.len() == 572 {
                info!("Long amiibo loaded, manufacturer signature is ignored")
            } else if data.len() != 540 {
                warn!("Illegal Amiibo tag size")
            }
        }
        Self {
            data: data.into(),
            tag_type,
            source,
        }
    }

    pub async fn load_amiibo(source: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let mut reader = File::open(source).await?;
        let mut buf = vec![];
        reader.read_to_end(&mut buf).await;
        Ok(Self::new(
            &buf,
            Some(NFCTagType::Amiibo),
            Some(source.into()),
        ))
    }

    pub fn get_uid(&self) -> [u8; 6] {
        [&self.data[0..3], &self.data[4..8]]
            .concat()
            .try_into()
            .unwrap()
    }

    pub fn write(&mut self, idx: usize, data: &[u8]) {
        if idx > self.data.len() || idx + data.len() > self.data.len() {
            error!(
                "Some index error {}, {:x} {}",
                idx,
                Bytes::copy_from_slice(data),
                data.len()
            );
        }
        self.data[idx..(idx + data.len())].copy_from_slice(data);
    }

    pub fn save(&mut self) -> Result<(), io::Error> {
        if let Some(source) = &self.source {
            let mut writer = fs::File::open(source)?;
            writer.write_all(&self.data)?;
            info!("Saved altered amiibo as {}", source);
        } else {
            warn!("No save path provided, ignoring save call");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NFCTagType {
    Amiibo,
}
