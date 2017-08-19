extern crate quick_protobuf;

use quick_protobuf::errors::Error as ProtobufError;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum ConnectionErr {
    NameTooLong,
    TcpConnectionFailure(IoError),
    HelloMessageFailed(IoError),
    NameMessageFailed(IoError),
    IdMessageFailed(IoError),
    IdMessageTooShort,
    OkMessageFailed(IoError),
    OkMessageWrong,
    RWCreationError(ProtobufError)
}
#[derive(Debug)]
pub enum TransceiverError {
    Protobuf(ProtobufError),
    Socket(IoError),
    ResponseHasError(String)
}
impl From<ProtobufError> for TransceiverError { fn from(i : ProtobufError) -> Self { TransceiverError::Protobuf(i) } }
impl From<IoError> for TransceiverError { fn from(i : IoError) -> Self { TransceiverError::Socket(i) } }

pub type Id = [u8; 16];
#[macro_use]
mod macros;
pub mod schema;
pub mod rpc;
pub mod stream;
