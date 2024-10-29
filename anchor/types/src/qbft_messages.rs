use crate::qbft_types::{OperatorId, Round};

// VaLidatorPK is an eth32 validator public key 48 bytes long
pub struct ValdiatorPK {
    pub key: [u8; 48],
}

// ShareValidatorPK is a partial eth32 validator public key 48 bytes long
pub struct ShareValidatorPK {
    pub key: [u8; 48],
}

// omitting validation functions for now
#[derive(Debug, Clone)]
pub struct MessageID([u8; 56]);

#[derive(Debug, Clone)]
pub enum MsgType {
    /// QBFT related consensus messages
    SSVConsensusMsgType,
    /// Partial signature messages
    SSVPartialSignatureMsgType,
}

// SSV message is the main message passed within the SSV network
#[derive(Debug, Clone)]
pub struct SSVMessage<D> {
    msg_type: MsgType,
    msg_id: MessageID,
    data: D,
}
// SignedSSVMessage is a signed message passed within the SSV network
pub struct SignedSSVMessage {
    signatures: usize,
    operator_ids: Vec<OperatorId>,
}
