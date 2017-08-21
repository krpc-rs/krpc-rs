use super::*;
use super::schema::*;
use super::rpc::*;

use std::io::prelude::*;
use std::net::*;
use std::collections::BTreeMap;

use protobuf::*;

static STREAM_HELLO_MESSAGE : [u8; 12] = [0x48, 0x45, 0x4C, 0x4C, 0x4F, 0x2D, 0x53, 0x54, 0x52, 0x45, 0x41, 0x4D];
static STREAM_OK : [u8; 2] = [0x4F, 0x4B];
pub struct Stream {
    socket : TcpStream,
    rpc : Rpc
}
impl Stream {
    pub fn new<T : ToSocketAddrs>(address : T, rpc : Rpc) -> Result<Self, ConnectionErr> {
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
    pub fn add_stream(&mut self, service : String, procedure : String, args : Vec<Vec<u8>>) -> Result<u32, TransceiverError> {
        let mut buf = vec!();
        let request : Request = make_request!(service, procedure, args);
        request.write_to_vec(&mut buf)?;
        let mut v = vec!();
        v.push(buf);
        let response =
            if let Some(i) = self.rpc.invoke("KRPC".to_owned(), "AddStream".to_owned(), v)? { i }
            else { return Err(TransceiverError::ResponseHasError("No id".to_owned())) };
        let id = CodedInputStream::from_bytes(&response).read_uint32()?;
        Ok(id)
    }
    pub fn remove_stream(&mut self, id : u32) -> Result<(), TransceiverError> {
        let mut v = vec!();
        CodedOutputStream::vec(&mut v).write_uint32_no_tag(id)?;
        self.rpc.invoke("KRPC".to_owned(), "RemoveStream".to_owned(), vec!())?;
        Ok(())
    }
    pub fn receive(&mut self) -> Result<BTreeMap<u32, Option<Vec<u8>>>, TransceiverError> {
        let mut len = 0;
        {
            let ref mut buf = [0];
            let mut i = 0;
            loop {
                self.socket.read_exact(buf)?;
                len += ((buf[0] & 0b01111111) as usize) << i;
                if buf[0] & 0b10000000 == 0 { break }
                i += 7;
            }
        }
        let mut buffer = vec!(0; len);
        self.socket.read_exact(&mut buffer)?;
        let mut message = parse_from_bytes::<StreamMessage>(&buffer)?;
        let mut ret = BTreeMap::new();
        for i in message.responses.iter_mut() {
            let response = match i.response.take() {
                Some(i) => unwrap_response!(i)?,
                _ => continue
            };
            ret.insert(i.id, response);
        }
        Ok(ret)
    }
}
