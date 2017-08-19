#[macro_export]
macro_rules! proto {
    ($s : expr) => { Some(::std::borrow::Cow::Borrowed($s)) }
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
        let mut request = Request {
           service : proto!($service),
           procedure : proto!($procedure),
           arguments : vec!()
       };
       for i in 0 .. $args.len() { request.arguments.push(Argument { position : Some(i as _), value : proto!(&$args[i]) }) }
       request
   }}
}

#[macro_export]
macro_rules! unwrap_response {
    ($response : expr) => {
        match $response {
            Response { has_error : None, return_value : Some(i), .. } |
            Response { has_error : Some(false), return_value : Some(i), .. } => Ok(i.to_vec()),
            Response { has_error : Some(true), error : Some(e), ..} => Err(TransceiverError::ResponseHasError(e.to_string())),
            _ => {Err(TransceiverError::ResponseHasError("EMPTY".to_owned()))}
        }
    }
}
