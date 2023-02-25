use log::{info, warn};
use std::error::Error;
use tokio::{fs::File, io::AsyncReadExt};

// TODO: other method impls
#[derive(Debug, Clone)]
pub struct NFCTag {
    data: Vec<u8>,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NFCTagType {
    Amiibo,
}
