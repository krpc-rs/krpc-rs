use super::*;
use super::schema::*;

use std::io::prelude::*;
use std::net::*;
use protobuf::*;

//use quick_protobuf::{BytesReader, Writer};

static RPC_HELLO_MESSAGE : [u8; 12] = [0x48, 0x45, 0x4C, 0x4C, 0x4F, 0x2D, 0x52, 0x50, 0x43, 0x00, 0x00, 0x00];

#[derive(Debug)]
pub struct Rpc {
    pub id : Id,
    socket: TcpStream
}

impl Rpc {
    pub fn new<T : ToSocketAddrs>(address : T, name : &str) -> Result<Self, ConnectionErr> {
        // The struct we're going to fill
        if name.len() > 32 { return Err(ConnectionErr::NameTooLong) }
        let mut socket : TcpStream = unwrap_ret!(TcpStream::connect(address), ConnectionErr::TcpConnectionFailure);
        // Send hello message
        unwrap_ret!(socket.write(&RPC_HELLO_MESSAGE), ConnectionErr::HelloMessageFailed);
        // Send name
        {
            // Get the name and knock off the null at the end
            let name = &name.as_bytes();
            let mut buf = [0; 32];
            buf[..name.len()].copy_from_slice(name);
            unwrap_ret!(socket.write(&buf), ConnectionErr::NameMessageFailed);
        }

        let mut id = [0; 16];
        if unwrap_ret!(socket.read(&mut id), ConnectionErr::IdMessageFailed) != 16 {
            return Err(ConnectionErr::IdMessageTooShort)
        };
        Ok(Self {
            id : id,
            socket : socket
        })
    }
    pub fn send(&mut self, r : Request) -> Result<(), ProtobufError> { r.write_length_delimited_to_writer(&mut self.socket) }
    pub fn receive<'a>(&'a mut self) -> Result<Option<Vec<u8>>, TransceiverError> {
        let mut len = 0;
        {
            let ref mut buf = [0];
            let mut i = 0;
            loop {
                self.socket.read_exact(buf)?;
                len += ((buf[0] & 0b01111111) as usize) << i;
                if buf[0] & 0b10000000 == 0 { break }
                i += 1;
            }
        }
        let mut buffer = vec!(0; len);
        self.socket.read_exact(&mut buffer)?;
        let message = parse_from_bytes::<Response>(&buffer)?;
        unwrap_response!(message)
    }
    pub fn invoke(&mut self, service : String, procedure : String, args : Vec<Vec<u8>>) -> Result<Option<Vec<u8>>, TransceiverError> {
        let request = make_request!(service, procedure, args);
        self.send(request)?;
        self.receive()
    }
}
