//! Automatically generated rust module for 'krpc.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
//#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::io::Write;
use std::borrow::Cow;
use quick_protobuf::{MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Request<'a> {
    pub service: Option<Cow<'a, str>>,
    pub procedure: Option<Cow<'a, str>>,
    pub arguments: Vec<self::Argument<'a>>,
}

impl<'a> Request<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.service = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.procedure = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.arguments = r.read_packed(bytes, |r, bytes| r.read_message(bytes, self::Argument::from_reader))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Request<'a> {
    fn get_size(&self) -> usize {
        0
        + self.service.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.procedure.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + if self.arguments.is_empty() { 0 } else { 1 + sizeof_len(self.arguments.iter().map(|s| sizeof_len((s).get_size())).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.service { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.procedure { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        w.write_packed_with_tag(26, &self.arguments, |w, m| w.write_message(m), &|m| sizeof_len((m).get_size()))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Argument<'a> {
    pub position: Option<u32>,
    pub value: Option<Cow<'a, [u8]>>,
}

impl<'a> Argument<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.position = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.value = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Argument<'a> {
    fn get_size(&self) -> usize {
        0
        + self.position.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.position { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response<'a> {
    pub time: Option<f64>,
    pub has_error: Option<bool>,
    pub error: Option<Cow<'a, str>>,
    pub has_return_value: Option<bool>,
    pub return_value: Option<Cow<'a, [u8]>>,
}

impl<'a> Response<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.time = Some(r.read_double(bytes)?),
                Ok(16) => msg.has_error = Some(r.read_bool(bytes)?),
                Ok(26) => msg.error = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.has_return_value = Some(r.read_bool(bytes)?),
                Ok(42) => msg.return_value = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Response<'a> {
    fn get_size(&self) -> usize {
        0
        + self.time.as_ref().map_or(0, |_| 1 + 8)
        + self.has_error.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.error.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.has_return_value.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.return_value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.time { w.write_with_tag(9, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.has_error { w.write_with_tag(16, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.error { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.has_return_value { w.write_with_tag(32, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.return_value { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct StreamMessage<'a> {
    pub responses: Vec<self::StreamResponse<'a>>,
}

impl<'a> StreamMessage<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.responses = r.read_packed(bytes, |r, bytes| r.read_message(bytes, self::StreamResponse::from_reader))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StreamMessage<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.responses.is_empty() { 0 } else { 1 + sizeof_len(self.responses.iter().map(|s| sizeof_len((s).get_size())).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(10, &self.responses, |w, m| w.write_message(m), &|m| sizeof_len((m).get_size()))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct StreamResponse<'a> {
    pub id: Option<u32>,
    pub response: Option<self::Response<'a>>,
}

impl<'a> StreamResponse<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.response = Some(r.read_message(bytes, self::Response::from_reader)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StreamResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + self.id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.response.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.response { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Services<'a> {
    pub services: Vec<self::Service<'a>>,
}

impl<'a> Services<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.services = r.read_packed(bytes, |r, bytes| r.read_message(bytes, self::Service::from_reader))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Services<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.services.is_empty() { 0 } else { 1 + sizeof_len(self.services.iter().map(|s| sizeof_len((s).get_size())).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(10, &self.services, |w, m| w.write_message(m), &|m| sizeof_len((m).get_size()))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Service<'a> {
    pub name: Option<Cow<'a, str>>,
    pub procedures: Vec<self::Procedure<'a>>,
    pub classes: Vec<self::Class<'a>>,
    pub enumerations: Vec<self::Enumeration<'a>>,
    pub documentation: Option<Cow<'a, str>>,
}

impl<'a> Service<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.procedures = r.read_packed(bytes, |r, bytes| r.read_message(bytes, self::Procedure::from_reader))?,
                Ok(26) => msg.classes = r.read_packed(bytes, |r, bytes| r.read_message(bytes, self::Class::from_reader))?,
                Ok(34) => msg.enumerations = r.read_packed(bytes, |r, bytes| r.read_message(bytes, self::Enumeration::from_reader))?,
                Ok(42) => msg.documentation = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Service<'a> {
    fn get_size(&self) -> usize {
        0
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + if self.procedures.is_empty() { 0 } else { 1 + sizeof_len(self.procedures.iter().map(|s| sizeof_len((s).get_size())).sum::<usize>()) }
        + if self.classes.is_empty() { 0 } else { 1 + sizeof_len(self.classes.iter().map(|s| sizeof_len((s).get_size())).sum::<usize>()) }
        + if self.enumerations.is_empty() { 0 } else { 1 + sizeof_len(self.enumerations.iter().map(|s| sizeof_len((s).get_size())).sum::<usize>()) }
        + self.documentation.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.name { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        w.write_packed_with_tag(18, &self.procedures, |w, m| w.write_message(m), &|m| sizeof_len((m).get_size()))?;
        w.write_packed_with_tag(26, &self.classes, |w, m| w.write_message(m), &|m| sizeof_len((m).get_size()))?;
        w.write_packed_with_tag(34, &self.enumerations, |w, m| w.write_message(m), &|m| sizeof_len((m).get_size()))?;
        if let Some(ref s) = self.documentation { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Procedure<'a> {
    pub name: Option<Cow<'a, str>>,
    pub parameters: Vec<self::Parameter<'a>>,
    pub has_return_type: Option<bool>,
    pub return_type: Option<Cow<'a, str>>,
    pub attributes: Vec<Cow<'a, str>>,
    pub documentation: Option<Cow<'a, str>>,
}

impl<'a> Procedure<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.parameters = r.read_packed(bytes, |r, bytes| r.read_message(bytes, self::Parameter::from_reader))?,
                Ok(24) => msg.has_return_type = Some(r.read_bool(bytes)?),
                Ok(34) => msg.return_type = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.attributes = r.read_packed(bytes, |r, bytes| r.read_string(bytes).map(Cow::Borrowed))?,
                Ok(50) => msg.documentation = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Procedure<'a> {
    fn get_size(&self) -> usize {
        0
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + if self.parameters.is_empty() { 0 } else { 1 + sizeof_len(self.parameters.iter().map(|s| sizeof_len((s).get_size())).sum::<usize>()) }
        + self.has_return_type.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.return_type.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + if self.attributes.is_empty() { 0 } else { 1 + sizeof_len(self.attributes.iter().map(|s| sizeof_len((s).len())).sum::<usize>()) }
        + self.documentation.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.name { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        w.write_packed_with_tag(18, &self.parameters, |w, m| w.write_message(m), &|m| sizeof_len((m).get_size()))?;
        if let Some(ref s) = self.has_return_type { w.write_with_tag(24, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.return_type { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        w.write_packed_with_tag(42, &self.attributes, |w, m| w.write_string(&**m), &|m| sizeof_len((m).len()))?;
        if let Some(ref s) = self.documentation { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Parameter<'a> {
    pub name: Option<Cow<'a, str>>,
    pub type_pb: Option<Cow<'a, str>>,
    pub has_default_value: Option<bool>,
    pub default_value: Option<Cow<'a, [u8]>>,
}

impl<'a> Parameter<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.type_pb = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.has_default_value = Some(r.read_bool(bytes)?),
                Ok(34) => msg.default_value = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Parameter<'a> {
    fn get_size(&self) -> usize {
        0
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.has_default_value.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.default_value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.name { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.has_default_value { w.write_with_tag(24, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.default_value { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Class<'a> {
    pub name: Option<Cow<'a, str>>,
    pub documentation: Option<Cow<'a, str>>,
}

impl<'a> Class<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.documentation = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Class<'a> {
    fn get_size(&self) -> usize {
        0
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.documentation.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.name { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.documentation { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Enumeration<'a> {
    pub name: Option<Cow<'a, str>>,
    pub values: Vec<self::EnumerationValue<'a>>,
    pub documentation: Option<Cow<'a, str>>,
}

impl<'a> Enumeration<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.values = r.read_packed(bytes, |r, bytes| r.read_message(bytes, self::EnumerationValue::from_reader))?,
                Ok(26) => msg.documentation = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Enumeration<'a> {
    fn get_size(&self) -> usize {
        0
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + if self.values.is_empty() { 0 } else { 1 + sizeof_len(self.values.iter().map(|s| sizeof_len((s).get_size())).sum::<usize>()) }
        + self.documentation.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.name { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        w.write_packed_with_tag(18, &self.values, |w, m| w.write_message(m), &|m| sizeof_len((m).get_size()))?;
        if let Some(ref s) = self.documentation { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct EnumerationValue<'a> {
    pub name: Option<Cow<'a, str>>,
    pub value: Option<i32>,
    pub documentation: Option<Cow<'a, str>>,
}

impl<'a> EnumerationValue<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.value = Some(r.read_int32(bytes)?),
                Ok(26) => msg.documentation = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for EnumerationValue<'a> {
    fn get_size(&self) -> usize {
        0
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.documentation.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.name { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.documentation { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct List<'a> {
    pub items: Vec<Cow<'a, [u8]>>,
}

impl<'a> List<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.items = r.read_packed(bytes, |r, bytes| r.read_bytes(bytes).map(Cow::Borrowed))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for List<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.items.is_empty() { 0 } else { 1 + sizeof_len(self.items.iter().map(|s| sizeof_len((s).len())).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(10, &self.items, |w, m| w.write_bytes(&**m), &|m| sizeof_len((m).len()))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Dictionary<'a> {
    pub entries: Vec<self::DictionaryEntry<'a>>,
}

impl<'a> Dictionary<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.entries = r.read_packed(bytes, |r, bytes| r.read_message(bytes, self::DictionaryEntry::from_reader))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Dictionary<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.entries.is_empty() { 0 } else { 1 + sizeof_len(self.entries.iter().map(|s| sizeof_len((s).get_size())).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(10, &self.entries, |w, m| w.write_message(m), &|m| sizeof_len((m).get_size()))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DictionaryEntry<'a> {
    pub key: Option<Cow<'a, [u8]>>,
    pub value: Option<Cow<'a, [u8]>>,
}

impl<'a> DictionaryEntry<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.value = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DictionaryEntry<'a> {
    fn get_size(&self) -> usize {
        0
        + self.key.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.key { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Set<'a> {
    pub items: Vec<Cow<'a, [u8]>>,
}

impl<'a> Set<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.items = r.read_packed(bytes, |r, bytes| r.read_bytes(bytes).map(Cow::Borrowed))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Set<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.items.is_empty() { 0 } else { 1 + sizeof_len(self.items.iter().map(|s| sizeof_len((s).len())).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(10, &self.items, |w, m| w.write_bytes(&**m), &|m| sizeof_len((m).len()))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Tuple<'a> {
    pub items: Vec<Cow<'a, [u8]>>,
}

impl<'a> Tuple<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.items = r.read_packed(bytes, |r, bytes| r.read_bytes(bytes).map(Cow::Borrowed))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Tuple<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.items.is_empty() { 0 } else { 1 + sizeof_len(self.items.iter().map(|s| sizeof_len((s).len())).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(10, &self.items, |w, m| w.write_bytes(&**m), &|m| sizeof_len((m).len()))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Status<'a> {
    pub version: Option<Cow<'a, str>>,
    pub bytes_read: Option<u64>,
    pub bytes_written: Option<u64>,
    pub bytes_read_rate: Option<f32>,
    pub bytes_written_rate: Option<f32>,
    pub rpcs_executed: Option<u64>,
    pub rpc_rate: Option<f32>,
    pub one_rpc_per_update: Option<bool>,
    pub max_time_per_update: Option<u32>,
    pub adaptive_rate_control: Option<bool>,
    pub blocking_recv: Option<bool>,
    pub recv_timeout: Option<u32>,
    pub time_per_rpc_update: Option<f32>,
    pub poll_time_per_rpc_update: Option<f32>,
    pub exec_time_per_rpc_update: Option<f32>,
    pub stream_rpcs: Option<u32>,
    pub stream_rpcs_executed: Option<u64>,
    pub stream_rpc_rate: Option<f32>,
    pub time_per_stream_update: Option<f32>,
}

impl<'a> Status<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.version = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.bytes_read = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.bytes_written = Some(r.read_uint64(bytes)?),
                Ok(37) => msg.bytes_read_rate = Some(r.read_float(bytes)?),
                Ok(45) => msg.bytes_written_rate = Some(r.read_float(bytes)?),
                Ok(48) => msg.rpcs_executed = Some(r.read_uint64(bytes)?),
                Ok(61) => msg.rpc_rate = Some(r.read_float(bytes)?),
                Ok(64) => msg.one_rpc_per_update = Some(r.read_bool(bytes)?),
                Ok(72) => msg.max_time_per_update = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.adaptive_rate_control = Some(r.read_bool(bytes)?),
                Ok(88) => msg.blocking_recv = Some(r.read_bool(bytes)?),
                Ok(96) => msg.recv_timeout = Some(r.read_uint32(bytes)?),
                Ok(109) => msg.time_per_rpc_update = Some(r.read_float(bytes)?),
                Ok(117) => msg.poll_time_per_rpc_update = Some(r.read_float(bytes)?),
                Ok(125) => msg.exec_time_per_rpc_update = Some(r.read_float(bytes)?),
                Ok(128) => msg.stream_rpcs = Some(r.read_uint32(bytes)?),
                Ok(136) => msg.stream_rpcs_executed = Some(r.read_uint64(bytes)?),
                Ok(149) => msg.stream_rpc_rate = Some(r.read_float(bytes)?),
                Ok(157) => msg.time_per_stream_update = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Status<'a> {
    fn get_size(&self) -> usize {
        0
        + self.version.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.bytes_read.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.bytes_written.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.bytes_read_rate.as_ref().map_or(0, |_| 1 + 4)
        + self.bytes_written_rate.as_ref().map_or(0, |_| 1 + 4)
        + self.rpcs_executed.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.rpc_rate.as_ref().map_or(0, |_| 1 + 4)
        + self.one_rpc_per_update.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.max_time_per_update.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.adaptive_rate_control.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.blocking_recv.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.recv_timeout.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.time_per_rpc_update.as_ref().map_or(0, |_| 1 + 4)
        + self.poll_time_per_rpc_update.as_ref().map_or(0, |_| 1 + 4)
        + self.exec_time_per_rpc_update.as_ref().map_or(0, |_| 1 + 4)
        + self.stream_rpcs.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.stream_rpcs_executed.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.stream_rpc_rate.as_ref().map_or(0, |_| 2 + 4)
        + self.time_per_stream_update.as_ref().map_or(0, |_| 2 + 4)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.version { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.bytes_read { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.bytes_written { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.bytes_read_rate { w.write_with_tag(37, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.bytes_written_rate { w.write_with_tag(45, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.rpcs_executed { w.write_with_tag(48, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.rpc_rate { w.write_with_tag(61, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.one_rpc_per_update { w.write_with_tag(64, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.max_time_per_update { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.adaptive_rate_control { w.write_with_tag(80, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.blocking_recv { w.write_with_tag(88, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.recv_timeout { w.write_with_tag(96, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.time_per_rpc_update { w.write_with_tag(109, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.poll_time_per_rpc_update { w.write_with_tag(117, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.exec_time_per_rpc_update { w.write_with_tag(125, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.stream_rpcs { w.write_with_tag(128, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.stream_rpcs_executed { w.write_with_tag(136, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.stream_rpc_rate { w.write_with_tag(149, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.time_per_stream_update { w.write_with_tag(157, |w| w.write_float(*s))?; }
        Ok(())
    }
}
