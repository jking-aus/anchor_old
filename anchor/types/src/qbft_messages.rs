use std::convert::TryInto;
use std::error::Error;
use std::fmt;

// VaLidatorPK is an eth32 validator public key 48 bytes long
pub struct ValdiatorPK {
    pub key: [u8; 48],
}

// ShareValidatorPK is a partial eth32 validator public key 48 bytes long
pub struct ShareValidatorPK {
    pub key: [u8; 48],
}

// omitting validation functions for now

pub struct MessageID([u8; 56]);

#[derive(Debug, Clone)]
pub struct SSVMessage<D> {
    msg_type: MsgType,
    msg_id: MessageID,
    data: D,
}
