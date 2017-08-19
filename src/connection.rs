use super::*;

use std::io::prelude::*;
use std::mem::uninitialized;

use super::schema::*;

use quick_protobuf::{BytesReader, Writer};
use quick_protobuf::errors::Error as ProtoBufError;

#[macro_export]
macro_rules! protostr {
    ($s : expr) => { Some(::std::borrow::Cow::Borrowed($s)) }
}

static HELLO_MESSAGE : [u8; 12] = [0x48, 0x45, 0x4C, 0x4C, 0x4F, 0x2D, 0x52, 0x50, 0x43, 0x00, 0x00, 0x00];

#[derive(Debug)]
pub enum ConnectionErr {
    NameTooLong,
    TcpCreationFailure(::std::io::Error),
    HelloMessageFailed(::std::io::Error),
    NameMessageFailed(::std::io::Error),
    IdMessageFailed(::std::io::Error),
    IdMessageTooShort
}
#[derive(Debug)]
pub enum TransceiverError {
    Protobuf(ProtoBufError),
    Socket(::std::io::Error)
}
#[derive(Debug)]
pub struct Connection {
    id : [u8; 16],
    socket: TcpStream
}

impl Connection {
    pub fn new<T : ToSocketAddrs>(address : T, name : &str) -> Result<Self, ConnectionErr> {
        // The struct we're going to fill
        let mut ret : Self = unsafe { uninitialized() };
        macro_rules! unwrap_ret {
            ($res : expr, $err : tt) => {
                match $res {
                    Ok(i) => i,
                    Err(e) => return Err(ConnectionErr::$err(e))
                }
            }
        }
        if name.len() > 32 { return Err(ConnectionErr::NameTooLong) }
        let mut socket : TcpStream = unwrap_ret!(TcpStream::connect(address), TcpCreationFailure);
        // Send hello message
        unwrap_ret!(socket.write(&HELLO_MESSAGE), HelloMessageFailed);
        // Send name
        {
            // Get the name and knock off the nul at the end
            let name = &name.as_bytes();
            let mut buf = [0; 32];
            buf[..name.len()].copy_from_slice(name);
            unwrap_ret!(socket.write(&buf), NameMessageFailed);
        }

        if unwrap_ret!(socket.read(&mut ret.id), IdMessageFailed) != 16 {
            return Err(ConnectionErr::IdMessageTooShort)
        };
        // Set ret's socket
        ret.socket = socket;

        Ok(ret)
    }
    pub fn send(&mut self, r : Request) -> Result<(), ProtoBufError> {
        let mut w = Writer::new(&mut self.socket);

        w.write_message(&r)
    }
    pub fn receive<'a>(&mut self, buf : &'a mut Vec<u8>) -> Result<Response<'a>, TransceiverError> {

        if let Err(e) = self.socket.read_to_end(buf) { return Err(TransceiverError::Socket(e)); }

        println!("{:?}", buf);

        let mut r : BytesReader = BytesReader::from_bytes(buf);
        let ret = r.read_message(buf, Response::from_reader).unwrap();

        Ok(ret)
    }
}
