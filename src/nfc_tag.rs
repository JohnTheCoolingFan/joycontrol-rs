use log::{info, warn};

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
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NFCTagType {
    Amiibo,
}
