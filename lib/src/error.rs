use thiserror::Error;

#[derive(Error, Debug)]
pub enum BtcError {
    #[error("Invalid transaction")]
    InvalidTransaction,
    #[error("Invalid Block")]
    InvalidBlock,
    #[error("Invalid BlockHeader,")]
    InvalidBlockHeader,
    #[error("Invalid TransactionInput,")]
    InvalidTransactionInput,
    #[error("Invalid TransactionOutput")]
    InvalidTransactionOutput,
    #[error("Invalid MerkleRoot,")]
    InvalidMerkleRoot,
    #[error("Invalid Hash")]
    InvalidHash,
    #[error("Invalid Signature")]
    InvalidSignature,
    #[error("Invalid Public Key")]
    InvalidPublicKey,
    #[error("Invalid Private Key")]
    InvalidPrivateKey,
}

pub type Result<T> = std::result::Result<T, BtcError>;
