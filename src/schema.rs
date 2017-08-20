// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Request {
    // message fields
    pub service: ::std::string::String,
    pub procedure: ::std::string::String,
    pub arguments: ::protobuf::RepeatedField<Argument>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request {}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(Request::new)
        }
    }

    // string service = 1;

    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: ::std::string::String) {
        self.service = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service(&mut self) -> &mut ::std::string::String {
        &mut self.service
    }

    // Take field
    pub fn take_service(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.service, ::std::string::String::new())
    }

    pub fn get_service(&self) -> &str {
        &self.service
    }

    fn get_service_for_reflect(&self) -> &::std::string::String {
        &self.service
    }

    fn mut_service_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.service
    }

    // string procedure = 2;

    pub fn clear_procedure(&mut self) {
        self.procedure.clear();
    }

    // Param is passed by value, moved
    pub fn set_procedure(&mut self, v: ::std::string::String) {
        self.procedure = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_procedure(&mut self) -> &mut ::std::string::String {
        &mut self.procedure
    }

    // Take field
    pub fn take_procedure(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.procedure, ::std::string::String::new())
    }

    pub fn get_procedure(&self) -> &str {
        &self.procedure
    }

    fn get_procedure_for_reflect(&self) -> &::std::string::String {
        &self.procedure
    }

    fn mut_procedure_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.procedure
    }

    // repeated .krpc.schema.Argument arguments = 3;

    pub fn clear_arguments(&mut self) {
        self.arguments.clear();
    }

    // Param is passed by value, moved
    pub fn set_arguments(&mut self, v: ::protobuf::RepeatedField<Argument>) {
        self.arguments = v;
    }

    // Mutable pointer to the field.
    pub fn mut_arguments(&mut self) -> &mut ::protobuf::RepeatedField<Argument> {
        &mut self.arguments
    }

    // Take field
    pub fn take_arguments(&mut self) -> ::protobuf::RepeatedField<Argument> {
        ::std::mem::replace(&mut self.arguments, ::protobuf::RepeatedField::new())
    }

    pub fn get_arguments(&self) -> &[Argument] {
        &self.arguments
    }

    fn get_arguments_for_reflect(&self) -> &::protobuf::RepeatedField<Argument> {
        &self.arguments
    }

    fn mut_arguments_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Argument> {
        &mut self.arguments
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        for v in &self.arguments {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.service)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.procedure)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.arguments)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.service.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.service);
        }
        if !self.procedure.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.procedure);
        }
        for value in &self.arguments {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.service.is_empty() {
            os.write_string(1, &self.service)?;
        }
        if !self.procedure.is_empty() {
            os.write_string(2, &self.procedure)?;
        }
        for v in &self.arguments {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Request {
    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service",
                    Request::get_service_for_reflect,
                    Request::mut_service_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "procedure",
                    Request::get_procedure_for_reflect,
                    Request::mut_procedure_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Argument>>(
                    "arguments",
                    Request::get_arguments_for_reflect,
                    Request::mut_arguments_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_service();
        self.clear_procedure();
        self.clear_arguments();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Argument {
    // message fields
    pub position: u32,
    pub value: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Argument {}

impl Argument {
    pub fn new() -> Argument {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Argument {
        static mut instance: ::protobuf::lazy::Lazy<Argument> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Argument,
        };
        unsafe {
            instance.get(Argument::new)
        }
    }

    // uint32 position = 1;

    pub fn clear_position(&mut self) {
        self.position = 0;
    }

    // Param is passed by value, moved
    pub fn set_position(&mut self, v: u32) {
        self.position = v;
    }

    pub fn get_position(&self) -> u32 {
        self.position
    }

    fn get_position_for_reflect(&self) -> &u32 {
        &self.position
    }

    fn mut_position_for_reflect(&mut self) -> &mut u32 {
        &mut self.position
    }

    // bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }
}

impl ::protobuf::Message for Argument {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.position = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.position != 0 {
            my_size += ::protobuf::rt::value_size(1, self.position, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.position != 0 {
            os.write_uint32(1, self.position)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(2, &self.value)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Argument {
    fn new() -> Argument {
        Argument::new()
    }

    fn descriptor_static(_: ::std::option::Option<Argument>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "position",
                    Argument::get_position_for_reflect,
                    Argument::mut_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    Argument::get_value_for_reflect,
                    Argument::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Argument>(
                    "Argument",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Argument {
    fn clear(&mut self) {
        self.clear_position();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Argument {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Argument {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message fields
    pub time: f64,
    pub has_error: bool,
    pub error: ::std::string::String,
    pub has_return_value: bool,
    pub return_value: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(Response::new)
        }
    }

    // double time = 1;

    pub fn clear_time(&mut self) {
        self.time = 0.;
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: f64) {
        self.time = v;
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }

    fn get_time_for_reflect(&self) -> &f64 {
        &self.time
    }

    fn mut_time_for_reflect(&mut self) -> &mut f64 {
        &mut self.time
    }

    // bool has_error = 2;

    pub fn clear_has_error(&mut self) {
        self.has_error = false;
    }

    // Param is passed by value, moved
    pub fn set_has_error(&mut self, v: bool) {
        self.has_error = v;
    }

    pub fn get_has_error(&self) -> bool {
        self.has_error
    }

    fn get_has_error_for_reflect(&self) -> &bool {
        &self.has_error
    }

    fn mut_has_error_for_reflect(&mut self) -> &mut bool {
        &mut self.has_error
    }

    // string error = 3;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // bool has_return_value = 4;

    pub fn clear_has_return_value(&mut self) {
        self.has_return_value = false;
    }

    // Param is passed by value, moved
    pub fn set_has_return_value(&mut self, v: bool) {
        self.has_return_value = v;
    }

    pub fn get_has_return_value(&self) -> bool {
        self.has_return_value
    }

    fn get_has_return_value_for_reflect(&self) -> &bool {
        &self.has_return_value
    }

    fn mut_has_return_value_for_reflect(&mut self) -> &mut bool {
        &mut self.has_return_value
    }

    // bytes return_value = 5;

    pub fn clear_return_value(&mut self) {
        self.return_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_return_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.return_value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_return_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.return_value
    }

    // Take field
    pub fn take_return_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.return_value, ::std::vec::Vec::new())
    }

    pub fn get_return_value(&self) -> &[u8] {
        &self.return_value
    }

    fn get_return_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.return_value
    }

    fn mut_return_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.return_value
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.time = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.has_error = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.has_return_value = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.return_value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.time != 0. {
            my_size += 9;
        }
        if self.has_error != false {
            my_size += 2;
        }
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.error);
        }
        if self.has_return_value != false {
            my_size += 2;
        }
        if !self.return_value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.return_value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.time != 0. {
            os.write_double(1, self.time)?;
        }
        if self.has_error != false {
            os.write_bool(2, self.has_error)?;
        }
        if !self.error.is_empty() {
            os.write_string(3, &self.error)?;
        }
        if self.has_return_value != false {
            os.write_bool(4, self.has_return_value)?;
        }
        if !self.return_value.is_empty() {
            os.write_bytes(5, &self.return_value)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Response {
    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "time",
                    Response::get_time_for_reflect,
                    Response::mut_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_error",
                    Response::get_has_error_for_reflect,
                    Response::mut_has_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    Response::get_error_for_reflect,
                    Response::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_return_value",
                    Response::get_has_return_value_for_reflect,
                    Response::mut_has_return_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "return_value",
                    Response::get_return_value_for_reflect,
                    Response::mut_return_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.clear_time();
        self.clear_has_error();
        self.clear_error();
        self.clear_has_return_value();
        self.clear_return_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StreamMessage {
    // message fields
    pub responses: ::protobuf::RepeatedField<StreamResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StreamMessage {}

impl StreamMessage {
    pub fn new() -> StreamMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StreamMessage {
        static mut instance: ::protobuf::lazy::Lazy<StreamMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StreamMessage,
        };
        unsafe {
            instance.get(StreamMessage::new)
        }
    }

    // repeated .krpc.schema.StreamResponse responses = 1;

    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }

    // Param is passed by value, moved
    pub fn set_responses(&mut self, v: ::protobuf::RepeatedField<StreamResponse>) {
        self.responses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_responses(&mut self) -> &mut ::protobuf::RepeatedField<StreamResponse> {
        &mut self.responses
    }

    // Take field
    pub fn take_responses(&mut self) -> ::protobuf::RepeatedField<StreamResponse> {
        ::std::mem::replace(&mut self.responses, ::protobuf::RepeatedField::new())
    }

    pub fn get_responses(&self) -> &[StreamResponse] {
        &self.responses
    }

    fn get_responses_for_reflect(&self) -> &::protobuf::RepeatedField<StreamResponse> {
        &self.responses
    }

    fn mut_responses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<StreamResponse> {
        &mut self.responses
    }
}

impl ::protobuf::Message for StreamMessage {
    fn is_initialized(&self) -> bool {
        for v in &self.responses {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.responses)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.responses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.responses {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StreamMessage {
    fn new() -> StreamMessage {
        StreamMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<StreamMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StreamResponse>>(
                    "responses",
                    StreamMessage::get_responses_for_reflect,
                    StreamMessage::mut_responses_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StreamMessage>(
                    "StreamMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StreamMessage {
    fn clear(&mut self) {
        self.clear_responses();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StreamMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StreamResponse {
    // message fields
    pub id: u32,
    pub response: ::protobuf::SingularPtrField<Response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StreamResponse {}

impl StreamResponse {
    pub fn new() -> StreamResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StreamResponse {
        static mut instance: ::protobuf::lazy::Lazy<StreamResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StreamResponse,
        };
        unsafe {
            instance.get(StreamResponse::new)
        }
    }

    // uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.id
    }

    // .krpc.schema.Response response = 2;

    pub fn clear_response(&mut self) {
        self.response.clear();
    }

    pub fn has_response(&self) -> bool {
        self.response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: Response) {
        self.response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response(&mut self) -> &mut Response {
        if self.response.is_none() {
            self.response.set_default();
        }
        self.response.as_mut().unwrap()
    }

    // Take field
    pub fn take_response(&mut self) -> Response {
        self.response.take().unwrap_or_else(|| Response::new())
    }

    pub fn get_response(&self) -> &Response {
        self.response.as_ref().unwrap_or_else(|| Response::default_instance())
    }

    fn get_response_for_reflect(&self) -> &::protobuf::SingularPtrField<Response> {
        &self.response
    }

    fn mut_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Response> {
        &mut self.response
    }
}

impl ::protobuf::Message for StreamResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.response {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.response)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_uint32(1, self.id)?;
        }
        if let Some(ref v) = self.response.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StreamResponse {
    fn new() -> StreamResponse {
        StreamResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<StreamResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    StreamResponse::get_id_for_reflect,
                    StreamResponse::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Response>>(
                    "response",
                    StreamResponse::get_response_for_reflect,
                    StreamResponse::mut_response_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StreamResponse>(
                    "StreamResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StreamResponse {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_response();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StreamResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Services {
    // message fields
    pub services: ::protobuf::RepeatedField<Service>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Services {}

impl Services {
    pub fn new() -> Services {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Services {
        static mut instance: ::protobuf::lazy::Lazy<Services> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Services,
        };
        unsafe {
            instance.get(Services::new)
        }
    }

    // repeated .krpc.schema.Service services = 1;

    pub fn clear_services(&mut self) {
        self.services.clear();
    }

    // Param is passed by value, moved
    pub fn set_services(&mut self, v: ::protobuf::RepeatedField<Service>) {
        self.services = v;
    }

    // Mutable pointer to the field.
    pub fn mut_services(&mut self) -> &mut ::protobuf::RepeatedField<Service> {
        &mut self.services
    }

    // Take field
    pub fn take_services(&mut self) -> ::protobuf::RepeatedField<Service> {
        ::std::mem::replace(&mut self.services, ::protobuf::RepeatedField::new())
    }

    pub fn get_services(&self) -> &[Service] {
        &self.services
    }

    fn get_services_for_reflect(&self) -> &::protobuf::RepeatedField<Service> {
        &self.services
    }

    fn mut_services_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Service> {
        &mut self.services
    }
}

impl ::protobuf::Message for Services {
    fn is_initialized(&self) -> bool {
        for v in &self.services {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.services)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.services {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.services {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Services {
    fn new() -> Services {
        Services::new()
    }

    fn descriptor_static(_: ::std::option::Option<Services>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Service>>(
                    "services",
                    Services::get_services_for_reflect,
                    Services::mut_services_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Services>(
                    "Services",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Services {
    fn clear(&mut self) {
        self.clear_services();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Services {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Services {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Service {
    // message fields
    pub name: ::std::string::String,
    pub procedures: ::protobuf::RepeatedField<Procedure>,
    pub classes: ::protobuf::RepeatedField<Class>,
    pub enumerations: ::protobuf::RepeatedField<Enumeration>,
    pub documentation: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Service {}

impl Service {
    pub fn new() -> Service {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Service {
        static mut instance: ::protobuf::lazy::Lazy<Service> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Service,
        };
        unsafe {
            instance.get(Service::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // repeated .krpc.schema.Procedure procedures = 2;

    pub fn clear_procedures(&mut self) {
        self.procedures.clear();
    }

    // Param is passed by value, moved
    pub fn set_procedures(&mut self, v: ::protobuf::RepeatedField<Procedure>) {
        self.procedures = v;
    }

    // Mutable pointer to the field.
    pub fn mut_procedures(&mut self) -> &mut ::protobuf::RepeatedField<Procedure> {
        &mut self.procedures
    }

    // Take field
    pub fn take_procedures(&mut self) -> ::protobuf::RepeatedField<Procedure> {
        ::std::mem::replace(&mut self.procedures, ::protobuf::RepeatedField::new())
    }

    pub fn get_procedures(&self) -> &[Procedure] {
        &self.procedures
    }

    fn get_procedures_for_reflect(&self) -> &::protobuf::RepeatedField<Procedure> {
        &self.procedures
    }

    fn mut_procedures_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Procedure> {
        &mut self.procedures
    }

    // repeated .krpc.schema.Class classes = 3;

    pub fn clear_classes(&mut self) {
        self.classes.clear();
    }

    // Param is passed by value, moved
    pub fn set_classes(&mut self, v: ::protobuf::RepeatedField<Class>) {
        self.classes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_classes(&mut self) -> &mut ::protobuf::RepeatedField<Class> {
        &mut self.classes
    }

    // Take field
    pub fn take_classes(&mut self) -> ::protobuf::RepeatedField<Class> {
        ::std::mem::replace(&mut self.classes, ::protobuf::RepeatedField::new())
    }

    pub fn get_classes(&self) -> &[Class] {
        &self.classes
    }

    fn get_classes_for_reflect(&self) -> &::protobuf::RepeatedField<Class> {
        &self.classes
    }

    fn mut_classes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Class> {
        &mut self.classes
    }

    // repeated .krpc.schema.Enumeration enumerations = 4;

    pub fn clear_enumerations(&mut self) {
        self.enumerations.clear();
    }

    // Param is passed by value, moved
    pub fn set_enumerations(&mut self, v: ::protobuf::RepeatedField<Enumeration>) {
        self.enumerations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_enumerations(&mut self) -> &mut ::protobuf::RepeatedField<Enumeration> {
        &mut self.enumerations
    }

    // Take field
    pub fn take_enumerations(&mut self) -> ::protobuf::RepeatedField<Enumeration> {
        ::std::mem::replace(&mut self.enumerations, ::protobuf::RepeatedField::new())
    }

    pub fn get_enumerations(&self) -> &[Enumeration] {
        &self.enumerations
    }

    fn get_enumerations_for_reflect(&self) -> &::protobuf::RepeatedField<Enumeration> {
        &self.enumerations
    }

    fn mut_enumerations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Enumeration> {
        &mut self.enumerations
    }

    // string documentation = 5;

    pub fn clear_documentation(&mut self) {
        self.documentation.clear();
    }

    // Param is passed by value, moved
    pub fn set_documentation(&mut self, v: ::std::string::String) {
        self.documentation = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_documentation(&mut self) -> &mut ::std::string::String {
        &mut self.documentation
    }

    // Take field
    pub fn take_documentation(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.documentation, ::std::string::String::new())
    }

    pub fn get_documentation(&self) -> &str {
        &self.documentation
    }

    fn get_documentation_for_reflect(&self) -> &::std::string::String {
        &self.documentation
    }

    fn mut_documentation_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.documentation
    }
}

impl ::protobuf::Message for Service {
    fn is_initialized(&self) -> bool {
        for v in &self.procedures {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.classes {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.enumerations {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.procedures)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.classes)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.enumerations)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.documentation)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        for value in &self.procedures {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.classes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.enumerations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.documentation.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.documentation);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        for v in &self.procedures {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.classes {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.enumerations {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.documentation.is_empty() {
            os.write_string(5, &self.documentation)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Service {
    fn new() -> Service {
        Service::new()
    }

    fn descriptor_static(_: ::std::option::Option<Service>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Service::get_name_for_reflect,
                    Service::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Procedure>>(
                    "procedures",
                    Service::get_procedures_for_reflect,
                    Service::mut_procedures_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Class>>(
                    "classes",
                    Service::get_classes_for_reflect,
                    Service::mut_classes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Enumeration>>(
                    "enumerations",
                    Service::get_enumerations_for_reflect,
                    Service::mut_enumerations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "documentation",
                    Service::get_documentation_for_reflect,
                    Service::mut_documentation_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Service>(
                    "Service",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Service {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_procedures();
        self.clear_classes();
        self.clear_enumerations();
        self.clear_documentation();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Service {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Service {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Procedure {
    // message fields
    pub name: ::std::string::String,
    pub parameters: ::protobuf::RepeatedField<Parameter>,
    pub has_return_type: bool,
    pub return_type: ::std::string::String,
    pub attributes: ::protobuf::RepeatedField<::std::string::String>,
    pub documentation: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Procedure {}

impl Procedure {
    pub fn new() -> Procedure {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Procedure {
        static mut instance: ::protobuf::lazy::Lazy<Procedure> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Procedure,
        };
        unsafe {
            instance.get(Procedure::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // repeated .krpc.schema.Parameter parameters = 2;

    pub fn clear_parameters(&mut self) {
        self.parameters.clear();
    }

    // Param is passed by value, moved
    pub fn set_parameters(&mut self, v: ::protobuf::RepeatedField<Parameter>) {
        self.parameters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_parameters(&mut self) -> &mut ::protobuf::RepeatedField<Parameter> {
        &mut self.parameters
    }

    // Take field
    pub fn take_parameters(&mut self) -> ::protobuf::RepeatedField<Parameter> {
        ::std::mem::replace(&mut self.parameters, ::protobuf::RepeatedField::new())
    }

    pub fn get_parameters(&self) -> &[Parameter] {
        &self.parameters
    }

    fn get_parameters_for_reflect(&self) -> &::protobuf::RepeatedField<Parameter> {
        &self.parameters
    }

    fn mut_parameters_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Parameter> {
        &mut self.parameters
    }

    // bool has_return_type = 3;

    pub fn clear_has_return_type(&mut self) {
        self.has_return_type = false;
    }

    // Param is passed by value, moved
    pub fn set_has_return_type(&mut self, v: bool) {
        self.has_return_type = v;
    }

    pub fn get_has_return_type(&self) -> bool {
        self.has_return_type
    }

    fn get_has_return_type_for_reflect(&self) -> &bool {
        &self.has_return_type
    }

    fn mut_has_return_type_for_reflect(&mut self) -> &mut bool {
        &mut self.has_return_type
    }

    // string return_type = 4;

    pub fn clear_return_type(&mut self) {
        self.return_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_return_type(&mut self, v: ::std::string::String) {
        self.return_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_return_type(&mut self) -> &mut ::std::string::String {
        &mut self.return_type
    }

    // Take field
    pub fn take_return_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.return_type, ::std::string::String::new())
    }

    pub fn get_return_type(&self) -> &str {
        &self.return_type
    }

    fn get_return_type_for_reflect(&self) -> &::std::string::String {
        &self.return_type
    }

    fn mut_return_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.return_type
    }

    // repeated string attributes = 5;

    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }

    // Param is passed by value, moved
    pub fn set_attributes(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.attributes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attributes(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.attributes
    }

    // Take field
    pub fn take_attributes(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.attributes, ::protobuf::RepeatedField::new())
    }

    pub fn get_attributes(&self) -> &[::std::string::String] {
        &self.attributes
    }

    fn get_attributes_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.attributes
    }

    fn mut_attributes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.attributes
    }

    // string documentation = 6;

    pub fn clear_documentation(&mut self) {
        self.documentation.clear();
    }

    // Param is passed by value, moved
    pub fn set_documentation(&mut self, v: ::std::string::String) {
        self.documentation = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_documentation(&mut self) -> &mut ::std::string::String {
        &mut self.documentation
    }

    // Take field
    pub fn take_documentation(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.documentation, ::std::string::String::new())
    }

    pub fn get_documentation(&self) -> &str {
        &self.documentation
    }

    fn get_documentation_for_reflect(&self) -> &::std::string::String {
        &self.documentation
    }

    fn mut_documentation_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.documentation
    }
}

impl ::protobuf::Message for Procedure {
    fn is_initialized(&self) -> bool {
        for v in &self.parameters {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.parameters)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.has_return_type = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.return_type)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.attributes)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.documentation)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        for value in &self.parameters {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.has_return_type != false {
            my_size += 2;
        }
        if !self.return_type.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.return_type);
        }
        for value in &self.attributes {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        if !self.documentation.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.documentation);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        for v in &self.parameters {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.has_return_type != false {
            os.write_bool(3, self.has_return_type)?;
        }
        if !self.return_type.is_empty() {
            os.write_string(4, &self.return_type)?;
        }
        for v in &self.attributes {
            os.write_string(5, &v)?;
        };
        if !self.documentation.is_empty() {
            os.write_string(6, &self.documentation)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Procedure {
    fn new() -> Procedure {
        Procedure::new()
    }

    fn descriptor_static(_: ::std::option::Option<Procedure>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Procedure::get_name_for_reflect,
                    Procedure::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Parameter>>(
                    "parameters",
                    Procedure::get_parameters_for_reflect,
                    Procedure::mut_parameters_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_return_type",
                    Procedure::get_has_return_type_for_reflect,
                    Procedure::mut_has_return_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "return_type",
                    Procedure::get_return_type_for_reflect,
                    Procedure::mut_return_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "attributes",
                    Procedure::get_attributes_for_reflect,
                    Procedure::mut_attributes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "documentation",
                    Procedure::get_documentation_for_reflect,
                    Procedure::mut_documentation_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Procedure>(
                    "Procedure",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Procedure {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_parameters();
        self.clear_has_return_type();
        self.clear_return_type();
        self.clear_attributes();
        self.clear_documentation();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Procedure {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Procedure {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Parameter {
    // message fields
    pub name: ::std::string::String,
    pub field_type: ::std::string::String,
    pub has_default_value: bool,
    pub default_value: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Parameter {}

impl Parameter {
    pub fn new() -> Parameter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Parameter {
        static mut instance: ::protobuf::lazy::Lazy<Parameter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Parameter,
        };
        unsafe {
            instance.get(Parameter::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_type, ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        &self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &::std::string::String {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // bool has_default_value = 3;

    pub fn clear_has_default_value(&mut self) {
        self.has_default_value = false;
    }

    // Param is passed by value, moved
    pub fn set_has_default_value(&mut self, v: bool) {
        self.has_default_value = v;
    }

    pub fn get_has_default_value(&self) -> bool {
        self.has_default_value
    }

    fn get_has_default_value_for_reflect(&self) -> &bool {
        &self.has_default_value
    }

    fn mut_has_default_value_for_reflect(&mut self) -> &mut bool {
        &mut self.has_default_value
    }

    // bytes default_value = 4;

    pub fn clear_default_value(&mut self) {
        self.default_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_default_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.default_value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_default_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.default_value
    }

    // Take field
    pub fn take_default_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.default_value, ::std::vec::Vec::new())
    }

    pub fn get_default_value(&self) -> &[u8] {
        &self.default_value
    }

    fn get_default_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.default_value
    }

    fn mut_default_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.default_value
    }
}

impl ::protobuf::Message for Parameter {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.has_default_value = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.default_value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.field_type);
        }
        if self.has_default_value != false {
            my_size += 2;
        }
        if !self.default_value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.default_value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.field_type.is_empty() {
            os.write_string(2, &self.field_type)?;
        }
        if self.has_default_value != false {
            os.write_bool(3, self.has_default_value)?;
        }
        if !self.default_value.is_empty() {
            os.write_bytes(4, &self.default_value)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Parameter {
    fn new() -> Parameter {
        Parameter::new()
    }

    fn descriptor_static(_: ::std::option::Option<Parameter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Parameter::get_name_for_reflect,
                    Parameter::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    Parameter::get_field_type_for_reflect,
                    Parameter::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_default_value",
                    Parameter::get_has_default_value_for_reflect,
                    Parameter::mut_has_default_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "default_value",
                    Parameter::get_default_value_for_reflect,
                    Parameter::mut_default_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Parameter>(
                    "Parameter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Parameter {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_field_type();
        self.clear_has_default_value();
        self.clear_default_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Parameter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Parameter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Class {
    // message fields
    pub name: ::std::string::String,
    pub documentation: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Class {}

impl Class {
    pub fn new() -> Class {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Class {
        static mut instance: ::protobuf::lazy::Lazy<Class> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Class,
        };
        unsafe {
            instance.get(Class::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string documentation = 2;

    pub fn clear_documentation(&mut self) {
        self.documentation.clear();
    }

    // Param is passed by value, moved
    pub fn set_documentation(&mut self, v: ::std::string::String) {
        self.documentation = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_documentation(&mut self) -> &mut ::std::string::String {
        &mut self.documentation
    }

    // Take field
    pub fn take_documentation(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.documentation, ::std::string::String::new())
    }

    pub fn get_documentation(&self) -> &str {
        &self.documentation
    }

    fn get_documentation_for_reflect(&self) -> &::std::string::String {
        &self.documentation
    }

    fn mut_documentation_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.documentation
    }
}

impl ::protobuf::Message for Class {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.documentation)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.documentation.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.documentation);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.documentation.is_empty() {
            os.write_string(2, &self.documentation)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Class {
    fn new() -> Class {
        Class::new()
    }

    fn descriptor_static(_: ::std::option::Option<Class>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Class::get_name_for_reflect,
                    Class::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "documentation",
                    Class::get_documentation_for_reflect,
                    Class::mut_documentation_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Class>(
                    "Class",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Class {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_documentation();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Class {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Class {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Enumeration {
    // message fields
    pub name: ::std::string::String,
    pub values: ::protobuf::RepeatedField<EnumerationValue>,
    pub documentation: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Enumeration {}

impl Enumeration {
    pub fn new() -> Enumeration {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Enumeration {
        static mut instance: ::protobuf::lazy::Lazy<Enumeration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Enumeration,
        };
        unsafe {
            instance.get(Enumeration::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // repeated .krpc.schema.EnumerationValue values = 2;

    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::protobuf::RepeatedField<EnumerationValue>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values(&mut self) -> &mut ::protobuf::RepeatedField<EnumerationValue> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> ::protobuf::RepeatedField<EnumerationValue> {
        ::std::mem::replace(&mut self.values, ::protobuf::RepeatedField::new())
    }

    pub fn get_values(&self) -> &[EnumerationValue] {
        &self.values
    }

    fn get_values_for_reflect(&self) -> &::protobuf::RepeatedField<EnumerationValue> {
        &self.values
    }

    fn mut_values_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<EnumerationValue> {
        &mut self.values
    }

    // string documentation = 3;

    pub fn clear_documentation(&mut self) {
        self.documentation.clear();
    }

    // Param is passed by value, moved
    pub fn set_documentation(&mut self, v: ::std::string::String) {
        self.documentation = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_documentation(&mut self) -> &mut ::std::string::String {
        &mut self.documentation
    }

    // Take field
    pub fn take_documentation(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.documentation, ::std::string::String::new())
    }

    pub fn get_documentation(&self) -> &str {
        &self.documentation
    }

    fn get_documentation_for_reflect(&self) -> &::std::string::String {
        &self.documentation
    }

    fn mut_documentation_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.documentation
    }
}

impl ::protobuf::Message for Enumeration {
    fn is_initialized(&self) -> bool {
        for v in &self.values {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.values)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.documentation)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        for value in &self.values {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.documentation.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.documentation);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        for v in &self.values {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.documentation.is_empty() {
            os.write_string(3, &self.documentation)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Enumeration {
    fn new() -> Enumeration {
        Enumeration::new()
    }

    fn descriptor_static(_: ::std::option::Option<Enumeration>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Enumeration::get_name_for_reflect,
                    Enumeration::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EnumerationValue>>(
                    "values",
                    Enumeration::get_values_for_reflect,
                    Enumeration::mut_values_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "documentation",
                    Enumeration::get_documentation_for_reflect,
                    Enumeration::mut_documentation_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Enumeration>(
                    "Enumeration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Enumeration {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_values();
        self.clear_documentation();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Enumeration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Enumeration {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EnumerationValue {
    // message fields
    pub name: ::std::string::String,
    pub value: i32,
    pub documentation: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EnumerationValue {}

impl EnumerationValue {
    pub fn new() -> EnumerationValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EnumerationValue {
        static mut instance: ::protobuf::lazy::Lazy<EnumerationValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EnumerationValue,
        };
        unsafe {
            instance.get(EnumerationValue::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // int32 value = 2;

    pub fn clear_value(&mut self) {
        self.value = 0;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i32) {
        self.value = v;
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    fn get_value_for_reflect(&self) -> &i32 {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut i32 {
        &mut self.value
    }

    // string documentation = 3;

    pub fn clear_documentation(&mut self) {
        self.documentation.clear();
    }

    // Param is passed by value, moved
    pub fn set_documentation(&mut self, v: ::std::string::String) {
        self.documentation = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_documentation(&mut self) -> &mut ::std::string::String {
        &mut self.documentation
    }

    // Take field
    pub fn take_documentation(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.documentation, ::std::string::String::new())
    }

    pub fn get_documentation(&self) -> &str {
        &self.documentation
    }

    fn get_documentation_for_reflect(&self) -> &::std::string::String {
        &self.documentation
    }

    fn mut_documentation_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.documentation
    }
}

impl ::protobuf::Message for EnumerationValue {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.value = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.documentation)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if self.value != 0 {
            my_size += ::protobuf::rt::value_size(2, self.value, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.documentation.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.documentation);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.value != 0 {
            os.write_int32(2, self.value)?;
        }
        if !self.documentation.is_empty() {
            os.write_string(3, &self.documentation)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EnumerationValue {
    fn new() -> EnumerationValue {
        EnumerationValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<EnumerationValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    EnumerationValue::get_name_for_reflect,
                    EnumerationValue::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "value",
                    EnumerationValue::get_value_for_reflect,
                    EnumerationValue::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "documentation",
                    EnumerationValue::get_documentation_for_reflect,
                    EnumerationValue::mut_documentation_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EnumerationValue>(
                    "EnumerationValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EnumerationValue {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.clear_documentation();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EnumerationValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnumerationValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct List {
    // message fields
    pub items: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for List {}

impl List {
    pub fn new() -> List {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static List {
        static mut instance: ::protobuf::lazy::Lazy<List> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const List,
        };
        unsafe {
            instance.get(List::new)
        }
    }

    // repeated bytes items = 1;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[::std::vec::Vec<u8>] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.items
    }
}

impl ::protobuf::Message for List {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.items)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.items {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.items {
            os.write_bytes(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for List {
    fn new() -> List {
        List::new()
    }

    fn descriptor_static(_: ::std::option::Option<List>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "items",
                    List::get_items_for_reflect,
                    List::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<List>(
                    "List",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for List {
    fn clear(&mut self) {
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for List {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for List {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Dictionary {
    // message fields
    pub entries: ::protobuf::RepeatedField<DictionaryEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Dictionary {}

impl Dictionary {
    pub fn new() -> Dictionary {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Dictionary {
        static mut instance: ::protobuf::lazy::Lazy<Dictionary> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Dictionary,
        };
        unsafe {
            instance.get(Dictionary::new)
        }
    }

    // repeated .krpc.schema.DictionaryEntry entries = 1;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<DictionaryEntry>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<DictionaryEntry> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<DictionaryEntry> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[DictionaryEntry] {
        &self.entries
    }

    fn get_entries_for_reflect(&self) -> &::protobuf::RepeatedField<DictionaryEntry> {
        &self.entries
    }

    fn mut_entries_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DictionaryEntry> {
        &mut self.entries
    }
}

impl ::protobuf::Message for Dictionary {
    fn is_initialized(&self) -> bool {
        for v in &self.entries {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.entries {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Dictionary {
    fn new() -> Dictionary {
        Dictionary::new()
    }

    fn descriptor_static(_: ::std::option::Option<Dictionary>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DictionaryEntry>>(
                    "entries",
                    Dictionary::get_entries_for_reflect,
                    Dictionary::mut_entries_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Dictionary>(
                    "Dictionary",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Dictionary {
    fn clear(&mut self) {
        self.clear_entries();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Dictionary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Dictionary {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DictionaryEntry {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub value: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DictionaryEntry {}

impl DictionaryEntry {
    pub fn new() -> DictionaryEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DictionaryEntry {
        static mut instance: ::protobuf::lazy::Lazy<DictionaryEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DictionaryEntry,
        };
        unsafe {
            instance.get(DictionaryEntry::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }
}

impl ::protobuf::Message for DictionaryEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(2, &self.value)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DictionaryEntry {
    fn new() -> DictionaryEntry {
        DictionaryEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<DictionaryEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    DictionaryEntry::get_key_for_reflect,
                    DictionaryEntry::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    DictionaryEntry::get_value_for_reflect,
                    DictionaryEntry::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DictionaryEntry>(
                    "DictionaryEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DictionaryEntry {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DictionaryEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DictionaryEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Set {
    // message fields
    pub items: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Set {}

impl Set {
    pub fn new() -> Set {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Set {
        static mut instance: ::protobuf::lazy::Lazy<Set> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Set,
        };
        unsafe {
            instance.get(Set::new)
        }
    }

    // repeated bytes items = 1;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[::std::vec::Vec<u8>] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.items
    }
}

impl ::protobuf::Message for Set {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.items)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.items {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.items {
            os.write_bytes(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Set {
    fn new() -> Set {
        Set::new()
    }

    fn descriptor_static(_: ::std::option::Option<Set>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "items",
                    Set::get_items_for_reflect,
                    Set::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Set>(
                    "Set",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Set {
    fn clear(&mut self) {
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Set {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Set {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Tuple {
    // message fields
    pub items: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Tuple {}

impl Tuple {
    pub fn new() -> Tuple {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Tuple {
        static mut instance: ::protobuf::lazy::Lazy<Tuple> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Tuple,
        };
        unsafe {
            instance.get(Tuple::new)
        }
    }

    // repeated bytes items = 1;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[::std::vec::Vec<u8>] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.items
    }
}

impl ::protobuf::Message for Tuple {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.items)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.items {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.items {
            os.write_bytes(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Tuple {
    fn new() -> Tuple {
        Tuple::new()
    }

    fn descriptor_static(_: ::std::option::Option<Tuple>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "items",
                    Tuple::get_items_for_reflect,
                    Tuple::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Tuple>(
                    "Tuple",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Tuple {
    fn clear(&mut self) {
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Tuple {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Tuple {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Status {
    // message fields
    pub version: ::std::string::String,
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub bytes_read_rate: f32,
    pub bytes_written_rate: f32,
    pub rpcs_executed: u64,
    pub rpc_rate: f32,
    pub one_rpc_per_update: bool,
    pub max_time_per_update: u32,
    pub adaptive_rate_control: bool,
    pub blocking_recv: bool,
    pub recv_timeout: u32,
    pub time_per_rpc_update: f32,
    pub poll_time_per_rpc_update: f32,
    pub exec_time_per_rpc_update: f32,
    pub stream_rpcs: u32,
    pub stream_rpcs_executed: u64,
    pub stream_rpc_rate: f32,
    pub time_per_stream_update: f32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Status {}

impl Status {
    pub fn new() -> Status {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Status {
        static mut instance: ::protobuf::lazy::Lazy<Status> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Status,
        };
        unsafe {
            instance.get(Status::new)
        }
    }

    // string version = 1;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.version, ::std::string::String::new())
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    fn get_version_for_reflect(&self) -> &::std::string::String {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }

    // uint64 bytes_read = 2;

    pub fn clear_bytes_read(&mut self) {
        self.bytes_read = 0;
    }

    // Param is passed by value, moved
    pub fn set_bytes_read(&mut self, v: u64) {
        self.bytes_read = v;
    }

    pub fn get_bytes_read(&self) -> u64 {
        self.bytes_read
    }

    fn get_bytes_read_for_reflect(&self) -> &u64 {
        &self.bytes_read
    }

    fn mut_bytes_read_for_reflect(&mut self) -> &mut u64 {
        &mut self.bytes_read
    }

    // uint64 bytes_written = 3;

    pub fn clear_bytes_written(&mut self) {
        self.bytes_written = 0;
    }

    // Param is passed by value, moved
    pub fn set_bytes_written(&mut self, v: u64) {
        self.bytes_written = v;
    }

    pub fn get_bytes_written(&self) -> u64 {
        self.bytes_written
    }

    fn get_bytes_written_for_reflect(&self) -> &u64 {
        &self.bytes_written
    }

    fn mut_bytes_written_for_reflect(&mut self) -> &mut u64 {
        &mut self.bytes_written
    }

    // float bytes_read_rate = 4;

    pub fn clear_bytes_read_rate(&mut self) {
        self.bytes_read_rate = 0.;
    }

    // Param is passed by value, moved
    pub fn set_bytes_read_rate(&mut self, v: f32) {
        self.bytes_read_rate = v;
    }

    pub fn get_bytes_read_rate(&self) -> f32 {
        self.bytes_read_rate
    }

    fn get_bytes_read_rate_for_reflect(&self) -> &f32 {
        &self.bytes_read_rate
    }

    fn mut_bytes_read_rate_for_reflect(&mut self) -> &mut f32 {
        &mut self.bytes_read_rate
    }

    // float bytes_written_rate = 5;

    pub fn clear_bytes_written_rate(&mut self) {
        self.bytes_written_rate = 0.;
    }

    // Param is passed by value, moved
    pub fn set_bytes_written_rate(&mut self, v: f32) {
        self.bytes_written_rate = v;
    }

    pub fn get_bytes_written_rate(&self) -> f32 {
        self.bytes_written_rate
    }

    fn get_bytes_written_rate_for_reflect(&self) -> &f32 {
        &self.bytes_written_rate
    }

    fn mut_bytes_written_rate_for_reflect(&mut self) -> &mut f32 {
        &mut self.bytes_written_rate
    }

    // uint64 rpcs_executed = 6;

    pub fn clear_rpcs_executed(&mut self) {
        self.rpcs_executed = 0;
    }

    // Param is passed by value, moved
    pub fn set_rpcs_executed(&mut self, v: u64) {
        self.rpcs_executed = v;
    }

    pub fn get_rpcs_executed(&self) -> u64 {
        self.rpcs_executed
    }

    fn get_rpcs_executed_for_reflect(&self) -> &u64 {
        &self.rpcs_executed
    }

    fn mut_rpcs_executed_for_reflect(&mut self) -> &mut u64 {
        &mut self.rpcs_executed
    }

    // float rpc_rate = 7;

    pub fn clear_rpc_rate(&mut self) {
        self.rpc_rate = 0.;
    }

    // Param is passed by value, moved
    pub fn set_rpc_rate(&mut self, v: f32) {
        self.rpc_rate = v;
    }

    pub fn get_rpc_rate(&self) -> f32 {
        self.rpc_rate
    }

    fn get_rpc_rate_for_reflect(&self) -> &f32 {
        &self.rpc_rate
    }

    fn mut_rpc_rate_for_reflect(&mut self) -> &mut f32 {
        &mut self.rpc_rate
    }

    // bool one_rpc_per_update = 8;

    pub fn clear_one_rpc_per_update(&mut self) {
        self.one_rpc_per_update = false;
    }

    // Param is passed by value, moved
    pub fn set_one_rpc_per_update(&mut self, v: bool) {
        self.one_rpc_per_update = v;
    }

    pub fn get_one_rpc_per_update(&self) -> bool {
        self.one_rpc_per_update
    }

    fn get_one_rpc_per_update_for_reflect(&self) -> &bool {
        &self.one_rpc_per_update
    }

    fn mut_one_rpc_per_update_for_reflect(&mut self) -> &mut bool {
        &mut self.one_rpc_per_update
    }

    // uint32 max_time_per_update = 9;

    pub fn clear_max_time_per_update(&mut self) {
        self.max_time_per_update = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_time_per_update(&mut self, v: u32) {
        self.max_time_per_update = v;
    }

    pub fn get_max_time_per_update(&self) -> u32 {
        self.max_time_per_update
    }

    fn get_max_time_per_update_for_reflect(&self) -> &u32 {
        &self.max_time_per_update
    }

    fn mut_max_time_per_update_for_reflect(&mut self) -> &mut u32 {
        &mut self.max_time_per_update
    }

    // bool adaptive_rate_control = 10;

    pub fn clear_adaptive_rate_control(&mut self) {
        self.adaptive_rate_control = false;
    }

    // Param is passed by value, moved
    pub fn set_adaptive_rate_control(&mut self, v: bool) {
        self.adaptive_rate_control = v;
    }

    pub fn get_adaptive_rate_control(&self) -> bool {
        self.adaptive_rate_control
    }

    fn get_adaptive_rate_control_for_reflect(&self) -> &bool {
        &self.adaptive_rate_control
    }

    fn mut_adaptive_rate_control_for_reflect(&mut self) -> &mut bool {
        &mut self.adaptive_rate_control
    }

    // bool blocking_recv = 11;

    pub fn clear_blocking_recv(&mut self) {
        self.blocking_recv = false;
    }

    // Param is passed by value, moved
    pub fn set_blocking_recv(&mut self, v: bool) {
        self.blocking_recv = v;
    }

    pub fn get_blocking_recv(&self) -> bool {
        self.blocking_recv
    }

    fn get_blocking_recv_for_reflect(&self) -> &bool {
        &self.blocking_recv
    }

    fn mut_blocking_recv_for_reflect(&mut self) -> &mut bool {
        &mut self.blocking_recv
    }

    // uint32 recv_timeout = 12;

    pub fn clear_recv_timeout(&mut self) {
        self.recv_timeout = 0;
    }

    // Param is passed by value, moved
    pub fn set_recv_timeout(&mut self, v: u32) {
        self.recv_timeout = v;
    }

    pub fn get_recv_timeout(&self) -> u32 {
        self.recv_timeout
    }

    fn get_recv_timeout_for_reflect(&self) -> &u32 {
        &self.recv_timeout
    }

    fn mut_recv_timeout_for_reflect(&mut self) -> &mut u32 {
        &mut self.recv_timeout
    }

    // float time_per_rpc_update = 13;

    pub fn clear_time_per_rpc_update(&mut self) {
        self.time_per_rpc_update = 0.;
    }

    // Param is passed by value, moved
    pub fn set_time_per_rpc_update(&mut self, v: f32) {
        self.time_per_rpc_update = v;
    }

    pub fn get_time_per_rpc_update(&self) -> f32 {
        self.time_per_rpc_update
    }

    fn get_time_per_rpc_update_for_reflect(&self) -> &f32 {
        &self.time_per_rpc_update
    }

    fn mut_time_per_rpc_update_for_reflect(&mut self) -> &mut f32 {
        &mut self.time_per_rpc_update
    }

    // float poll_time_per_rpc_update = 14;

    pub fn clear_poll_time_per_rpc_update(&mut self) {
        self.poll_time_per_rpc_update = 0.;
    }

    // Param is passed by value, moved
    pub fn set_poll_time_per_rpc_update(&mut self, v: f32) {
        self.poll_time_per_rpc_update = v;
    }

    pub fn get_poll_time_per_rpc_update(&self) -> f32 {
        self.poll_time_per_rpc_update
    }

    fn get_poll_time_per_rpc_update_for_reflect(&self) -> &f32 {
        &self.poll_time_per_rpc_update
    }

    fn mut_poll_time_per_rpc_update_for_reflect(&mut self) -> &mut f32 {
        &mut self.poll_time_per_rpc_update
    }

    // float exec_time_per_rpc_update = 15;

    pub fn clear_exec_time_per_rpc_update(&mut self) {
        self.exec_time_per_rpc_update = 0.;
    }

    // Param is passed by value, moved
    pub fn set_exec_time_per_rpc_update(&mut self, v: f32) {
        self.exec_time_per_rpc_update = v;
    }

    pub fn get_exec_time_per_rpc_update(&self) -> f32 {
        self.exec_time_per_rpc_update
    }

    fn get_exec_time_per_rpc_update_for_reflect(&self) -> &f32 {
        &self.exec_time_per_rpc_update
    }

    fn mut_exec_time_per_rpc_update_for_reflect(&mut self) -> &mut f32 {
        &mut self.exec_time_per_rpc_update
    }

    // uint32 stream_rpcs = 16;

    pub fn clear_stream_rpcs(&mut self) {
        self.stream_rpcs = 0;
    }

    // Param is passed by value, moved
    pub fn set_stream_rpcs(&mut self, v: u32) {
        self.stream_rpcs = v;
    }

    pub fn get_stream_rpcs(&self) -> u32 {
        self.stream_rpcs
    }

    fn get_stream_rpcs_for_reflect(&self) -> &u32 {
        &self.stream_rpcs
    }

    fn mut_stream_rpcs_for_reflect(&mut self) -> &mut u32 {
        &mut self.stream_rpcs
    }

    // uint64 stream_rpcs_executed = 17;

    pub fn clear_stream_rpcs_executed(&mut self) {
        self.stream_rpcs_executed = 0;
    }

    // Param is passed by value, moved
    pub fn set_stream_rpcs_executed(&mut self, v: u64) {
        self.stream_rpcs_executed = v;
    }

    pub fn get_stream_rpcs_executed(&self) -> u64 {
        self.stream_rpcs_executed
    }

    fn get_stream_rpcs_executed_for_reflect(&self) -> &u64 {
        &self.stream_rpcs_executed
    }

    fn mut_stream_rpcs_executed_for_reflect(&mut self) -> &mut u64 {
        &mut self.stream_rpcs_executed
    }

    // float stream_rpc_rate = 18;

    pub fn clear_stream_rpc_rate(&mut self) {
        self.stream_rpc_rate = 0.;
    }

    // Param is passed by value, moved
    pub fn set_stream_rpc_rate(&mut self, v: f32) {
        self.stream_rpc_rate = v;
    }

    pub fn get_stream_rpc_rate(&self) -> f32 {
        self.stream_rpc_rate
    }

    fn get_stream_rpc_rate_for_reflect(&self) -> &f32 {
        &self.stream_rpc_rate
    }

    fn mut_stream_rpc_rate_for_reflect(&mut self) -> &mut f32 {
        &mut self.stream_rpc_rate
    }

    // float time_per_stream_update = 19;

    pub fn clear_time_per_stream_update(&mut self) {
        self.time_per_stream_update = 0.;
    }

    // Param is passed by value, moved
    pub fn set_time_per_stream_update(&mut self, v: f32) {
        self.time_per_stream_update = v;
    }

    pub fn get_time_per_stream_update(&self) -> f32 {
        self.time_per_stream_update
    }

    fn get_time_per_stream_update_for_reflect(&self) -> &f32 {
        &self.time_per_stream_update
    }

    fn mut_time_per_stream_update_for_reflect(&mut self) -> &mut f32 {
        &mut self.time_per_stream_update
    }
}

impl ::protobuf::Message for Status {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.version)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.bytes_read = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.bytes_written = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.bytes_read_rate = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.bytes_written_rate = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.rpcs_executed = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.rpc_rate = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.one_rpc_per_update = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.max_time_per_update = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.adaptive_rate_control = tmp;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.blocking_recv = tmp;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.recv_timeout = tmp;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.time_per_rpc_update = tmp;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.poll_time_per_rpc_update = tmp;
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.exec_time_per_rpc_update = tmp;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.stream_rpcs = tmp;
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.stream_rpcs_executed = tmp;
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.stream_rpc_rate = tmp;
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.time_per_stream_update = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.version.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.version);
        }
        if self.bytes_read != 0 {
            my_size += ::protobuf::rt::value_size(2, self.bytes_read, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.bytes_written != 0 {
            my_size += ::protobuf::rt::value_size(3, self.bytes_written, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.bytes_read_rate != 0. {
            my_size += 5;
        }
        if self.bytes_written_rate != 0. {
            my_size += 5;
        }
        if self.rpcs_executed != 0 {
            my_size += ::protobuf::rt::value_size(6, self.rpcs_executed, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.rpc_rate != 0. {
            my_size += 5;
        }
        if self.one_rpc_per_update != false {
            my_size += 2;
        }
        if self.max_time_per_update != 0 {
            my_size += ::protobuf::rt::value_size(9, self.max_time_per_update, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.adaptive_rate_control != false {
            my_size += 2;
        }
        if self.blocking_recv != false {
            my_size += 2;
        }
        if self.recv_timeout != 0 {
            my_size += ::protobuf::rt::value_size(12, self.recv_timeout, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.time_per_rpc_update != 0. {
            my_size += 5;
        }
        if self.poll_time_per_rpc_update != 0. {
            my_size += 5;
        }
        if self.exec_time_per_rpc_update != 0. {
            my_size += 5;
        }
        if self.stream_rpcs != 0 {
            my_size += ::protobuf::rt::value_size(16, self.stream_rpcs, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.stream_rpcs_executed != 0 {
            my_size += ::protobuf::rt::value_size(17, self.stream_rpcs_executed, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.stream_rpc_rate != 0. {
            my_size += 6;
        }
        if self.time_per_stream_update != 0. {
            my_size += 6;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.version.is_empty() {
            os.write_string(1, &self.version)?;
        }
        if self.bytes_read != 0 {
            os.write_uint64(2, self.bytes_read)?;
        }
        if self.bytes_written != 0 {
            os.write_uint64(3, self.bytes_written)?;
        }
        if self.bytes_read_rate != 0. {
            os.write_float(4, self.bytes_read_rate)?;
        }
        if self.bytes_written_rate != 0. {
            os.write_float(5, self.bytes_written_rate)?;
        }
        if self.rpcs_executed != 0 {
            os.write_uint64(6, self.rpcs_executed)?;
        }
        if self.rpc_rate != 0. {
            os.write_float(7, self.rpc_rate)?;
        }
        if self.one_rpc_per_update != false {
            os.write_bool(8, self.one_rpc_per_update)?;
        }
        if self.max_time_per_update != 0 {
            os.write_uint32(9, self.max_time_per_update)?;
        }
        if self.adaptive_rate_control != false {
            os.write_bool(10, self.adaptive_rate_control)?;
        }
        if self.blocking_recv != false {
            os.write_bool(11, self.blocking_recv)?;
        }
        if self.recv_timeout != 0 {
            os.write_uint32(12, self.recv_timeout)?;
        }
        if self.time_per_rpc_update != 0. {
            os.write_float(13, self.time_per_rpc_update)?;
        }
        if self.poll_time_per_rpc_update != 0. {
            os.write_float(14, self.poll_time_per_rpc_update)?;
        }
        if self.exec_time_per_rpc_update != 0. {
            os.write_float(15, self.exec_time_per_rpc_update)?;
        }
        if self.stream_rpcs != 0 {
            os.write_uint32(16, self.stream_rpcs)?;
        }
        if self.stream_rpcs_executed != 0 {
            os.write_uint64(17, self.stream_rpcs_executed)?;
        }
        if self.stream_rpc_rate != 0. {
            os.write_float(18, self.stream_rpc_rate)?;
        }
        if self.time_per_stream_update != 0. {
            os.write_float(19, self.time_per_stream_update)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Status {
    fn new() -> Status {
        Status::new()
    }

    fn descriptor_static(_: ::std::option::Option<Status>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    Status::get_version_for_reflect,
                    Status::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "bytes_read",
                    Status::get_bytes_read_for_reflect,
                    Status::mut_bytes_read_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "bytes_written",
                    Status::get_bytes_written_for_reflect,
                    Status::mut_bytes_written_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "bytes_read_rate",
                    Status::get_bytes_read_rate_for_reflect,
                    Status::mut_bytes_read_rate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "bytes_written_rate",
                    Status::get_bytes_written_rate_for_reflect,
                    Status::mut_bytes_written_rate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "rpcs_executed",
                    Status::get_rpcs_executed_for_reflect,
                    Status::mut_rpcs_executed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "rpc_rate",
                    Status::get_rpc_rate_for_reflect,
                    Status::mut_rpc_rate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "one_rpc_per_update",
                    Status::get_one_rpc_per_update_for_reflect,
                    Status::mut_one_rpc_per_update_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "max_time_per_update",
                    Status::get_max_time_per_update_for_reflect,
                    Status::mut_max_time_per_update_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "adaptive_rate_control",
                    Status::get_adaptive_rate_control_for_reflect,
                    Status::mut_adaptive_rate_control_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "blocking_recv",
                    Status::get_blocking_recv_for_reflect,
                    Status::mut_blocking_recv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "recv_timeout",
                    Status::get_recv_timeout_for_reflect,
                    Status::mut_recv_timeout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "time_per_rpc_update",
                    Status::get_time_per_rpc_update_for_reflect,
                    Status::mut_time_per_rpc_update_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "poll_time_per_rpc_update",
                    Status::get_poll_time_per_rpc_update_for_reflect,
                    Status::mut_poll_time_per_rpc_update_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "exec_time_per_rpc_update",
                    Status::get_exec_time_per_rpc_update_for_reflect,
                    Status::mut_exec_time_per_rpc_update_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "stream_rpcs",
                    Status::get_stream_rpcs_for_reflect,
                    Status::mut_stream_rpcs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "stream_rpcs_executed",
                    Status::get_stream_rpcs_executed_for_reflect,
                    Status::mut_stream_rpcs_executed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "stream_rpc_rate",
                    Status::get_stream_rpc_rate_for_reflect,
                    Status::mut_stream_rpc_rate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "time_per_stream_update",
                    Status::get_time_per_stream_update_for_reflect,
                    Status::mut_time_per_stream_update_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Status>(
                    "Status",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Status {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_bytes_read();
        self.clear_bytes_written();
        self.clear_bytes_read_rate();
        self.clear_bytes_written_rate();
        self.clear_rpcs_executed();
        self.clear_rpc_rate();
        self.clear_one_rpc_per_update();
        self.clear_max_time_per_update();
        self.clear_adaptive_rate_control();
        self.clear_blocking_recv();
        self.clear_recv_timeout();
        self.clear_time_per_rpc_update();
        self.clear_poll_time_per_rpc_update();
        self.clear_exec_time_per_rpc_update();
        self.clear_stream_rpcs();
        self.clear_stream_rpcs_executed();
        self.clear_stream_rpc_rate();
        self.clear_time_per_stream_update();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Status {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nkrpc.proto\x12\x0bkrpc.schema\"v\n\x07Request\x12\x18\n\x07service\
    \x18\x01\x20\x01(\tR\x07service\x12\x1c\n\tprocedure\x18\x02\x20\x01(\tR\
    \tprocedure\x123\n\targuments\x18\x03\x20\x03(\x0b2\x15.krpc.schema.Argu\
    mentR\targuments\"<\n\x08Argument\x12\x1a\n\x08position\x18\x01\x20\x01(\
    \rR\x08position\x12\x14\n\x05value\x18\x02\x20\x01(\x0cR\x05value\"\x9e\
    \x01\n\x08Response\x12\x12\n\x04time\x18\x01\x20\x01(\x01R\x04time\x12\
    \x1b\n\thas_error\x18\x02\x20\x01(\x08R\x08hasError\x12\x14\n\x05error\
    \x18\x03\x20\x01(\tR\x05error\x12(\n\x10has_return_value\x18\x04\x20\x01\
    (\x08R\x0ehasReturnValue\x12!\n\x0creturn_value\x18\x05\x20\x01(\x0cR\
    \x0breturnValue\"J\n\rStreamMessage\x129\n\tresponses\x18\x01\x20\x03(\
    \x0b2\x1b.krpc.schema.StreamResponseR\tresponses\"S\n\x0eStreamResponse\
    \x12\x0e\n\x02id\x18\x01\x20\x01(\rR\x02id\x121\n\x08response\x18\x02\
    \x20\x01(\x0b2\x15.krpc.schema.ResponseR\x08response\"<\n\x08Services\
    \x120\n\x08services\x18\x01\x20\x03(\x0b2\x14.krpc.schema.ServiceR\x08se\
    rvices\"\xe7\x01\n\x07Service\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04n\
    ame\x126\n\nprocedures\x18\x02\x20\x03(\x0b2\x16.krpc.schema.ProcedureR\
    \nprocedures\x12,\n\x07classes\x18\x03\x20\x03(\x0b2\x12.krpc.schema.Cla\
    ssR\x07classes\x12<\n\x0cenumerations\x18\x04\x20\x03(\x0b2\x18.krpc.sch\
    ema.EnumerationR\x0cenumerations\x12$\n\rdocumentation\x18\x05\x20\x01(\
    \tR\rdocumentation\"\xe6\x01\n\tProcedure\x12\x12\n\x04name\x18\x01\x20\
    \x01(\tR\x04name\x126\n\nparameters\x18\x02\x20\x03(\x0b2\x16.krpc.schem\
    a.ParameterR\nparameters\x12&\n\x0fhas_return_type\x18\x03\x20\x01(\x08R\
    \rhasReturnType\x12\x1f\n\x0breturn_type\x18\x04\x20\x01(\tR\nreturnType\
    \x12\x1e\n\nattributes\x18\x05\x20\x03(\tR\nattributes\x12$\n\rdocumenta\
    tion\x18\x06\x20\x01(\tR\rdocumentation\"\x84\x01\n\tParameter\x12\x12\n\
    \x04name\x18\x01\x20\x01(\tR\x04name\x12\x12\n\x04type\x18\x02\x20\x01(\
    \tR\x04type\x12*\n\x11has_default_value\x18\x03\x20\x01(\x08R\x0fhasDefa\
    ultValue\x12#\n\rdefault_value\x18\x04\x20\x01(\x0cR\x0cdefaultValue\"A\
    \n\x05Class\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12$\n\rdocume\
    ntation\x18\x02\x20\x01(\tR\rdocumentation\"~\n\x0bEnumeration\x12\x12\n\
    \x04name\x18\x01\x20\x01(\tR\x04name\x125\n\x06values\x18\x02\x20\x03(\
    \x0b2\x1d.krpc.schema.EnumerationValueR\x06values\x12$\n\rdocumentation\
    \x18\x03\x20\x01(\tR\rdocumentation\"b\n\x10EnumerationValue\x12\x12\n\
    \x04name\x18\x01\x20\x01(\tR\x04name\x12\x14\n\x05value\x18\x02\x20\x01(\
    \x05R\x05value\x12$\n\rdocumentation\x18\x03\x20\x01(\tR\rdocumentation\
    \"\x1c\n\x04List\x12\x14\n\x05items\x18\x01\x20\x03(\x0cR\x05items\"D\n\
    \nDictionary\x126\n\x07entries\x18\x01\x20\x03(\x0b2\x1c.krpc.schema.Dic\
    tionaryEntryR\x07entries\"9\n\x0fDictionaryEntry\x12\x10\n\x03key\x18\
    \x01\x20\x01(\x0cR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\x0cR\x05va\
    lue\"\x1b\n\x03Set\x12\x14\n\x05items\x18\x01\x20\x03(\x0cR\x05items\"\
    \x1d\n\x05Tuple\x12\x14\n\x05items\x18\x01\x20\x03(\x0cR\x05items\"\xa3\
    \x06\n\x06Status\x12\x18\n\x07version\x18\x01\x20\x01(\tR\x07version\x12\
    \x1d\n\nbytes_read\x18\x02\x20\x01(\x04R\tbytesRead\x12#\n\rbytes_writte\
    n\x18\x03\x20\x01(\x04R\x0cbytesWritten\x12&\n\x0fbytes_read_rate\x18\
    \x04\x20\x01(\x02R\rbytesReadRate\x12,\n\x12bytes_written_rate\x18\x05\
    \x20\x01(\x02R\x10bytesWrittenRate\x12#\n\rrpcs_executed\x18\x06\x20\x01\
    (\x04R\x0crpcsExecuted\x12\x19\n\x08rpc_rate\x18\x07\x20\x01(\x02R\x07rp\
    cRate\x12+\n\x12one_rpc_per_update\x18\x08\x20\x01(\x08R\x0foneRpcPerUpd\
    ate\x12-\n\x13max_time_per_update\x18\t\x20\x01(\rR\x10maxTimePerUpdate\
    \x122\n\x15adaptive_rate_control\x18\n\x20\x01(\x08R\x13adaptiveRateCont\
    rol\x12#\n\rblocking_recv\x18\x0b\x20\x01(\x08R\x0cblockingRecv\x12!\n\
    \x0crecv_timeout\x18\x0c\x20\x01(\rR\x0brecvTimeout\x12-\n\x13time_per_r\
    pc_update\x18\r\x20\x01(\x02R\x10timePerRpcUpdate\x126\n\x18poll_time_pe\
    r_rpc_update\x18\x0e\x20\x01(\x02R\x14pollTimePerRpcUpdate\x126\n\x18exe\
    c_time_per_rpc_update\x18\x0f\x20\x01(\x02R\x14execTimePerRpcUpdate\x12\
    \x1f\n\x0bstream_rpcs\x18\x10\x20\x01(\rR\nstreamRpcs\x120\n\x14stream_r\
    pcs_executed\x18\x11\x20\x01(\x04R\x12streamRpcsExecuted\x12&\n\x0fstrea\
    m_rpc_rate\x18\x12\x20\x01(\x02R\rstreamRpcRate\x123\n\x16time_per_strea\
    m_update\x18\x13\x20\x01(\x02R\x13timePerStreamUpdateB&\n\x0bkrpc.schema\
    B\x04KRPC\xaa\x02\x10KRPC.Schema.KRPCJ\xa9'\n\x06\x12\x04\0\0x\x01\n\x08\
    \n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\x13\n\x08\n\
    \x01\x08\x12\x03\x02\0-\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x02\0-\n\x0c\n\
    \x05\x08\xe7\x07\0\x02\x12\x03\x02\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\
    \x12\x03\x02\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x02\x07\
    \x17\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x02\x1a,\n\x08\n\x01\x08\x12\
    \x03\x03\0$\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x03\0$\n\x0c\n\x05\x08\
    \xe7\x07\x01\x02\x12\x03\x03\x07\x13\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\
    \x03\x03\x07\x13\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x03\x07\
    \x13\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x03\x16#\n\x08\n\x01\x08\
    \x12\x03\x04\0%\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x04\0%\n\x0c\n\x05\
    \x08\xe7\x07\x02\x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x02\x02\0\
    \x12\x03\x04\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x04\
    \x07\x1b\n\x0c\n\x05\x08\xe7\x07\x02\x07\x12\x03\x04\x1e$\n\n\n\x02\x04\
    \0\x12\x04\x06\0\n\x01\n\n\n\x03\x04\0\x01\x12\x03\x06\x08\x0f\n\x0b\n\
    \x04\x04\0\x02\0\x12\x03\x07\x02\x15\n\r\n\x05\x04\0\x02\0\x04\x12\x04\
    \x07\x02\x06\x11\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x07\x02\x08\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\x07\t\x10\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x07\x13\x14\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x08\x02\x17\n\r\n\x05\
    \x04\0\x02\x01\x04\x12\x04\x08\x02\x07\x15\n\x0c\n\x05\x04\0\x02\x01\x05\
    \x12\x03\x08\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x08\t\x12\n\
    \x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x08\x15\x16\n\x0b\n\x04\x04\0\x02\
    \x02\x12\x03\t\x02\"\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\t\x02\n\n\x0c\
    \n\x05\x04\0\x02\x02\x06\x12\x03\t\x0b\x13\n\x0c\n\x05\x04\0\x02\x02\x01\
    \x12\x03\t\x14\x1d\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\t\x20!\n\n\n\
    \x02\x04\x01\x12\x04\x0c\0\x0f\x01\n\n\n\x03\x04\x01\x01\x12\x03\x0c\x08\
    \x10\n\x0b\n\x04\x04\x01\x02\0\x12\x03\r\x02\x16\n\r\n\x05\x04\x01\x02\0\
    \x04\x12\x04\r\x02\x0c\x12\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\r\x02\
    \x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\r\t\x11\n\x0c\n\x05\x04\x01\
    \x02\0\x03\x12\x03\r\x14\x15\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x0e\x02\
    \x12\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x0e\x02\r\x16\n\x0c\n\x05\x04\
    \x01\x02\x01\x05\x12\x03\x0e\x02\x07\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\
    \x03\x0e\x08\r\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x0e\x10\x11\n\n\n\
    \x02\x04\x02\x12\x04\x11\0\x17\x01\n\n\n\x03\x04\x02\x01\x12\x03\x11\x08\
    \x10\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x12\x02\x12\n\r\n\x05\x04\x02\x02\
    \0\x04\x12\x04\x12\x02\x11\x12\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x12\
    \x02\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x12\t\r\n\x0c\n\x05\x04\
    \x02\x02\0\x03\x12\x03\x12\x10\x11\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\
    \x13\x02\x15\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\x13\x02\x12\x12\n\x0c\
    \n\x05\x04\x02\x02\x01\x05\x12\x03\x13\x02\x06\n\x0c\n\x05\x04\x02\x02\
    \x01\x01\x12\x03\x13\x07\x10\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x13\
    \x13\x14\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x14\x02\x13\n\r\n\x05\x04\
    \x02\x02\x02\x04\x12\x04\x14\x02\x13\x15\n\x0c\n\x05\x04\x02\x02\x02\x05\
    \x12\x03\x14\x02\x08\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\x14\t\x0e\n\
    \x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x14\x11\x12\n\x0b\n\x04\x04\x02\
    \x02\x03\x12\x03\x15\x02\x1c\n\r\n\x05\x04\x02\x02\x03\x04\x12\x04\x15\
    \x02\x14\x13\n\x0c\n\x05\x04\x02\x02\x03\x05\x12\x03\x15\x02\x06\n\x0c\n\
    \x05\x04\x02\x02\x03\x01\x12\x03\x15\x07\x17\n\x0c\n\x05\x04\x02\x02\x03\
    \x03\x12\x03\x15\x1a\x1b\n\x0b\n\x04\x04\x02\x02\x04\x12\x03\x16\x02\x19\
    \n\r\n\x05\x04\x02\x02\x04\x04\x12\x04\x16\x02\x15\x1c\n\x0c\n\x05\x04\
    \x02\x02\x04\x05\x12\x03\x16\x02\x07\n\x0c\n\x05\x04\x02\x02\x04\x01\x12\
    \x03\x16\x08\x14\n\x0c\n\x05\x04\x02\x02\x04\x03\x12\x03\x16\x17\x18\n\n\
    \n\x02\x04\x03\x12\x04\x19\0\x1b\x01\n\n\n\x03\x04\x03\x01\x12\x03\x19\
    \x08\x15\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x1a\x02(\n\x0c\n\x05\x04\x03\
    \x02\0\x04\x12\x03\x1a\x02\n\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03\x1a\
    \x0b\x19\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x1a\x1a#\n\x0c\n\x05\x04\
    \x03\x02\0\x03\x12\x03\x1a&'\n\n\n\x02\x04\x04\x12\x04\x1d\0\x20\x01\n\n\
    \n\x03\x04\x04\x01\x12\x03\x1d\x08\x16\n\x0b\n\x04\x04\x04\x02\0\x12\x03\
    \x1e\x02\x10\n\r\n\x05\x04\x04\x02\0\x04\x12\x04\x1e\x02\x1d\x18\n\x0c\n\
    \x05\x04\x04\x02\0\x05\x12\x03\x1e\x02\x08\n\x0c\n\x05\x04\x04\x02\0\x01\
    \x12\x03\x1e\t\x0b\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03\x1e\x0e\x0f\n\
    \x0b\n\x04\x04\x04\x02\x01\x12\x03\x1f\x02\x18\n\r\n\x05\x04\x04\x02\x01\
    \x04\x12\x04\x1f\x02\x1e\x10\n\x0c\n\x05\x04\x04\x02\x01\x06\x12\x03\x1f\
    \x02\n\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03\x1f\x0b\x13\n\x0c\n\x05\
    \x04\x04\x02\x01\x03\x12\x03\x1f\x16\x17\n\n\n\x02\x04\x05\x12\x04\"\0$\
    \x01\n\n\n\x03\x04\x05\x01\x12\x03\"\x08\x10\n\x0b\n\x04\x04\x05\x02\0\
    \x12\x03#\x02\x20\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03#\x02\n\n\x0c\n\
    \x05\x04\x05\x02\0\x06\x12\x03#\x0b\x12\n\x0c\n\x05\x04\x05\x02\0\x01\
    \x12\x03#\x13\x1b\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03#\x1e\x1f\n\n\n\
    \x02\x04\x06\x12\x04&\0,\x01\n\n\n\x03\x04\x06\x01\x12\x03&\x08\x0f\n\
    \x0b\n\x04\x04\x06\x02\0\x12\x03'\x02\x12\n\r\n\x05\x04\x06\x02\0\x04\
    \x12\x04'\x02&\x11\n\x0c\n\x05\x04\x06\x02\0\x05\x12\x03'\x02\x08\n\x0c\
    \n\x05\x04\x06\x02\0\x01\x12\x03'\t\r\n\x0c\n\x05\x04\x06\x02\0\x03\x12\
    \x03'\x10\x11\n\x0b\n\x04\x04\x06\x02\x01\x12\x03(\x02$\n\x0c\n\x05\x04\
    \x06\x02\x01\x04\x12\x03(\x02\n\n\x0c\n\x05\x04\x06\x02\x01\x06\x12\x03(\
    \x0b\x14\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x03(\x15\x1f\n\x0c\n\x05\
    \x04\x06\x02\x01\x03\x12\x03(\"#\n\x0b\n\x04\x04\x06\x02\x02\x12\x03)\
    \x02\x1d\n\x0c\n\x05\x04\x06\x02\x02\x04\x12\x03)\x02\n\n\x0c\n\x05\x04\
    \x06\x02\x02\x06\x12\x03)\x0b\x10\n\x0c\n\x05\x04\x06\x02\x02\x01\x12\
    \x03)\x11\x18\n\x0c\n\x05\x04\x06\x02\x02\x03\x12\x03)\x1b\x1c\n\x0b\n\
    \x04\x04\x06\x02\x03\x12\x03*\x02(\n\x0c\n\x05\x04\x06\x02\x03\x04\x12\
    \x03*\x02\n\n\x0c\n\x05\x04\x06\x02\x03\x06\x12\x03*\x0b\x16\n\x0c\n\x05\
    \x04\x06\x02\x03\x01\x12\x03*\x17#\n\x0c\n\x05\x04\x06\x02\x03\x03\x12\
    \x03*&'\n\x0b\n\x04\x04\x06\x02\x04\x12\x03+\x02\x1b\n\r\n\x05\x04\x06\
    \x02\x04\x04\x12\x04+\x02*(\n\x0c\n\x05\x04\x06\x02\x04\x05\x12\x03+\x02\
    \x08\n\x0c\n\x05\x04\x06\x02\x04\x01\x12\x03+\t\x16\n\x0c\n\x05\x04\x06\
    \x02\x04\x03\x12\x03+\x19\x1a\n\n\n\x02\x04\x07\x12\x04.\05\x01\n\n\n\
    \x03\x04\x07\x01\x12\x03.\x08\x11\n\x0b\n\x04\x04\x07\x02\0\x12\x03/\x02\
    \x12\n\r\n\x05\x04\x07\x02\0\x04\x12\x04/\x02.\x13\n\x0c\n\x05\x04\x07\
    \x02\0\x05\x12\x03/\x02\x08\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03/\t\r\n\
    \x0c\n\x05\x04\x07\x02\0\x03\x12\x03/\x10\x11\n\x0b\n\x04\x04\x07\x02\
    \x01\x12\x030\x02$\n\x0c\n\x05\x04\x07\x02\x01\x04\x12\x030\x02\n\n\x0c\
    \n\x05\x04\x07\x02\x01\x06\x12\x030\x0b\x14\n\x0c\n\x05\x04\x07\x02\x01\
    \x01\x12\x030\x15\x1f\n\x0c\n\x05\x04\x07\x02\x01\x03\x12\x030\"#\n\x0b\
    \n\x04\x04\x07\x02\x02\x12\x031\x02\x1b\n\r\n\x05\x04\x07\x02\x02\x04\
    \x12\x041\x020$\n\x0c\n\x05\x04\x07\x02\x02\x05\x12\x031\x02\x06\n\x0c\n\
    \x05\x04\x07\x02\x02\x01\x12\x031\x07\x16\n\x0c\n\x05\x04\x07\x02\x02\
    \x03\x12\x031\x19\x1a\n\x0b\n\x04\x04\x07\x02\x03\x12\x032\x02\x19\n\r\n\
    \x05\x04\x07\x02\x03\x04\x12\x042\x021\x1b\n\x0c\n\x05\x04\x07\x02\x03\
    \x05\x12\x032\x02\x08\n\x0c\n\x05\x04\x07\x02\x03\x01\x12\x032\t\x14\n\
    \x0c\n\x05\x04\x07\x02\x03\x03\x12\x032\x17\x18\n\x0b\n\x04\x04\x07\x02\
    \x04\x12\x033\x02!\n\x0c\n\x05\x04\x07\x02\x04\x04\x12\x033\x02\n\n\x0c\
    \n\x05\x04\x07\x02\x04\x05\x12\x033\x0b\x11\n\x0c\n\x05\x04\x07\x02\x04\
    \x01\x12\x033\x12\x1c\n\x0c\n\x05\x04\x07\x02\x04\x03\x12\x033\x1f\x20\n\
    \x0b\n\x04\x04\x07\x02\x05\x12\x034\x02\x1b\n\r\n\x05\x04\x07\x02\x05\
    \x04\x12\x044\x023!\n\x0c\n\x05\x04\x07\x02\x05\x05\x12\x034\x02\x08\n\
    \x0c\n\x05\x04\x07\x02\x05\x01\x12\x034\t\x16\n\x0c\n\x05\x04\x07\x02\
    \x05\x03\x12\x034\x19\x1a\n\n\n\x02\x04\x08\x12\x047\0<\x01\n\n\n\x03\
    \x04\x08\x01\x12\x037\x08\x11\n\x0b\n\x04\x04\x08\x02\0\x12\x038\x02\x12\
    \n\r\n\x05\x04\x08\x02\0\x04\x12\x048\x027\x13\n\x0c\n\x05\x04\x08\x02\0\
    \x05\x12\x038\x02\x08\n\x0c\n\x05\x04\x08\x02\0\x01\x12\x038\t\r\n\x0c\n\
    \x05\x04\x08\x02\0\x03\x12\x038\x10\x11\n\x0b\n\x04\x04\x08\x02\x01\x12\
    \x039\x02\x12\n\r\n\x05\x04\x08\x02\x01\x04\x12\x049\x028\x12\n\x0c\n\
    \x05\x04\x08\x02\x01\x05\x12\x039\x02\x08\n\x0c\n\x05\x04\x08\x02\x01\
    \x01\x12\x039\t\r\n\x0c\n\x05\x04\x08\x02\x01\x03\x12\x039\x10\x11\n\x0b\
    \n\x04\x04\x08\x02\x02\x12\x03:\x02\x1d\n\r\n\x05\x04\x08\x02\x02\x04\
    \x12\x04:\x029\x12\n\x0c\n\x05\x04\x08\x02\x02\x05\x12\x03:\x02\x06\n\
    \x0c\n\x05\x04\x08\x02\x02\x01\x12\x03:\x07\x18\n\x0c\n\x05\x04\x08\x02\
    \x02\x03\x12\x03:\x1b\x1c\n\x0b\n\x04\x04\x08\x02\x03\x12\x03;\x02\x1a\n\
    \r\n\x05\x04\x08\x02\x03\x04\x12\x04;\x02:\x1d\n\x0c\n\x05\x04\x08\x02\
    \x03\x05\x12\x03;\x02\x07\n\x0c\n\x05\x04\x08\x02\x03\x01\x12\x03;\x08\
    \x15\n\x0c\n\x05\x04\x08\x02\x03\x03\x12\x03;\x18\x19\n\n\n\x02\x04\t\
    \x12\x04>\0A\x01\n\n\n\x03\x04\t\x01\x12\x03>\x08\r\n\x0b\n\x04\x04\t\
    \x02\0\x12\x03?\x02\x12\n\r\n\x05\x04\t\x02\0\x04\x12\x04?\x02>\x0f\n\
    \x0c\n\x05\x04\t\x02\0\x05\x12\x03?\x02\x08\n\x0c\n\x05\x04\t\x02\0\x01\
    \x12\x03?\t\r\n\x0c\n\x05\x04\t\x02\0\x03\x12\x03?\x10\x11\n\x0b\n\x04\
    \x04\t\x02\x01\x12\x03@\x02\x1b\n\r\n\x05\x04\t\x02\x01\x04\x12\x04@\x02\
    ?\x12\n\x0c\n\x05\x04\t\x02\x01\x05\x12\x03@\x02\x08\n\x0c\n\x05\x04\t\
    \x02\x01\x01\x12\x03@\t\x16\n\x0c\n\x05\x04\t\x02\x01\x03\x12\x03@\x19\
    \x1a\n\n\n\x02\x04\n\x12\x04C\0G\x01\n\n\n\x03\x04\n\x01\x12\x03C\x08\
    \x13\n\x0b\n\x04\x04\n\x02\0\x12\x03D\x02\x12\n\r\n\x05\x04\n\x02\0\x04\
    \x12\x04D\x02C\x15\n\x0c\n\x05\x04\n\x02\0\x05\x12\x03D\x02\x08\n\x0c\n\
    \x05\x04\n\x02\0\x01\x12\x03D\t\r\n\x0c\n\x05\x04\n\x02\0\x03\x12\x03D\
    \x10\x11\n\x0b\n\x04\x04\n\x02\x01\x12\x03E\x02'\n\x0c\n\x05\x04\n\x02\
    \x01\x04\x12\x03E\x02\n\n\x0c\n\x05\x04\n\x02\x01\x06\x12\x03E\x0b\x1b\n\
    \x0c\n\x05\x04\n\x02\x01\x01\x12\x03E\x1c\"\n\x0c\n\x05\x04\n\x02\x01\
    \x03\x12\x03E%&\n\x0b\n\x04\x04\n\x02\x02\x12\x03F\x02\x1b\n\r\n\x05\x04\
    \n\x02\x02\x04\x12\x04F\x02E'\n\x0c\n\x05\x04\n\x02\x02\x05\x12\x03F\x02\
    \x08\n\x0c\n\x05\x04\n\x02\x02\x01\x12\x03F\t\x16\n\x0c\n\x05\x04\n\x02\
    \x02\x03\x12\x03F\x19\x1a\n\n\n\x02\x04\x0b\x12\x04I\0M\x01\n\n\n\x03\
    \x04\x0b\x01\x12\x03I\x08\x18\n\x0b\n\x04\x04\x0b\x02\0\x12\x03J\x02\x12\
    \n\r\n\x05\x04\x0b\x02\0\x04\x12\x04J\x02I\x1a\n\x0c\n\x05\x04\x0b\x02\0\
    \x05\x12\x03J\x02\x08\n\x0c\n\x05\x04\x0b\x02\0\x01\x12\x03J\t\r\n\x0c\n\
    \x05\x04\x0b\x02\0\x03\x12\x03J\x10\x11\n\x0b\n\x04\x04\x0b\x02\x01\x12\
    \x03K\x02\x12\n\r\n\x05\x04\x0b\x02\x01\x04\x12\x04K\x02J\x12\n\x0c\n\
    \x05\x04\x0b\x02\x01\x05\x12\x03K\x02\x07\n\x0c\n\x05\x04\x0b\x02\x01\
    \x01\x12\x03K\x08\r\n\x0c\n\x05\x04\x0b\x02\x01\x03\x12\x03K\x10\x11\n\
    \x0b\n\x04\x04\x0b\x02\x02\x12\x03L\x02\x1b\n\r\n\x05\x04\x0b\x02\x02\
    \x04\x12\x04L\x02K\x12\n\x0c\n\x05\x04\x0b\x02\x02\x05\x12\x03L\x02\x08\
    \n\x0c\n\x05\x04\x0b\x02\x02\x01\x12\x03L\t\x16\n\x0c\n\x05\x04\x0b\x02\
    \x02\x03\x12\x03L\x19\x1a\n\n\n\x02\x04\x0c\x12\x04O\0Q\x01\n\n\n\x03\
    \x04\x0c\x01\x12\x03O\x08\x0c\n\x0b\n\x04\x04\x0c\x02\0\x12\x03P\x02\x1b\
    \n\x0c\n\x05\x04\x0c\x02\0\x04\x12\x03P\x02\n\n\x0c\n\x05\x04\x0c\x02\0\
    \x05\x12\x03P\x0b\x10\n\x0c\n\x05\x04\x0c\x02\0\x01\x12\x03P\x11\x16\n\
    \x0c\n\x05\x04\x0c\x02\0\x03\x12\x03P\x19\x1a\n\n\n\x02\x04\r\x12\x04S\0\
    U\x01\n\n\n\x03\x04\r\x01\x12\x03S\x08\x12\n\x0b\n\x04\x04\r\x02\0\x12\
    \x03T\x02'\n\x0c\n\x05\x04\r\x02\0\x04\x12\x03T\x02\n\n\x0c\n\x05\x04\r\
    \x02\0\x06\x12\x03T\x0b\x1a\n\x0c\n\x05\x04\r\x02\0\x01\x12\x03T\x1b\"\n\
    \x0c\n\x05\x04\r\x02\0\x03\x12\x03T%&\n\n\n\x02\x04\x0e\x12\x04W\0Z\x01\
    \n\n\n\x03\x04\x0e\x01\x12\x03W\x08\x17\n\x0b\n\x04\x04\x0e\x02\0\x12\
    \x03X\x02\x10\n\r\n\x05\x04\x0e\x02\0\x04\x12\x04X\x02W\x19\n\x0c\n\x05\
    \x04\x0e\x02\0\x05\x12\x03X\x02\x07\n\x0c\n\x05\x04\x0e\x02\0\x01\x12\
    \x03X\x08\x0b\n\x0c\n\x05\x04\x0e\x02\0\x03\x12\x03X\x0e\x0f\n\x0b\n\x04\
    \x04\x0e\x02\x01\x12\x03Y\x02\x12\n\r\n\x05\x04\x0e\x02\x01\x04\x12\x04Y\
    \x02X\x10\n\x0c\n\x05\x04\x0e\x02\x01\x05\x12\x03Y\x02\x07\n\x0c\n\x05\
    \x04\x0e\x02\x01\x01\x12\x03Y\x08\r\n\x0c\n\x05\x04\x0e\x02\x01\x03\x12\
    \x03Y\x10\x11\n\n\n\x02\x04\x0f\x12\x04\\\0^\x01\n\n\n\x03\x04\x0f\x01\
    \x12\x03\\\x08\x0b\n\x0b\n\x04\x04\x0f\x02\0\x12\x03]\x02\x1b\n\x0c\n\
    \x05\x04\x0f\x02\0\x04\x12\x03]\x02\n\n\x0c\n\x05\x04\x0f\x02\0\x05\x12\
    \x03]\x0b\x10\n\x0c\n\x05\x04\x0f\x02\0\x01\x12\x03]\x11\x16\n\x0c\n\x05\
    \x04\x0f\x02\0\x03\x12\x03]\x19\x1a\n\n\n\x02\x04\x10\x12\x04`\0b\x01\n\
    \n\n\x03\x04\x10\x01\x12\x03`\x08\r\n\x0b\n\x04\x04\x10\x02\0\x12\x03a\
    \x02\x1b\n\x0c\n\x05\x04\x10\x02\0\x04\x12\x03a\x02\n\n\x0c\n\x05\x04\
    \x10\x02\0\x05\x12\x03a\x0b\x10\n\x0c\n\x05\x04\x10\x02\0\x01\x12\x03a\
    \x11\x16\n\x0c\n\x05\x04\x10\x02\0\x03\x12\x03a\x19\x1a\n\n\n\x02\x04\
    \x11\x12\x04d\0x\x01\n\n\n\x03\x04\x11\x01\x12\x03d\x08\x0e\n\x0b\n\x04\
    \x04\x11\x02\0\x12\x03e\x02\x15\n\r\n\x05\x04\x11\x02\0\x04\x12\x04e\x02\
    d\x10\n\x0c\n\x05\x04\x11\x02\0\x05\x12\x03e\x02\x08\n\x0c\n\x05\x04\x11\
    \x02\0\x01\x12\x03e\t\x10\n\x0c\n\x05\x04\x11\x02\0\x03\x12\x03e\x13\x14\
    \n\x0b\n\x04\x04\x11\x02\x01\x12\x03f\x02\x18\n\r\n\x05\x04\x11\x02\x01\
    \x04\x12\x04f\x02e\x15\n\x0c\n\x05\x04\x11\x02\x01\x05\x12\x03f\x02\x08\
    \n\x0c\n\x05\x04\x11\x02\x01\x01\x12\x03f\t\x13\n\x0c\n\x05\x04\x11\x02\
    \x01\x03\x12\x03f\x16\x17\n\x0b\n\x04\x04\x11\x02\x02\x12\x03g\x02\x1b\n\
    \r\n\x05\x04\x11\x02\x02\x04\x12\x04g\x02f\x18\n\x0c\n\x05\x04\x11\x02\
    \x02\x05\x12\x03g\x02\x08\n\x0c\n\x05\x04\x11\x02\x02\x01\x12\x03g\t\x16\
    \n\x0c\n\x05\x04\x11\x02\x02\x03\x12\x03g\x19\x1a\n\x0b\n\x04\x04\x11\
    \x02\x03\x12\x03h\x02\x1c\n\r\n\x05\x04\x11\x02\x03\x04\x12\x04h\x02g\
    \x1b\n\x0c\n\x05\x04\x11\x02\x03\x05\x12\x03h\x02\x07\n\x0c\n\x05\x04\
    \x11\x02\x03\x01\x12\x03h\x08\x17\n\x0c\n\x05\x04\x11\x02\x03\x03\x12\
    \x03h\x1a\x1b\n\x0b\n\x04\x04\x11\x02\x04\x12\x03i\x02\x1f\n\r\n\x05\x04\
    \x11\x02\x04\x04\x12\x04i\x02h\x1c\n\x0c\n\x05\x04\x11\x02\x04\x05\x12\
    \x03i\x02\x07\n\x0c\n\x05\x04\x11\x02\x04\x01\x12\x03i\x08\x1a\n\x0c\n\
    \x05\x04\x11\x02\x04\x03\x12\x03i\x1d\x1e\n\x0b\n\x04\x04\x11\x02\x05\
    \x12\x03j\x02\x1b\n\r\n\x05\x04\x11\x02\x05\x04\x12\x04j\x02i\x1f\n\x0c\
    \n\x05\x04\x11\x02\x05\x05\x12\x03j\x02\x08\n\x0c\n\x05\x04\x11\x02\x05\
    \x01\x12\x03j\t\x16\n\x0c\n\x05\x04\x11\x02\x05\x03\x12\x03j\x19\x1a\n\
    \x0b\n\x04\x04\x11\x02\x06\x12\x03k\x02\x15\n\r\n\x05\x04\x11\x02\x06\
    \x04\x12\x04k\x02j\x1b\n\x0c\n\x05\x04\x11\x02\x06\x05\x12\x03k\x02\x07\
    \n\x0c\n\x05\x04\x11\x02\x06\x01\x12\x03k\x08\x10\n\x0c\n\x05\x04\x11\
    \x02\x06\x03\x12\x03k\x13\x14\n\x0b\n\x04\x04\x11\x02\x07\x12\x03l\x02\
    \x1e\n\r\n\x05\x04\x11\x02\x07\x04\x12\x04l\x02k\x15\n\x0c\n\x05\x04\x11\
    \x02\x07\x05\x12\x03l\x02\x06\n\x0c\n\x05\x04\x11\x02\x07\x01\x12\x03l\
    \x07\x19\n\x0c\n\x05\x04\x11\x02\x07\x03\x12\x03l\x1c\x1d\n\x0b\n\x04\
    \x04\x11\x02\x08\x12\x03m\x02!\n\r\n\x05\x04\x11\x02\x08\x04\x12\x04m\
    \x02l\x1e\n\x0c\n\x05\x04\x11\x02\x08\x05\x12\x03m\x02\x08\n\x0c\n\x05\
    \x04\x11\x02\x08\x01\x12\x03m\t\x1c\n\x0c\n\x05\x04\x11\x02\x08\x03\x12\
    \x03m\x1f\x20\n\x0b\n\x04\x04\x11\x02\t\x12\x03n\x02\"\n\r\n\x05\x04\x11\
    \x02\t\x04\x12\x04n\x02m!\n\x0c\n\x05\x04\x11\x02\t\x05\x12\x03n\x02\x06\
    \n\x0c\n\x05\x04\x11\x02\t\x01\x12\x03n\x07\x1c\n\x0c\n\x05\x04\x11\x02\
    \t\x03\x12\x03n\x1f!\n\x0b\n\x04\x04\x11\x02\n\x12\x03o\x02\x1a\n\r\n\
    \x05\x04\x11\x02\n\x04\x12\x04o\x02n\"\n\x0c\n\x05\x04\x11\x02\n\x05\x12\
    \x03o\x02\x06\n\x0c\n\x05\x04\x11\x02\n\x01\x12\x03o\x07\x14\n\x0c\n\x05\
    \x04\x11\x02\n\x03\x12\x03o\x17\x19\n\x0b\n\x04\x04\x11\x02\x0b\x12\x03p\
    \x02\x1b\n\r\n\x05\x04\x11\x02\x0b\x04\x12\x04p\x02o\x1a\n\x0c\n\x05\x04\
    \x11\x02\x0b\x05\x12\x03p\x02\x08\n\x0c\n\x05\x04\x11\x02\x0b\x01\x12\
    \x03p\t\x15\n\x0c\n\x05\x04\x11\x02\x0b\x03\x12\x03p\x18\x1a\n\x0b\n\x04\
    \x04\x11\x02\x0c\x12\x03q\x02!\n\r\n\x05\x04\x11\x02\x0c\x04\x12\x04q\
    \x02p\x1b\n\x0c\n\x05\x04\x11\x02\x0c\x05\x12\x03q\x02\x07\n\x0c\n\x05\
    \x04\x11\x02\x0c\x01\x12\x03q\x08\x1b\n\x0c\n\x05\x04\x11\x02\x0c\x03\
    \x12\x03q\x1e\x20\n\x0b\n\x04\x04\x11\x02\r\x12\x03r\x02&\n\r\n\x05\x04\
    \x11\x02\r\x04\x12\x04r\x02q!\n\x0c\n\x05\x04\x11\x02\r\x05\x12\x03r\x02\
    \x07\n\x0c\n\x05\x04\x11\x02\r\x01\x12\x03r\x08\x20\n\x0c\n\x05\x04\x11\
    \x02\r\x03\x12\x03r#%\n\x0b\n\x04\x04\x11\x02\x0e\x12\x03s\x02&\n\r\n\
    \x05\x04\x11\x02\x0e\x04\x12\x04s\x02r&\n\x0c\n\x05\x04\x11\x02\x0e\x05\
    \x12\x03s\x02\x07\n\x0c\n\x05\x04\x11\x02\x0e\x01\x12\x03s\x08\x20\n\x0c\
    \n\x05\x04\x11\x02\x0e\x03\x12\x03s#%\n\x0b\n\x04\x04\x11\x02\x0f\x12\
    \x03t\x02\x1a\n\r\n\x05\x04\x11\x02\x0f\x04\x12\x04t\x02s&\n\x0c\n\x05\
    \x04\x11\x02\x0f\x05\x12\x03t\x02\x08\n\x0c\n\x05\x04\x11\x02\x0f\x01\
    \x12\x03t\t\x14\n\x0c\n\x05\x04\x11\x02\x0f\x03\x12\x03t\x17\x19\n\x0b\n\
    \x04\x04\x11\x02\x10\x12\x03u\x02#\n\r\n\x05\x04\x11\x02\x10\x04\x12\x04\
    u\x02t\x1a\n\x0c\n\x05\x04\x11\x02\x10\x05\x12\x03u\x02\x08\n\x0c\n\x05\
    \x04\x11\x02\x10\x01\x12\x03u\t\x1d\n\x0c\n\x05\x04\x11\x02\x10\x03\x12\
    \x03u\x20\"\n\x0b\n\x04\x04\x11\x02\x11\x12\x03v\x02\x1d\n\r\n\x05\x04\
    \x11\x02\x11\x04\x12\x04v\x02u#\n\x0c\n\x05\x04\x11\x02\x11\x05\x12\x03v\
    \x02\x07\n\x0c\n\x05\x04\x11\x02\x11\x01\x12\x03v\x08\x17\n\x0c\n\x05\
    \x04\x11\x02\x11\x03\x12\x03v\x1a\x1c\n\x0b\n\x04\x04\x11\x02\x12\x12\
    \x03w\x02$\n\r\n\x05\x04\x11\x02\x12\x04\x12\x04w\x02v\x1d\n\x0c\n\x05\
    \x04\x11\x02\x12\x05\x12\x03w\x02\x07\n\x0c\n\x05\x04\x11\x02\x12\x01\
    \x12\x03w\x08\x1e\n\x0c\n\x05\x04\x11\x02\x12\x03\x12\x03w!#b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
