#[macro_export]
macro_rules! proto {
    ($s : expr) => { ::std::borrow::Cow::Borrowed($s) }
}

#[macro_export]
macro_rules! unwrap_ret {
    ($res : expr, $err : expr) => {
        match $res {
            Ok(i) => i,
            Err(e) => return Err($err(e))
        }
    }
}

#[macro_export]
macro_rules! to_bytes {
    ($e : expr, $v : expr) => { quick_protobuf::Writer::new($v).write_message($e) };
    (varint, $e : expr, $v : expr) => { quick_protobuf::Writer::new($v).write_varint($e) }
}

#[macro_export]
macro_rules! from_bytes {
    (varint32, $v : expr) => { quick_protobuf::BytesReader::from_bytes($v).read_varint32($v) };
    ($t : tt, $v : expr) => { $t::from_reader(&mut quick_protobuf::BytesReader::from_bytes($v), $v) }
}

#[macro_export]
macro_rules! make_request {
    ($service : expr, $procedure : expr, $args : expr) => {{
        let mut request : Request = Request::new();
        request.set_service($service);
        request.set_procedure($procedure);
        for i in 0 .. $args.len() {
            let mut arg = Argument::new();
            arg.set_position(i as u32);
            arg.set_value($args[i].clone());
            request.arguments.push(arg);
        }
        request
   }}
}

#[macro_export]
macro_rules! unwrap_response {
    ($response : expr) => {{
        if $response.has_error { Err(TransceiverError::ResponseHasError($response.error)) }
        else if $response.has_return_value { Ok(Some($response.return_value)) }
        else { Ok(None) }
    }}
}
