use super::*;
use super::schema::*;

use std::io::prelude::*;
use std::net::*;

use quick_protobuf::{BytesReader, Writer};

static RPC_HELLO_MESSAGE : [u8; 12] = [0x48, 0x45, 0x4C, 0x4C, 0x4F, 0x2D, 0x52, 0x50, 0x43, 0x00, 0x00, 0x00];

#[derive(Debug)]
pub struct Rpc {
    pub id : Id,
    socket: TcpStream,
    buffer : Vec<u8>
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
            socket : socket,
            buffer : vec!()
        })
    }
    pub fn send(&mut self, r : Request) -> Result<(), ProtobufError> { Writer::new(&mut self.socket).write_message(&r) }
    pub fn receive<'a>(&'a mut self) -> Result<Vec<u8>, TransceiverError> {

        const BUFFER_SIZE : usize = 4 * 1024 * 1024;

        self.buffer.resize(4 * 1024 * 1024, 0);
        let mut last_buf = 0;
        let mut total = 0;
        // Poll the socket
        while last_buf == 0 { last_buf = unwrap_ret!(self.socket.read(&mut self.buffer), TransceiverError::Socket) };
        total += last_buf;
        // Resize as appropriate
        while last_buf == BUFFER_SIZE {
            &self.buffer.resize(total + BUFFER_SIZE, 0);
            last_buf = unwrap_ret!(self.socket.read(&mut self.buffer[total..]), TransceiverError::Socket);
            total += last_buf;
        }
        &self.buffer.resize(total, 0);

        let mut r : BytesReader = BytesReader::from_bytes(&self.buffer);

        let message = r.read_message(&self.buffer, Response::from_reader)?;

        let r = unwrap_response!(message);
        r
    }
    pub fn invoke(&mut self, service : &str, procedure : &str, args : Vec<Vec<u8>>) -> Result<Vec<u8>, TransceiverError> {
        let request = make_request!(service, procedure, args);
        self.send(request)?;
        self.receive()
    }
}
