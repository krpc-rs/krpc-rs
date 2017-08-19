//! This module doesn't seem to work yet

use super::*;
use super::schema::*;
use super::rpc::*;

use std::io::prelude::*;
use std::net::*;
use std::collections::BTreeMap;

static STREAM_HELLO_MESSAGE : [u8; 12] = [0x48, 0x45, 0x4C, 0x4C, 0x4F, 0x2D, 0x53, 0x54, 0x52, 0x45, 0x41, 0x4D];
static STREAM_OK : [u8; 2] = [0x4F, 0x4B];
pub struct Stream {
    socket : TcpStream,
    rpc : Rpc
}
impl Stream {
    pub unsafe fn new<T : ToSocketAddrs>(address : T, rpc : Rpc) -> Result<Self, ConnectionErr> {
        let mut socket : TcpStream = unwrap_ret!(TcpStream::connect(address), ConnectionErr::TcpConnectionFailure);
        unwrap_ret!(socket.write_all(&STREAM_HELLO_MESSAGE), ConnectionErr::HelloMessageFailed);
        unwrap_ret!(socket.write_all(&rpc.id), ConnectionErr::IdMessageFailed);
        let ref mut buf = [0; 2];
        unwrap_ret!(socket.read(buf), ConnectionErr::OkMessageFailed);
        if buf[..] != STREAM_OK[..] { return Err(ConnectionErr::OkMessageWrong) };
        Ok(Stream {
            socket : socket,
            rpc : rpc
        })
    }
    pub unsafe fn add_stream(&mut self, service : &str, procedure : &str, args : Vec<Vec<u8>>) -> Result<u32, TransceiverError> {
        let mut buf = vec!();
        let mut request : Request = make_request!(service, procedure, args);
        request.arguments = vec!();
        to_bytes!(&request, &mut buf)?;
        println!("{:?}", buf);
        let mut v = vec!();
        v.push(buf);
        println!("a");
        let response = self.rpc.invoke("KRPC", "AddStream", v)?;
        println!("a");
        let id = from_bytes!(varint32, &response)?;
        println!("a");
        Ok(id)
    }
    pub unsafe fn remove_stream(&mut self, id : u32) -> Result<(), TransceiverError> {
        let mut v = vec!();
        to_bytes!(varint, id as u64, &mut v)?;
        self.rpc.invoke("KRPC", "RemoveStream", vec!()).map(|_| ())
    }
    pub unsafe fn receive(&mut self) -> Result<BTreeMap<u32, Vec<u8>>, TransceiverError> {
        let mut buf = vec!();
        self.socket.read_to_end(&mut buf)?;
        let message : StreamMessage = from_bytes!(StreamMessage, &buf)?;
        let mut ret = BTreeMap::new();
        for StreamResponse { id, response } in message.responses {
            let response = match response { Some(i) => i, _ => continue };
            let id = match id { Some(i) => i, _ => continue };
            ret.insert(id, unwrap_response!(response)?);
        }
        Ok(ret)
    }
}
