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
    // message oneof groups
    value: ::std::option::Option<Request_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request {}

#[derive(Clone,PartialEq)]
pub enum Request_oneof_value {
    echo(RequestEcho),
    flush(RequestFlush),
    info(RequestInfo),
    set_option(RequestSetOption),
    deliver_tx(RequestDeliverTx),
    check_tx(RequestCheckTx),
    commit(RequestCommit),
    query(RequestQuery),
    init_chain(RequestInitChain),
    begin_block(RequestBeginBlock),
    end_block(RequestEndBlock),
}

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

    // .types.RequestEcho echo = 1;

    pub fn clear_echo(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_echo(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::echo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_echo(&mut self, v: RequestEcho) {
        self.value = ::std::option::Option::Some(Request_oneof_value::echo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_echo(&mut self) -> &mut RequestEcho {
        if let ::std::option::Option::Some(Request_oneof_value::echo(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::echo(RequestEcho::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::echo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_echo(&mut self) -> RequestEcho {
        if self.has_echo() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::echo(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestEcho::new()
        }
    }

    pub fn get_echo(&self) -> &RequestEcho {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::echo(ref v)) => v,
            _ => RequestEcho::default_instance(),
        }
    }

    // .types.RequestFlush flush = 2;

    pub fn clear_flush(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_flush(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::flush(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_flush(&mut self, v: RequestFlush) {
        self.value = ::std::option::Option::Some(Request_oneof_value::flush(v))
    }

    // Mutable pointer to the field.
    pub fn mut_flush(&mut self) -> &mut RequestFlush {
        if let ::std::option::Option::Some(Request_oneof_value::flush(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::flush(RequestFlush::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::flush(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_flush(&mut self) -> RequestFlush {
        if self.has_flush() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::flush(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestFlush::new()
        }
    }

    pub fn get_flush(&self) -> &RequestFlush {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::flush(ref v)) => v,
            _ => RequestFlush::default_instance(),
        }
    }

    // .types.RequestInfo info = 3;

    pub fn clear_info(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_info(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::info(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: RequestInfo) {
        self.value = ::std::option::Option::Some(Request_oneof_value::info(v))
    }

    // Mutable pointer to the field.
    pub fn mut_info(&mut self) -> &mut RequestInfo {
        if let ::std::option::Option::Some(Request_oneof_value::info(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::info(RequestInfo::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::info(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_info(&mut self) -> RequestInfo {
        if self.has_info() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::info(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestInfo::new()
        }
    }

    pub fn get_info(&self) -> &RequestInfo {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::info(ref v)) => v,
            _ => RequestInfo::default_instance(),
        }
    }

    // .types.RequestSetOption set_option = 4;

    pub fn clear_set_option(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_set_option(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::set_option(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_option(&mut self, v: RequestSetOption) {
        self.value = ::std::option::Option::Some(Request_oneof_value::set_option(v))
    }

    // Mutable pointer to the field.
    pub fn mut_set_option(&mut self) -> &mut RequestSetOption {
        if let ::std::option::Option::Some(Request_oneof_value::set_option(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::set_option(RequestSetOption::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::set_option(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_option(&mut self) -> RequestSetOption {
        if self.has_set_option() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::set_option(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestSetOption::new()
        }
    }

    pub fn get_set_option(&self) -> &RequestSetOption {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::set_option(ref v)) => v,
            _ => RequestSetOption::default_instance(),
        }
    }

    // .types.RequestDeliverTx deliver_tx = 5;

    pub fn clear_deliver_tx(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_deliver_tx(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::deliver_tx(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_deliver_tx(&mut self, v: RequestDeliverTx) {
        self.value = ::std::option::Option::Some(Request_oneof_value::deliver_tx(v))
    }

    // Mutable pointer to the field.
    pub fn mut_deliver_tx(&mut self) -> &mut RequestDeliverTx {
        if let ::std::option::Option::Some(Request_oneof_value::deliver_tx(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::deliver_tx(RequestDeliverTx::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::deliver_tx(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_deliver_tx(&mut self) -> RequestDeliverTx {
        if self.has_deliver_tx() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::deliver_tx(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestDeliverTx::new()
        }
    }

    pub fn get_deliver_tx(&self) -> &RequestDeliverTx {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::deliver_tx(ref v)) => v,
            _ => RequestDeliverTx::default_instance(),
        }
    }

    // .types.RequestCheckTx check_tx = 6;

    pub fn clear_check_tx(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_check_tx(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::check_tx(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_check_tx(&mut self, v: RequestCheckTx) {
        self.value = ::std::option::Option::Some(Request_oneof_value::check_tx(v))
    }

    // Mutable pointer to the field.
    pub fn mut_check_tx(&mut self) -> &mut RequestCheckTx {
        if let ::std::option::Option::Some(Request_oneof_value::check_tx(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::check_tx(RequestCheckTx::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::check_tx(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_check_tx(&mut self) -> RequestCheckTx {
        if self.has_check_tx() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::check_tx(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestCheckTx::new()
        }
    }

    pub fn get_check_tx(&self) -> &RequestCheckTx {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::check_tx(ref v)) => v,
            _ => RequestCheckTx::default_instance(),
        }
    }

    // .types.RequestCommit commit = 7;

    pub fn clear_commit(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_commit(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::commit(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_commit(&mut self, v: RequestCommit) {
        self.value = ::std::option::Option::Some(Request_oneof_value::commit(v))
    }

    // Mutable pointer to the field.
    pub fn mut_commit(&mut self) -> &mut RequestCommit {
        if let ::std::option::Option::Some(Request_oneof_value::commit(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::commit(RequestCommit::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::commit(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_commit(&mut self) -> RequestCommit {
        if self.has_commit() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::commit(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestCommit::new()
        }
    }

    pub fn get_commit(&self) -> &RequestCommit {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::commit(ref v)) => v,
            _ => RequestCommit::default_instance(),
        }
    }

    // .types.RequestQuery query = 8;

    pub fn clear_query(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_query(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::query(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_query(&mut self, v: RequestQuery) {
        self.value = ::std::option::Option::Some(Request_oneof_value::query(v))
    }

    // Mutable pointer to the field.
    pub fn mut_query(&mut self) -> &mut RequestQuery {
        if let ::std::option::Option::Some(Request_oneof_value::query(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::query(RequestQuery::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::query(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_query(&mut self) -> RequestQuery {
        if self.has_query() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::query(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestQuery::new()
        }
    }

    pub fn get_query(&self) -> &RequestQuery {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::query(ref v)) => v,
            _ => RequestQuery::default_instance(),
        }
    }

    // .types.RequestInitChain init_chain = 9;

    pub fn clear_init_chain(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_init_chain(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::init_chain(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_init_chain(&mut self, v: RequestInitChain) {
        self.value = ::std::option::Option::Some(Request_oneof_value::init_chain(v))
    }

    // Mutable pointer to the field.
    pub fn mut_init_chain(&mut self) -> &mut RequestInitChain {
        if let ::std::option::Option::Some(Request_oneof_value::init_chain(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::init_chain(RequestInitChain::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::init_chain(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_init_chain(&mut self) -> RequestInitChain {
        if self.has_init_chain() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::init_chain(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestInitChain::new()
        }
    }

    pub fn get_init_chain(&self) -> &RequestInitChain {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::init_chain(ref v)) => v,
            _ => RequestInitChain::default_instance(),
        }
    }

    // .types.RequestBeginBlock begin_block = 10;

    pub fn clear_begin_block(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_begin_block(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::begin_block(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_begin_block(&mut self, v: RequestBeginBlock) {
        self.value = ::std::option::Option::Some(Request_oneof_value::begin_block(v))
    }

    // Mutable pointer to the field.
    pub fn mut_begin_block(&mut self) -> &mut RequestBeginBlock {
        if let ::std::option::Option::Some(Request_oneof_value::begin_block(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::begin_block(RequestBeginBlock::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::begin_block(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_begin_block(&mut self) -> RequestBeginBlock {
        if self.has_begin_block() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::begin_block(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestBeginBlock::new()
        }
    }

    pub fn get_begin_block(&self) -> &RequestBeginBlock {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::begin_block(ref v)) => v,
            _ => RequestBeginBlock::default_instance(),
        }
    }

    // .types.RequestEndBlock end_block = 11;

    pub fn clear_end_block(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_end_block(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::end_block(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_end_block(&mut self, v: RequestEndBlock) {
        self.value = ::std::option::Option::Some(Request_oneof_value::end_block(v))
    }

    // Mutable pointer to the field.
    pub fn mut_end_block(&mut self) -> &mut RequestEndBlock {
        if let ::std::option::Option::Some(Request_oneof_value::end_block(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::end_block(RequestEndBlock::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::end_block(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_end_block(&mut self) -> RequestEndBlock {
        if self.has_end_block() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::end_block(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestEndBlock::new()
        }
    }

    pub fn get_end_block(&self) -> &RequestEndBlock {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::end_block(ref v)) => v,
            _ => RequestEndBlock::default_instance(),
        }
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Request_oneof_value::echo(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Request_oneof_value::flush(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Request_oneof_value::info(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Request_oneof_value::set_option(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Request_oneof_value::deliver_tx(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Request_oneof_value::check_tx(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Request_oneof_value::commit(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Request_oneof_value::query(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Request_oneof_value::init_chain(is.read_message()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Request_oneof_value::begin_block(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Request_oneof_value::end_block(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &Request_oneof_value::echo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::flush(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::info(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::set_option(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::deliver_tx(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::check_tx(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::commit(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::query(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::init_chain(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::begin_block(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::end_block(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &Request_oneof_value::echo(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::flush(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::info(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::set_option(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::deliver_tx(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::check_tx(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::commit(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::query(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::init_chain(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::begin_block(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::end_block(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestEcho>(
                    "echo",
                    Request::has_echo,
                    Request::get_echo,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestFlush>(
                    "flush",
                    Request::has_flush,
                    Request::get_flush,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestInfo>(
                    "info",
                    Request::has_info,
                    Request::get_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestSetOption>(
                    "set_option",
                    Request::has_set_option,
                    Request::get_set_option,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestDeliverTx>(
                    "deliver_tx",
                    Request::has_deliver_tx,
                    Request::get_deliver_tx,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestCheckTx>(
                    "check_tx",
                    Request::has_check_tx,
                    Request::get_check_tx,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestCommit>(
                    "commit",
                    Request::has_commit,
                    Request::get_commit,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestQuery>(
                    "query",
                    Request::has_query,
                    Request::get_query,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestInitChain>(
                    "init_chain",
                    Request::has_init_chain,
                    Request::get_init_chain,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestBeginBlock>(
                    "begin_block",
                    Request::has_begin_block,
                    Request::get_begin_block,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestEndBlock>(
                    "end_block",
                    Request::has_end_block,
                    Request::get_end_block,
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
        self.clear_echo();
        self.clear_flush();
        self.clear_info();
        self.clear_set_option();
        self.clear_deliver_tx();
        self.clear_check_tx();
        self.clear_commit();
        self.clear_query();
        self.clear_init_chain();
        self.clear_begin_block();
        self.clear_end_block();
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
pub struct RequestEcho {
    // message fields
    pub message: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestEcho {}

impl RequestEcho {
    pub fn new() -> RequestEcho {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestEcho {
        static mut instance: ::protobuf::lazy::Lazy<RequestEcho> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestEcho,
        };
        unsafe {
            instance.get(RequestEcho::new)
        }
    }

    // string message = 1;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    fn get_message_for_reflect(&self) -> &::std::string::String {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }
}

impl ::protobuf::Message for RequestEcho {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message)?;
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
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.message);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.message.is_empty() {
            os.write_string(1, &self.message)?;
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

impl ::protobuf::MessageStatic for RequestEcho {
    fn new() -> RequestEcho {
        RequestEcho::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestEcho>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    RequestEcho::get_message_for_reflect,
                    RequestEcho::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestEcho>(
                    "RequestEcho",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestEcho {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestEcho {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestEcho {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestFlush {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestFlush {}

impl RequestFlush {
    pub fn new() -> RequestFlush {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestFlush {
        static mut instance: ::protobuf::lazy::Lazy<RequestFlush> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestFlush,
        };
        unsafe {
            instance.get(RequestFlush::new)
        }
    }
}

impl ::protobuf::Message for RequestFlush {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for RequestFlush {
    fn new() -> RequestFlush {
        RequestFlush::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestFlush>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RequestFlush>(
                    "RequestFlush",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestFlush {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestFlush {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestFlush {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestInfo {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestInfo {}

impl RequestInfo {
    pub fn new() -> RequestInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestInfo {
        static mut instance: ::protobuf::lazy::Lazy<RequestInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestInfo,
        };
        unsafe {
            instance.get(RequestInfo::new)
        }
    }
}

impl ::protobuf::Message for RequestInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for RequestInfo {
    fn new() -> RequestInfo {
        RequestInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RequestInfo>(
                    "RequestInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestInfo {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestSetOption {
    // message fields
    pub key: ::std::string::String,
    pub value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestSetOption {}

impl RequestSetOption {
    pub fn new() -> RequestSetOption {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestSetOption {
        static mut instance: ::protobuf::lazy::Lazy<RequestSetOption> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestSetOption,
        };
        unsafe {
            instance.get(RequestSetOption::new)
        }
    }

    // string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::string::String {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.value, ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::string::String {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }
}

impl ::protobuf::Message for RequestSetOption {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.value)?;
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
            my_size += ::protobuf::rt::string_size(1, &self.key);
        };
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_string(1, &self.key)?;
        };
        if !self.value.is_empty() {
            os.write_string(2, &self.value)?;
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

impl ::protobuf::MessageStatic for RequestSetOption {
    fn new() -> RequestSetOption {
        RequestSetOption::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestSetOption>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    RequestSetOption::get_key_for_reflect,
                    RequestSetOption::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    RequestSetOption::get_value_for_reflect,
                    RequestSetOption::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestSetOption>(
                    "RequestSetOption",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestSetOption {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestSetOption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestSetOption {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestDeliverTx {
    // message fields
    pub tx: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestDeliverTx {}

impl RequestDeliverTx {
    pub fn new() -> RequestDeliverTx {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestDeliverTx {
        static mut instance: ::protobuf::lazy::Lazy<RequestDeliverTx> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestDeliverTx,
        };
        unsafe {
            instance.get(RequestDeliverTx::new)
        }
    }

    // bytes tx = 1;

    pub fn clear_tx(&mut self) {
        self.tx.clear();
    }

    // Param is passed by value, moved
    pub fn set_tx(&mut self, v: ::std::vec::Vec<u8>) {
        self.tx = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tx(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx
    }

    // Take field
    pub fn take_tx(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.tx, ::std::vec::Vec::new())
    }

    pub fn get_tx(&self) -> &[u8] {
        &self.tx
    }

    fn get_tx_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.tx
    }

    fn mut_tx_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx
    }
}

impl ::protobuf::Message for RequestDeliverTx {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.tx)?;
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
        if !self.tx.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.tx);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.tx.is_empty() {
            os.write_bytes(1, &self.tx)?;
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

impl ::protobuf::MessageStatic for RequestDeliverTx {
    fn new() -> RequestDeliverTx {
        RequestDeliverTx::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestDeliverTx>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tx",
                    RequestDeliverTx::get_tx_for_reflect,
                    RequestDeliverTx::mut_tx_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestDeliverTx>(
                    "RequestDeliverTx",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestDeliverTx {
    fn clear(&mut self) {
        self.clear_tx();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestDeliverTx {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestDeliverTx {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestCheckTx {
    // message fields
    pub tx: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestCheckTx {}

impl RequestCheckTx {
    pub fn new() -> RequestCheckTx {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestCheckTx {
        static mut instance: ::protobuf::lazy::Lazy<RequestCheckTx> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestCheckTx,
        };
        unsafe {
            instance.get(RequestCheckTx::new)
        }
    }

    // bytes tx = 1;

    pub fn clear_tx(&mut self) {
        self.tx.clear();
    }

    // Param is passed by value, moved
    pub fn set_tx(&mut self, v: ::std::vec::Vec<u8>) {
        self.tx = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tx(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx
    }

    // Take field
    pub fn take_tx(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.tx, ::std::vec::Vec::new())
    }

    pub fn get_tx(&self) -> &[u8] {
        &self.tx
    }

    fn get_tx_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.tx
    }

    fn mut_tx_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx
    }
}

impl ::protobuf::Message for RequestCheckTx {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.tx)?;
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
        if !self.tx.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.tx);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.tx.is_empty() {
            os.write_bytes(1, &self.tx)?;
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

impl ::protobuf::MessageStatic for RequestCheckTx {
    fn new() -> RequestCheckTx {
        RequestCheckTx::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestCheckTx>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tx",
                    RequestCheckTx::get_tx_for_reflect,
                    RequestCheckTx::mut_tx_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestCheckTx>(
                    "RequestCheckTx",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestCheckTx {
    fn clear(&mut self) {
        self.clear_tx();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestCheckTx {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestCheckTx {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestQuery {
    // message fields
    pub data: ::std::vec::Vec<u8>,
    pub path: ::std::string::String,
    pub height: u64,
    pub prove: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestQuery {}

impl RequestQuery {
    pub fn new() -> RequestQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestQuery {
        static mut instance: ::protobuf::lazy::Lazy<RequestQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestQuery,
        };
        unsafe {
            instance.get(RequestQuery::new)
        }
    }

    // bytes data = 1;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // string path = 2;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.path, ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    fn get_path_for_reflect(&self) -> &::std::string::String {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // uint64 height = 3;

    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u64) {
        self.height = v;
    }

    pub fn get_height(&self) -> u64 {
        self.height
    }

    fn get_height_for_reflect(&self) -> &u64 {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut u64 {
        &mut self.height
    }

    // bool prove = 4;

    pub fn clear_prove(&mut self) {
        self.prove = false;
    }

    // Param is passed by value, moved
    pub fn set_prove(&mut self, v: bool) {
        self.prove = v;
    }

    pub fn get_prove(&self) -> bool {
        self.prove
    }

    fn get_prove_for_reflect(&self) -> &bool {
        &self.prove
    }

    fn mut_prove_for_reflect(&mut self) -> &mut bool {
        &mut self.prove
    }
}

impl ::protobuf::Message for RequestQuery {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.path)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.prove = tmp;
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
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.data);
        };
        if !self.path.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.path);
        };
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(3, self.height, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.prove != false {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.data.is_empty() {
            os.write_bytes(1, &self.data)?;
        };
        if !self.path.is_empty() {
            os.write_string(2, &self.path)?;
        };
        if self.height != 0 {
            os.write_uint64(3, self.height)?;
        };
        if self.prove != false {
            os.write_bool(4, self.prove)?;
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

impl ::protobuf::MessageStatic for RequestQuery {
    fn new() -> RequestQuery {
        RequestQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    RequestQuery::get_data_for_reflect,
                    RequestQuery::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    RequestQuery::get_path_for_reflect,
                    RequestQuery::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    RequestQuery::get_height_for_reflect,
                    RequestQuery::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "prove",
                    RequestQuery::get_prove_for_reflect,
                    RequestQuery::mut_prove_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestQuery>(
                    "RequestQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestQuery {
    fn clear(&mut self) {
        self.clear_data();
        self.clear_path();
        self.clear_height();
        self.clear_prove();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestQuery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestCommit {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestCommit {}

impl RequestCommit {
    pub fn new() -> RequestCommit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestCommit {
        static mut instance: ::protobuf::lazy::Lazy<RequestCommit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestCommit,
        };
        unsafe {
            instance.get(RequestCommit::new)
        }
    }
}

impl ::protobuf::Message for RequestCommit {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for RequestCommit {
    fn new() -> RequestCommit {
        RequestCommit::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestCommit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RequestCommit>(
                    "RequestCommit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestCommit {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestCommit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestCommit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestInitChain {
    // message fields
    validators: ::protobuf::RepeatedField<Validator>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestInitChain {}

impl RequestInitChain {
    pub fn new() -> RequestInitChain {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestInitChain {
        static mut instance: ::protobuf::lazy::Lazy<RequestInitChain> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestInitChain,
        };
        unsafe {
            instance.get(RequestInitChain::new)
        }
    }

    // repeated .types.Validator validators = 1;

    pub fn clear_validators(&mut self) {
        self.validators.clear();
    }

    // Param is passed by value, moved
    pub fn set_validators(&mut self, v: ::protobuf::RepeatedField<Validator>) {
        self.validators = v;
    }

    // Mutable pointer to the field.
    pub fn mut_validators(&mut self) -> &mut ::protobuf::RepeatedField<Validator> {
        &mut self.validators
    }

    // Take field
    pub fn take_validators(&mut self) -> ::protobuf::RepeatedField<Validator> {
        ::std::mem::replace(&mut self.validators, ::protobuf::RepeatedField::new())
    }

    pub fn get_validators(&self) -> &[Validator] {
        &self.validators
    }

    fn get_validators_for_reflect(&self) -> &::protobuf::RepeatedField<Validator> {
        &self.validators
    }

    fn mut_validators_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Validator> {
        &mut self.validators
    }
}

impl ::protobuf::Message for RequestInitChain {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.validators)?;
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
        for value in &self.validators {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.validators {
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

impl ::protobuf::MessageStatic for RequestInitChain {
    fn new() -> RequestInitChain {
        RequestInitChain::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestInitChain>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Validator>>(
                    "validators",
                    RequestInitChain::get_validators_for_reflect,
                    RequestInitChain::mut_validators_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestInitChain>(
                    "RequestInitChain",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestInitChain {
    fn clear(&mut self) {
        self.clear_validators();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestInitChain {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestInitChain {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestBeginBlock {
    // message fields
    pub hash: ::std::vec::Vec<u8>,
    header: ::protobuf::SingularPtrField<Header>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestBeginBlock {}

impl RequestBeginBlock {
    pub fn new() -> RequestBeginBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestBeginBlock {
        static mut instance: ::protobuf::lazy::Lazy<RequestBeginBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestBeginBlock,
        };
        unsafe {
            instance.get(RequestBeginBlock::new)
        }
    }

    // bytes hash = 1;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    fn get_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // .types.Header header = 2;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: Header) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut Header {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> Header {
        self.header.take().unwrap_or_else(|| Header::new())
    }

    pub fn get_header(&self) -> &Header {
        self.header.as_ref().unwrap_or_else(|| Header::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<Header> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Header> {
        &mut self.header
    }
}

impl ::protobuf::Message for RequestBeginBlock {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
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
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.hash);
        };
        if let Some(v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.hash.is_empty() {
            os.write_bytes(1, &self.hash)?;
        };
        if let Some(v) = self.header.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RequestBeginBlock {
    fn new() -> RequestBeginBlock {
        RequestBeginBlock::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestBeginBlock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    RequestBeginBlock::get_hash_for_reflect,
                    RequestBeginBlock::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Header>>(
                    "header",
                    RequestBeginBlock::get_header_for_reflect,
                    RequestBeginBlock::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestBeginBlock>(
                    "RequestBeginBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestBeginBlock {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestBeginBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestBeginBlock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestEndBlock {
    // message fields
    pub height: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestEndBlock {}

impl RequestEndBlock {
    pub fn new() -> RequestEndBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestEndBlock {
        static mut instance: ::protobuf::lazy::Lazy<RequestEndBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestEndBlock,
        };
        unsafe {
            instance.get(RequestEndBlock::new)
        }
    }

    // uint64 height = 1;

    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u64) {
        self.height = v;
    }

    pub fn get_height(&self) -> u64 {
        self.height
    }

    fn get_height_for_reflect(&self) -> &u64 {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut u64 {
        &mut self.height
    }
}

impl ::protobuf::Message for RequestEndBlock {
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
                    };
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
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
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(1, self.height, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.height != 0 {
            os.write_uint64(1, self.height)?;
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

impl ::protobuf::MessageStatic for RequestEndBlock {
    fn new() -> RequestEndBlock {
        RequestEndBlock::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestEndBlock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    RequestEndBlock::get_height_for_reflect,
                    RequestEndBlock::mut_height_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestEndBlock>(
                    "RequestEndBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestEndBlock {
    fn clear(&mut self) {
        self.clear_height();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestEndBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestEndBlock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message oneof groups
    value: ::std::option::Option<Response_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

#[derive(Clone,PartialEq)]
pub enum Response_oneof_value {
    exception(ResponseException),
    echo(ResponseEcho),
    flush(ResponseFlush),
    info(ResponseInfo),
    set_option(ResponseSetOption),
    deliver_tx(ResponseDeliverTx),
    check_tx(ResponseCheckTx),
    commit(ResponseCommit),
    query(ResponseQuery),
    init_chain(ResponseInitChain),
    begin_block(ResponseBeginBlock),
    end_block(ResponseEndBlock),
}

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

    // .types.ResponseException exception = 1;

    pub fn clear_exception(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_exception(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::exception(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_exception(&mut self, v: ResponseException) {
        self.value = ::std::option::Option::Some(Response_oneof_value::exception(v))
    }

    // Mutable pointer to the field.
    pub fn mut_exception(&mut self) -> &mut ResponseException {
        if let ::std::option::Option::Some(Response_oneof_value::exception(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::exception(ResponseException::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::exception(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_exception(&mut self) -> ResponseException {
        if self.has_exception() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::exception(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseException::new()
        }
    }

    pub fn get_exception(&self) -> &ResponseException {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::exception(ref v)) => v,
            _ => ResponseException::default_instance(),
        }
    }

    // .types.ResponseEcho echo = 2;

    pub fn clear_echo(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_echo(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::echo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_echo(&mut self, v: ResponseEcho) {
        self.value = ::std::option::Option::Some(Response_oneof_value::echo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_echo(&mut self) -> &mut ResponseEcho {
        if let ::std::option::Option::Some(Response_oneof_value::echo(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::echo(ResponseEcho::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::echo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_echo(&mut self) -> ResponseEcho {
        if self.has_echo() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::echo(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseEcho::new()
        }
    }

    pub fn get_echo(&self) -> &ResponseEcho {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::echo(ref v)) => v,
            _ => ResponseEcho::default_instance(),
        }
    }

    // .types.ResponseFlush flush = 3;

    pub fn clear_flush(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_flush(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::flush(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_flush(&mut self, v: ResponseFlush) {
        self.value = ::std::option::Option::Some(Response_oneof_value::flush(v))
    }

    // Mutable pointer to the field.
    pub fn mut_flush(&mut self) -> &mut ResponseFlush {
        if let ::std::option::Option::Some(Response_oneof_value::flush(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::flush(ResponseFlush::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::flush(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_flush(&mut self) -> ResponseFlush {
        if self.has_flush() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::flush(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseFlush::new()
        }
    }

    pub fn get_flush(&self) -> &ResponseFlush {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::flush(ref v)) => v,
            _ => ResponseFlush::default_instance(),
        }
    }

    // .types.ResponseInfo info = 4;

    pub fn clear_info(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_info(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::info(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: ResponseInfo) {
        self.value = ::std::option::Option::Some(Response_oneof_value::info(v))
    }

    // Mutable pointer to the field.
    pub fn mut_info(&mut self) -> &mut ResponseInfo {
        if let ::std::option::Option::Some(Response_oneof_value::info(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::info(ResponseInfo::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::info(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_info(&mut self) -> ResponseInfo {
        if self.has_info() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::info(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseInfo::new()
        }
    }

    pub fn get_info(&self) -> &ResponseInfo {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::info(ref v)) => v,
            _ => ResponseInfo::default_instance(),
        }
    }

    // .types.ResponseSetOption set_option = 5;

    pub fn clear_set_option(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_set_option(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::set_option(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_option(&mut self, v: ResponseSetOption) {
        self.value = ::std::option::Option::Some(Response_oneof_value::set_option(v))
    }

    // Mutable pointer to the field.
    pub fn mut_set_option(&mut self) -> &mut ResponseSetOption {
        if let ::std::option::Option::Some(Response_oneof_value::set_option(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::set_option(ResponseSetOption::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::set_option(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_option(&mut self) -> ResponseSetOption {
        if self.has_set_option() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::set_option(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseSetOption::new()
        }
    }

    pub fn get_set_option(&self) -> &ResponseSetOption {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::set_option(ref v)) => v,
            _ => ResponseSetOption::default_instance(),
        }
    }

    // .types.ResponseDeliverTx deliver_tx = 6;

    pub fn clear_deliver_tx(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_deliver_tx(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::deliver_tx(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_deliver_tx(&mut self, v: ResponseDeliverTx) {
        self.value = ::std::option::Option::Some(Response_oneof_value::deliver_tx(v))
    }

    // Mutable pointer to the field.
    pub fn mut_deliver_tx(&mut self) -> &mut ResponseDeliverTx {
        if let ::std::option::Option::Some(Response_oneof_value::deliver_tx(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::deliver_tx(ResponseDeliverTx::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::deliver_tx(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_deliver_tx(&mut self) -> ResponseDeliverTx {
        if self.has_deliver_tx() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::deliver_tx(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseDeliverTx::new()
        }
    }

    pub fn get_deliver_tx(&self) -> &ResponseDeliverTx {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::deliver_tx(ref v)) => v,
            _ => ResponseDeliverTx::default_instance(),
        }
    }

    // .types.ResponseCheckTx check_tx = 7;

    pub fn clear_check_tx(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_check_tx(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::check_tx(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_check_tx(&mut self, v: ResponseCheckTx) {
        self.value = ::std::option::Option::Some(Response_oneof_value::check_tx(v))
    }

    // Mutable pointer to the field.
    pub fn mut_check_tx(&mut self) -> &mut ResponseCheckTx {
        if let ::std::option::Option::Some(Response_oneof_value::check_tx(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::check_tx(ResponseCheckTx::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::check_tx(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_check_tx(&mut self) -> ResponseCheckTx {
        if self.has_check_tx() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::check_tx(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseCheckTx::new()
        }
    }

    pub fn get_check_tx(&self) -> &ResponseCheckTx {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::check_tx(ref v)) => v,
            _ => ResponseCheckTx::default_instance(),
        }
    }

    // .types.ResponseCommit commit = 8;

    pub fn clear_commit(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_commit(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::commit(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_commit(&mut self, v: ResponseCommit) {
        self.value = ::std::option::Option::Some(Response_oneof_value::commit(v))
    }

    // Mutable pointer to the field.
    pub fn mut_commit(&mut self) -> &mut ResponseCommit {
        if let ::std::option::Option::Some(Response_oneof_value::commit(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::commit(ResponseCommit::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::commit(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_commit(&mut self) -> ResponseCommit {
        if self.has_commit() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::commit(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseCommit::new()
        }
    }

    pub fn get_commit(&self) -> &ResponseCommit {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::commit(ref v)) => v,
            _ => ResponseCommit::default_instance(),
        }
    }

    // .types.ResponseQuery query = 9;

    pub fn clear_query(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_query(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::query(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_query(&mut self, v: ResponseQuery) {
        self.value = ::std::option::Option::Some(Response_oneof_value::query(v))
    }

    // Mutable pointer to the field.
    pub fn mut_query(&mut self) -> &mut ResponseQuery {
        if let ::std::option::Option::Some(Response_oneof_value::query(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::query(ResponseQuery::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::query(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_query(&mut self) -> ResponseQuery {
        if self.has_query() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::query(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseQuery::new()
        }
    }

    pub fn get_query(&self) -> &ResponseQuery {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::query(ref v)) => v,
            _ => ResponseQuery::default_instance(),
        }
    }

    // .types.ResponseInitChain init_chain = 10;

    pub fn clear_init_chain(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_init_chain(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::init_chain(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_init_chain(&mut self, v: ResponseInitChain) {
        self.value = ::std::option::Option::Some(Response_oneof_value::init_chain(v))
    }

    // Mutable pointer to the field.
    pub fn mut_init_chain(&mut self) -> &mut ResponseInitChain {
        if let ::std::option::Option::Some(Response_oneof_value::init_chain(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::init_chain(ResponseInitChain::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::init_chain(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_init_chain(&mut self) -> ResponseInitChain {
        if self.has_init_chain() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::init_chain(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseInitChain::new()
        }
    }

    pub fn get_init_chain(&self) -> &ResponseInitChain {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::init_chain(ref v)) => v,
            _ => ResponseInitChain::default_instance(),
        }
    }

    // .types.ResponseBeginBlock begin_block = 11;

    pub fn clear_begin_block(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_begin_block(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::begin_block(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_begin_block(&mut self, v: ResponseBeginBlock) {
        self.value = ::std::option::Option::Some(Response_oneof_value::begin_block(v))
    }

    // Mutable pointer to the field.
    pub fn mut_begin_block(&mut self) -> &mut ResponseBeginBlock {
        if let ::std::option::Option::Some(Response_oneof_value::begin_block(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::begin_block(ResponseBeginBlock::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::begin_block(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_begin_block(&mut self) -> ResponseBeginBlock {
        if self.has_begin_block() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::begin_block(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseBeginBlock::new()
        }
    }

    pub fn get_begin_block(&self) -> &ResponseBeginBlock {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::begin_block(ref v)) => v,
            _ => ResponseBeginBlock::default_instance(),
        }
    }

    // .types.ResponseEndBlock end_block = 12;

    pub fn clear_end_block(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_end_block(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::end_block(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_end_block(&mut self, v: ResponseEndBlock) {
        self.value = ::std::option::Option::Some(Response_oneof_value::end_block(v))
    }

    // Mutable pointer to the field.
    pub fn mut_end_block(&mut self) -> &mut ResponseEndBlock {
        if let ::std::option::Option::Some(Response_oneof_value::end_block(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::end_block(ResponseEndBlock::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::end_block(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_end_block(&mut self) -> ResponseEndBlock {
        if self.has_end_block() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::end_block(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseEndBlock::new()
        }
    }

    pub fn get_end_block(&self) -> &ResponseEndBlock {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::end_block(ref v)) => v,
            _ => ResponseEndBlock::default_instance(),
        }
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Response_oneof_value::exception(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Response_oneof_value::echo(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Response_oneof_value::flush(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Response_oneof_value::info(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Response_oneof_value::set_option(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Response_oneof_value::deliver_tx(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Response_oneof_value::check_tx(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Response_oneof_value::commit(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Response_oneof_value::query(is.read_message()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Response_oneof_value::init_chain(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Response_oneof_value::begin_block(is.read_message()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Response_oneof_value::end_block(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &Response_oneof_value::exception(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::echo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::flush(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::info(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::set_option(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::deliver_tx(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::check_tx(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::commit(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::query(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::init_chain(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::begin_block(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::end_block(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &Response_oneof_value::exception(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::echo(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::flush(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::info(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::set_option(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::deliver_tx(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::check_tx(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::commit(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::query(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::init_chain(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::begin_block(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::end_block(ref v) => {
                    os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseException>(
                    "exception",
                    Response::has_exception,
                    Response::get_exception,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseEcho>(
                    "echo",
                    Response::has_echo,
                    Response::get_echo,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseFlush>(
                    "flush",
                    Response::has_flush,
                    Response::get_flush,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseInfo>(
                    "info",
                    Response::has_info,
                    Response::get_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseSetOption>(
                    "set_option",
                    Response::has_set_option,
                    Response::get_set_option,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseDeliverTx>(
                    "deliver_tx",
                    Response::has_deliver_tx,
                    Response::get_deliver_tx,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseCheckTx>(
                    "check_tx",
                    Response::has_check_tx,
                    Response::get_check_tx,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseCommit>(
                    "commit",
                    Response::has_commit,
                    Response::get_commit,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseQuery>(
                    "query",
                    Response::has_query,
                    Response::get_query,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseInitChain>(
                    "init_chain",
                    Response::has_init_chain,
                    Response::get_init_chain,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseBeginBlock>(
                    "begin_block",
                    Response::has_begin_block,
                    Response::get_begin_block,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseEndBlock>(
                    "end_block",
                    Response::has_end_block,
                    Response::get_end_block,
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
        self.clear_exception();
        self.clear_echo();
        self.clear_flush();
        self.clear_info();
        self.clear_set_option();
        self.clear_deliver_tx();
        self.clear_check_tx();
        self.clear_commit();
        self.clear_query();
        self.clear_init_chain();
        self.clear_begin_block();
        self.clear_end_block();
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
pub struct ResponseException {
    // message fields
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseException {}

impl ResponseException {
    pub fn new() -> ResponseException {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseException {
        static mut instance: ::protobuf::lazy::Lazy<ResponseException> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseException,
        };
        unsafe {
            instance.get(ResponseException::new)
        }
    }

    // string error = 1;

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
}

impl ::protobuf::Message for ResponseException {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.error);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.error.is_empty() {
            os.write_string(1, &self.error)?;
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

impl ::protobuf::MessageStatic for ResponseException {
    fn new() -> ResponseException {
        ResponseException::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseException>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    ResponseException::get_error_for_reflect,
                    ResponseException::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseException>(
                    "ResponseException",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseException {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseException {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseException {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseEcho {
    // message fields
    pub message: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseEcho {}

impl ResponseEcho {
    pub fn new() -> ResponseEcho {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseEcho {
        static mut instance: ::protobuf::lazy::Lazy<ResponseEcho> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseEcho,
        };
        unsafe {
            instance.get(ResponseEcho::new)
        }
    }

    // string message = 1;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    fn get_message_for_reflect(&self) -> &::std::string::String {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }
}

impl ::protobuf::Message for ResponseEcho {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message)?;
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
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.message);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.message.is_empty() {
            os.write_string(1, &self.message)?;
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

impl ::protobuf::MessageStatic for ResponseEcho {
    fn new() -> ResponseEcho {
        ResponseEcho::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseEcho>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    ResponseEcho::get_message_for_reflect,
                    ResponseEcho::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseEcho>(
                    "ResponseEcho",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseEcho {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseEcho {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseEcho {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseFlush {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseFlush {}

impl ResponseFlush {
    pub fn new() -> ResponseFlush {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseFlush {
        static mut instance: ::protobuf::lazy::Lazy<ResponseFlush> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseFlush,
        };
        unsafe {
            instance.get(ResponseFlush::new)
        }
    }
}

impl ::protobuf::Message for ResponseFlush {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for ResponseFlush {
    fn new() -> ResponseFlush {
        ResponseFlush::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseFlush>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ResponseFlush>(
                    "ResponseFlush",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseFlush {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseFlush {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseFlush {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseInfo {
    // message fields
    pub data: ::std::string::String,
    pub version: ::std::string::String,
    pub last_block_height: u64,
    pub last_block_app_hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseInfo {}

impl ResponseInfo {
    pub fn new() -> ResponseInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseInfo {
        static mut instance: ::protobuf::lazy::Lazy<ResponseInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseInfo,
        };
        unsafe {
            instance.get(ResponseInfo::new)
        }
    }

    // string data = 1;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::string::String) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::string::String {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.data, ::std::string::String::new())
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::string::String {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.data
    }

    // string version = 2;

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

    // uint64 last_block_height = 3;

    pub fn clear_last_block_height(&mut self) {
        self.last_block_height = 0;
    }

    // Param is passed by value, moved
    pub fn set_last_block_height(&mut self, v: u64) {
        self.last_block_height = v;
    }

    pub fn get_last_block_height(&self) -> u64 {
        self.last_block_height
    }

    fn get_last_block_height_for_reflect(&self) -> &u64 {
        &self.last_block_height
    }

    fn mut_last_block_height_for_reflect(&mut self) -> &mut u64 {
        &mut self.last_block_height
    }

    // bytes last_block_app_hash = 4;

    pub fn clear_last_block_app_hash(&mut self) {
        self.last_block_app_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_last_block_app_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.last_block_app_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_block_app_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.last_block_app_hash
    }

    // Take field
    pub fn take_last_block_app_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.last_block_app_hash, ::std::vec::Vec::new())
    }

    pub fn get_last_block_app_hash(&self) -> &[u8] {
        &self.last_block_app_hash
    }

    fn get_last_block_app_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.last_block_app_hash
    }

    fn mut_last_block_app_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.last_block_app_hash
    }
}

impl ::protobuf::Message for ResponseInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.data)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.version)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.last_block_height = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.last_block_app_hash)?;
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
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.data);
        };
        if !self.version.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.version);
        };
        if self.last_block_height != 0 {
            my_size += ::protobuf::rt::value_size(3, self.last_block_height, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.last_block_app_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.last_block_app_hash);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.data.is_empty() {
            os.write_string(1, &self.data)?;
        };
        if !self.version.is_empty() {
            os.write_string(2, &self.version)?;
        };
        if self.last_block_height != 0 {
            os.write_uint64(3, self.last_block_height)?;
        };
        if !self.last_block_app_hash.is_empty() {
            os.write_bytes(4, &self.last_block_app_hash)?;
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

impl ::protobuf::MessageStatic for ResponseInfo {
    fn new() -> ResponseInfo {
        ResponseInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "data",
                    ResponseInfo::get_data_for_reflect,
                    ResponseInfo::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    ResponseInfo::get_version_for_reflect,
                    ResponseInfo::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "last_block_height",
                    ResponseInfo::get_last_block_height_for_reflect,
                    ResponseInfo::mut_last_block_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "last_block_app_hash",
                    ResponseInfo::get_last_block_app_hash_for_reflect,
                    ResponseInfo::mut_last_block_app_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseInfo>(
                    "ResponseInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseInfo {
    fn clear(&mut self) {
        self.clear_data();
        self.clear_version();
        self.clear_last_block_height();
        self.clear_last_block_app_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseSetOption {
    // message fields
    pub log: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseSetOption {}

impl ResponseSetOption {
    pub fn new() -> ResponseSetOption {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseSetOption {
        static mut instance: ::protobuf::lazy::Lazy<ResponseSetOption> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseSetOption,
        };
        unsafe {
            instance.get(ResponseSetOption::new)
        }
    }

    // string log = 1;

    pub fn clear_log(&mut self) {
        self.log.clear();
    }

    // Param is passed by value, moved
    pub fn set_log(&mut self, v: ::std::string::String) {
        self.log = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }

    // Take field
    pub fn take_log(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.log, ::std::string::String::new())
    }

    pub fn get_log(&self) -> &str {
        &self.log
    }

    fn get_log_for_reflect(&self) -> &::std::string::String {
        &self.log
    }

    fn mut_log_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }
}

impl ::protobuf::Message for ResponseSetOption {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.log)?;
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
        if !self.log.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.log);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.log.is_empty() {
            os.write_string(1, &self.log)?;
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

impl ::protobuf::MessageStatic for ResponseSetOption {
    fn new() -> ResponseSetOption {
        ResponseSetOption::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseSetOption>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "log",
                    ResponseSetOption::get_log_for_reflect,
                    ResponseSetOption::mut_log_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseSetOption>(
                    "ResponseSetOption",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseSetOption {
    fn clear(&mut self) {
        self.clear_log();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseSetOption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseSetOption {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseDeliverTx {
    // message fields
    pub code: CodeType,
    pub data: ::std::vec::Vec<u8>,
    pub log: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseDeliverTx {}

impl ResponseDeliverTx {
    pub fn new() -> ResponseDeliverTx {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseDeliverTx {
        static mut instance: ::protobuf::lazy::Lazy<ResponseDeliverTx> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseDeliverTx,
        };
        unsafe {
            instance.get(ResponseDeliverTx::new)
        }
    }

    // .types.CodeType code = 1;

    pub fn clear_code(&mut self) {
        self.code = CodeType::OK;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: CodeType) {
        self.code = v;
    }

    pub fn get_code(&self) -> CodeType {
        self.code
    }

    fn get_code_for_reflect(&self) -> &CodeType {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut CodeType {
        &mut self.code
    }

    // bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // string log = 3;

    pub fn clear_log(&mut self) {
        self.log.clear();
    }

    // Param is passed by value, moved
    pub fn set_log(&mut self, v: ::std::string::String) {
        self.log = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }

    // Take field
    pub fn take_log(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.log, ::std::string::String::new())
    }

    pub fn get_log(&self) -> &str {
        &self.log
    }

    fn get_log_for_reflect(&self) -> &::std::string::String {
        &self.log
    }

    fn mut_log_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }
}

impl ::protobuf::Message for ResponseDeliverTx {
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
                    };
                    let tmp = is.read_enum()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.log)?;
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
        if self.code != CodeType::OK {
            my_size += ::protobuf::rt::enum_size(1, self.code);
        };
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        };
        if !self.log.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.log);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != CodeType::OK {
            os.write_enum(1, self.code.value())?;
        };
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
        };
        if !self.log.is_empty() {
            os.write_string(3, &self.log)?;
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

impl ::protobuf::MessageStatic for ResponseDeliverTx {
    fn new() -> ResponseDeliverTx {
        ResponseDeliverTx::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseDeliverTx>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CodeType>>(
                    "code",
                    ResponseDeliverTx::get_code_for_reflect,
                    ResponseDeliverTx::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    ResponseDeliverTx::get_data_for_reflect,
                    ResponseDeliverTx::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "log",
                    ResponseDeliverTx::get_log_for_reflect,
                    ResponseDeliverTx::mut_log_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseDeliverTx>(
                    "ResponseDeliverTx",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseDeliverTx {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_data();
        self.clear_log();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseDeliverTx {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseDeliverTx {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseCheckTx {
    // message fields
    pub code: CodeType,
    pub data: ::std::vec::Vec<u8>,
    pub log: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseCheckTx {}

impl ResponseCheckTx {
    pub fn new() -> ResponseCheckTx {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseCheckTx {
        static mut instance: ::protobuf::lazy::Lazy<ResponseCheckTx> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseCheckTx,
        };
        unsafe {
            instance.get(ResponseCheckTx::new)
        }
    }

    // .types.CodeType code = 1;

    pub fn clear_code(&mut self) {
        self.code = CodeType::OK;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: CodeType) {
        self.code = v;
    }

    pub fn get_code(&self) -> CodeType {
        self.code
    }

    fn get_code_for_reflect(&self) -> &CodeType {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut CodeType {
        &mut self.code
    }

    // bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // string log = 3;

    pub fn clear_log(&mut self) {
        self.log.clear();
    }

    // Param is passed by value, moved
    pub fn set_log(&mut self, v: ::std::string::String) {
        self.log = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }

    // Take field
    pub fn take_log(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.log, ::std::string::String::new())
    }

    pub fn get_log(&self) -> &str {
        &self.log
    }

    fn get_log_for_reflect(&self) -> &::std::string::String {
        &self.log
    }

    fn mut_log_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }
}

impl ::protobuf::Message for ResponseCheckTx {
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
                    };
                    let tmp = is.read_enum()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.log)?;
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
        if self.code != CodeType::OK {
            my_size += ::protobuf::rt::enum_size(1, self.code);
        };
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        };
        if !self.log.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.log);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != CodeType::OK {
            os.write_enum(1, self.code.value())?;
        };
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
        };
        if !self.log.is_empty() {
            os.write_string(3, &self.log)?;
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

impl ::protobuf::MessageStatic for ResponseCheckTx {
    fn new() -> ResponseCheckTx {
        ResponseCheckTx::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseCheckTx>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CodeType>>(
                    "code",
                    ResponseCheckTx::get_code_for_reflect,
                    ResponseCheckTx::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    ResponseCheckTx::get_data_for_reflect,
                    ResponseCheckTx::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "log",
                    ResponseCheckTx::get_log_for_reflect,
                    ResponseCheckTx::mut_log_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseCheckTx>(
                    "ResponseCheckTx",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseCheckTx {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_data();
        self.clear_log();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseCheckTx {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseCheckTx {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseQuery {
    // message fields
    pub code: CodeType,
    pub index: i64,
    pub key: ::std::vec::Vec<u8>,
    pub value: ::std::vec::Vec<u8>,
    pub proof: ::std::vec::Vec<u8>,
    pub height: u64,
    pub log: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseQuery {}

impl ResponseQuery {
    pub fn new() -> ResponseQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseQuery {
        static mut instance: ::protobuf::lazy::Lazy<ResponseQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseQuery,
        };
        unsafe {
            instance.get(ResponseQuery::new)
        }
    }

    // .types.CodeType code = 1;

    pub fn clear_code(&mut self) {
        self.code = CodeType::OK;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: CodeType) {
        self.code = v;
    }

    pub fn get_code(&self) -> CodeType {
        self.code
    }

    fn get_code_for_reflect(&self) -> &CodeType {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut CodeType {
        &mut self.code
    }

    // int64 index = 2;

    pub fn clear_index(&mut self) {
        self.index = 0;
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: i64) {
        self.index = v;
    }

    pub fn get_index(&self) -> i64 {
        self.index
    }

    fn get_index_for_reflect(&self) -> &i64 {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut i64 {
        &mut self.index
    }

    // bytes key = 3;

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

    // bytes value = 4;

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

    // bytes proof = 5;

    pub fn clear_proof(&mut self) {
        self.proof.clear();
    }

    // Param is passed by value, moved
    pub fn set_proof(&mut self, v: ::std::vec::Vec<u8>) {
        self.proof = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_proof(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.proof
    }

    // Take field
    pub fn take_proof(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.proof, ::std::vec::Vec::new())
    }

    pub fn get_proof(&self) -> &[u8] {
        &self.proof
    }

    fn get_proof_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.proof
    }

    fn mut_proof_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.proof
    }

    // uint64 height = 6;

    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u64) {
        self.height = v;
    }

    pub fn get_height(&self) -> u64 {
        self.height
    }

    fn get_height_for_reflect(&self) -> &u64 {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut u64 {
        &mut self.height
    }

    // string log = 7;

    pub fn clear_log(&mut self) {
        self.log.clear();
    }

    // Param is passed by value, moved
    pub fn set_log(&mut self, v: ::std::string::String) {
        self.log = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }

    // Take field
    pub fn take_log(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.log, ::std::string::String::new())
    }

    pub fn get_log(&self) -> &str {
        &self.log
    }

    fn get_log_for_reflect(&self) -> &::std::string::String {
        &self.log
    }

    fn mut_log_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }
}

impl ::protobuf::Message for ResponseQuery {
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
                    };
                    let tmp = is.read_enum()?;
                    self.code = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.index = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.proof)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.log)?;
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
        if self.code != CodeType::OK {
            my_size += ::protobuf::rt::enum_size(1, self.code);
        };
        if self.index != 0 {
            my_size += ::protobuf::rt::value_size(2, self.index, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.key);
        };
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.value);
        };
        if !self.proof.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.proof);
        };
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(6, self.height, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.log.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.log);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != CodeType::OK {
            os.write_enum(1, self.code.value())?;
        };
        if self.index != 0 {
            os.write_int64(2, self.index)?;
        };
        if !self.key.is_empty() {
            os.write_bytes(3, &self.key)?;
        };
        if !self.value.is_empty() {
            os.write_bytes(4, &self.value)?;
        };
        if !self.proof.is_empty() {
            os.write_bytes(5, &self.proof)?;
        };
        if self.height != 0 {
            os.write_uint64(6, self.height)?;
        };
        if !self.log.is_empty() {
            os.write_string(7, &self.log)?;
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

impl ::protobuf::MessageStatic for ResponseQuery {
    fn new() -> ResponseQuery {
        ResponseQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CodeType>>(
                    "code",
                    ResponseQuery::get_code_for_reflect,
                    ResponseQuery::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "index",
                    ResponseQuery::get_index_for_reflect,
                    ResponseQuery::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    ResponseQuery::get_key_for_reflect,
                    ResponseQuery::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    ResponseQuery::get_value_for_reflect,
                    ResponseQuery::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "proof",
                    ResponseQuery::get_proof_for_reflect,
                    ResponseQuery::mut_proof_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    ResponseQuery::get_height_for_reflect,
                    ResponseQuery::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "log",
                    ResponseQuery::get_log_for_reflect,
                    ResponseQuery::mut_log_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseQuery>(
                    "ResponseQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseQuery {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_index();
        self.clear_key();
        self.clear_value();
        self.clear_proof();
        self.clear_height();
        self.clear_log();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseQuery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseCommit {
    // message fields
    pub code: CodeType,
    pub data: ::std::vec::Vec<u8>,
    pub log: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseCommit {}

impl ResponseCommit {
    pub fn new() -> ResponseCommit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseCommit {
        static mut instance: ::protobuf::lazy::Lazy<ResponseCommit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseCommit,
        };
        unsafe {
            instance.get(ResponseCommit::new)
        }
    }

    // .types.CodeType code = 1;

    pub fn clear_code(&mut self) {
        self.code = CodeType::OK;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: CodeType) {
        self.code = v;
    }

    pub fn get_code(&self) -> CodeType {
        self.code
    }

    fn get_code_for_reflect(&self) -> &CodeType {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut CodeType {
        &mut self.code
    }

    // bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // string log = 3;

    pub fn clear_log(&mut self) {
        self.log.clear();
    }

    // Param is passed by value, moved
    pub fn set_log(&mut self, v: ::std::string::String) {
        self.log = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }

    // Take field
    pub fn take_log(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.log, ::std::string::String::new())
    }

    pub fn get_log(&self) -> &str {
        &self.log
    }

    fn get_log_for_reflect(&self) -> &::std::string::String {
        &self.log
    }

    fn mut_log_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }
}

impl ::protobuf::Message for ResponseCommit {
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
                    };
                    let tmp = is.read_enum()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.log)?;
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
        if self.code != CodeType::OK {
            my_size += ::protobuf::rt::enum_size(1, self.code);
        };
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        };
        if !self.log.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.log);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != CodeType::OK {
            os.write_enum(1, self.code.value())?;
        };
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
        };
        if !self.log.is_empty() {
            os.write_string(3, &self.log)?;
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

impl ::protobuf::MessageStatic for ResponseCommit {
    fn new() -> ResponseCommit {
        ResponseCommit::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseCommit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CodeType>>(
                    "code",
                    ResponseCommit::get_code_for_reflect,
                    ResponseCommit::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    ResponseCommit::get_data_for_reflect,
                    ResponseCommit::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "log",
                    ResponseCommit::get_log_for_reflect,
                    ResponseCommit::mut_log_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseCommit>(
                    "ResponseCommit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseCommit {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_data();
        self.clear_log();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseCommit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseCommit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseInitChain {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseInitChain {}

impl ResponseInitChain {
    pub fn new() -> ResponseInitChain {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseInitChain {
        static mut instance: ::protobuf::lazy::Lazy<ResponseInitChain> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseInitChain,
        };
        unsafe {
            instance.get(ResponseInitChain::new)
        }
    }
}

impl ::protobuf::Message for ResponseInitChain {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for ResponseInitChain {
    fn new() -> ResponseInitChain {
        ResponseInitChain::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseInitChain>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ResponseInitChain>(
                    "ResponseInitChain",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseInitChain {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseInitChain {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseInitChain {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseBeginBlock {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseBeginBlock {}

impl ResponseBeginBlock {
    pub fn new() -> ResponseBeginBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseBeginBlock {
        static mut instance: ::protobuf::lazy::Lazy<ResponseBeginBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseBeginBlock,
        };
        unsafe {
            instance.get(ResponseBeginBlock::new)
        }
    }
}

impl ::protobuf::Message for ResponseBeginBlock {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for ResponseBeginBlock {
    fn new() -> ResponseBeginBlock {
        ResponseBeginBlock::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseBeginBlock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ResponseBeginBlock>(
                    "ResponseBeginBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseBeginBlock {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseBeginBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseBeginBlock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseEndBlock {
    // message fields
    diffs: ::protobuf::RepeatedField<Validator>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseEndBlock {}

impl ResponseEndBlock {
    pub fn new() -> ResponseEndBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseEndBlock {
        static mut instance: ::protobuf::lazy::Lazy<ResponseEndBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseEndBlock,
        };
        unsafe {
            instance.get(ResponseEndBlock::new)
        }
    }

    // repeated .types.Validator diffs = 1;

    pub fn clear_diffs(&mut self) {
        self.diffs.clear();
    }

    // Param is passed by value, moved
    pub fn set_diffs(&mut self, v: ::protobuf::RepeatedField<Validator>) {
        self.diffs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_diffs(&mut self) -> &mut ::protobuf::RepeatedField<Validator> {
        &mut self.diffs
    }

    // Take field
    pub fn take_diffs(&mut self) -> ::protobuf::RepeatedField<Validator> {
        ::std::mem::replace(&mut self.diffs, ::protobuf::RepeatedField::new())
    }

    pub fn get_diffs(&self) -> &[Validator] {
        &self.diffs
    }

    fn get_diffs_for_reflect(&self) -> &::protobuf::RepeatedField<Validator> {
        &self.diffs
    }

    fn mut_diffs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Validator> {
        &mut self.diffs
    }
}

impl ::protobuf::Message for ResponseEndBlock {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.diffs)?;
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
        for value in &self.diffs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.diffs {
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

impl ::protobuf::MessageStatic for ResponseEndBlock {
    fn new() -> ResponseEndBlock {
        ResponseEndBlock::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseEndBlock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Validator>>(
                    "diffs",
                    ResponseEndBlock::get_diffs_for_reflect,
                    ResponseEndBlock::mut_diffs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseEndBlock>(
                    "ResponseEndBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseEndBlock {
    fn clear(&mut self) {
        self.clear_diffs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseEndBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseEndBlock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Header {
    // message fields
    pub chain_id: ::std::string::String,
    pub height: u64,
    pub time: u64,
    pub num_txs: u64,
    last_block_id: ::protobuf::SingularPtrField<BlockID>,
    pub last_commit_hash: ::std::vec::Vec<u8>,
    pub data_hash: ::std::vec::Vec<u8>,
    pub validators_hash: ::std::vec::Vec<u8>,
    pub app_hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Header {}

impl Header {
    pub fn new() -> Header {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Header {
        static mut instance: ::protobuf::lazy::Lazy<Header> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Header,
        };
        unsafe {
            instance.get(Header::new)
        }
    }

    // string chain_id = 1;

    pub fn clear_chain_id(&mut self) {
        self.chain_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_chain_id(&mut self, v: ::std::string::String) {
        self.chain_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chain_id(&mut self) -> &mut ::std::string::String {
        &mut self.chain_id
    }

    // Take field
    pub fn take_chain_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.chain_id, ::std::string::String::new())
    }

    pub fn get_chain_id(&self) -> &str {
        &self.chain_id
    }

    fn get_chain_id_for_reflect(&self) -> &::std::string::String {
        &self.chain_id
    }

    fn mut_chain_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.chain_id
    }

    // uint64 height = 2;

    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u64) {
        self.height = v;
    }

    pub fn get_height(&self) -> u64 {
        self.height
    }

    fn get_height_for_reflect(&self) -> &u64 {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut u64 {
        &mut self.height
    }

    // uint64 time = 3;

    pub fn clear_time(&mut self) {
        self.time = 0;
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: u64) {
        self.time = v;
    }

    pub fn get_time(&self) -> u64 {
        self.time
    }

    fn get_time_for_reflect(&self) -> &u64 {
        &self.time
    }

    fn mut_time_for_reflect(&mut self) -> &mut u64 {
        &mut self.time
    }

    // uint64 num_txs = 4;

    pub fn clear_num_txs(&mut self) {
        self.num_txs = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_txs(&mut self, v: u64) {
        self.num_txs = v;
    }

    pub fn get_num_txs(&self) -> u64 {
        self.num_txs
    }

    fn get_num_txs_for_reflect(&self) -> &u64 {
        &self.num_txs
    }

    fn mut_num_txs_for_reflect(&mut self) -> &mut u64 {
        &mut self.num_txs
    }

    // .types.BlockID last_block_id = 5;

    pub fn clear_last_block_id(&mut self) {
        self.last_block_id.clear();
    }

    pub fn has_last_block_id(&self) -> bool {
        self.last_block_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_block_id(&mut self, v: BlockID) {
        self.last_block_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_block_id(&mut self) -> &mut BlockID {
        if self.last_block_id.is_none() {
            self.last_block_id.set_default();
        };
        self.last_block_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_last_block_id(&mut self) -> BlockID {
        self.last_block_id.take().unwrap_or_else(|| BlockID::new())
    }

    pub fn get_last_block_id(&self) -> &BlockID {
        self.last_block_id.as_ref().unwrap_or_else(|| BlockID::default_instance())
    }

    fn get_last_block_id_for_reflect(&self) -> &::protobuf::SingularPtrField<BlockID> {
        &self.last_block_id
    }

    fn mut_last_block_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BlockID> {
        &mut self.last_block_id
    }

    // bytes last_commit_hash = 6;

    pub fn clear_last_commit_hash(&mut self) {
        self.last_commit_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_last_commit_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.last_commit_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_commit_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.last_commit_hash
    }

    // Take field
    pub fn take_last_commit_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.last_commit_hash, ::std::vec::Vec::new())
    }

    pub fn get_last_commit_hash(&self) -> &[u8] {
        &self.last_commit_hash
    }

    fn get_last_commit_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.last_commit_hash
    }

    fn mut_last_commit_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.last_commit_hash
    }

    // bytes data_hash = 7;

    pub fn clear_data_hash(&mut self) {
        self.data_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_data_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.data_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data_hash
    }

    // Take field
    pub fn take_data_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data_hash, ::std::vec::Vec::new())
    }

    pub fn get_data_hash(&self) -> &[u8] {
        &self.data_hash
    }

    fn get_data_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data_hash
    }

    fn mut_data_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data_hash
    }

    // bytes validators_hash = 8;

    pub fn clear_validators_hash(&mut self) {
        self.validators_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_validators_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.validators_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_validators_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.validators_hash
    }

    // Take field
    pub fn take_validators_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.validators_hash, ::std::vec::Vec::new())
    }

    pub fn get_validators_hash(&self) -> &[u8] {
        &self.validators_hash
    }

    fn get_validators_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.validators_hash
    }

    fn mut_validators_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.validators_hash
    }

    // bytes app_hash = 9;

    pub fn clear_app_hash(&mut self) {
        self.app_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_app_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.app_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_app_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.app_hash
    }

    // Take field
    pub fn take_app_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.app_hash, ::std::vec::Vec::new())
    }

    pub fn get_app_hash(&self) -> &[u8] {
        &self.app_hash
    }

    fn get_app_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.app_hash
    }

    fn mut_app_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.app_hash
    }
}

impl ::protobuf::Message for Header {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.chain_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.time = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.num_txs = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.last_block_id)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.last_commit_hash)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data_hash)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.validators_hash)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.app_hash)?;
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
        if !self.chain_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.chain_id);
        };
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(2, self.height, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.time != 0 {
            my_size += ::protobuf::rt::value_size(3, self.time, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.num_txs != 0 {
            my_size += ::protobuf::rt::value_size(4, self.num_txs, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.last_block_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.last_commit_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(6, &self.last_commit_hash);
        };
        if !self.data_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(7, &self.data_hash);
        };
        if !self.validators_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(8, &self.validators_hash);
        };
        if !self.app_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(9, &self.app_hash);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.chain_id.is_empty() {
            os.write_string(1, &self.chain_id)?;
        };
        if self.height != 0 {
            os.write_uint64(2, self.height)?;
        };
        if self.time != 0 {
            os.write_uint64(3, self.time)?;
        };
        if self.num_txs != 0 {
            os.write_uint64(4, self.num_txs)?;
        };
        if let Some(v) = self.last_block_id.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.last_commit_hash.is_empty() {
            os.write_bytes(6, &self.last_commit_hash)?;
        };
        if !self.data_hash.is_empty() {
            os.write_bytes(7, &self.data_hash)?;
        };
        if !self.validators_hash.is_empty() {
            os.write_bytes(8, &self.validators_hash)?;
        };
        if !self.app_hash.is_empty() {
            os.write_bytes(9, &self.app_hash)?;
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

impl ::protobuf::MessageStatic for Header {
    fn new() -> Header {
        Header::new()
    }

    fn descriptor_static(_: ::std::option::Option<Header>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "chain_id",
                    Header::get_chain_id_for_reflect,
                    Header::mut_chain_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    Header::get_height_for_reflect,
                    Header::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "time",
                    Header::get_time_for_reflect,
                    Header::mut_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "num_txs",
                    Header::get_num_txs_for_reflect,
                    Header::mut_num_txs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockID>>(
                    "last_block_id",
                    Header::get_last_block_id_for_reflect,
                    Header::mut_last_block_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "last_commit_hash",
                    Header::get_last_commit_hash_for_reflect,
                    Header::mut_last_commit_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data_hash",
                    Header::get_data_hash_for_reflect,
                    Header::mut_data_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "validators_hash",
                    Header::get_validators_hash_for_reflect,
                    Header::mut_validators_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "app_hash",
                    Header::get_app_hash_for_reflect,
                    Header::mut_app_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Header>(
                    "Header",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Header {
    fn clear(&mut self) {
        self.clear_chain_id();
        self.clear_height();
        self.clear_time();
        self.clear_num_txs();
        self.clear_last_block_id();
        self.clear_last_commit_hash();
        self.clear_data_hash();
        self.clear_validators_hash();
        self.clear_app_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Header {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Header {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockID {
    // message fields
    pub hash: ::std::vec::Vec<u8>,
    parts: ::protobuf::SingularPtrField<PartSetHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockID {}

impl BlockID {
    pub fn new() -> BlockID {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockID {
        static mut instance: ::protobuf::lazy::Lazy<BlockID> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockID,
        };
        unsafe {
            instance.get(BlockID::new)
        }
    }

    // bytes hash = 1;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    fn get_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // .types.PartSetHeader parts = 2;

    pub fn clear_parts(&mut self) {
        self.parts.clear();
    }

    pub fn has_parts(&self) -> bool {
        self.parts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parts(&mut self, v: PartSetHeader) {
        self.parts = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parts(&mut self) -> &mut PartSetHeader {
        if self.parts.is_none() {
            self.parts.set_default();
        };
        self.parts.as_mut().unwrap()
    }

    // Take field
    pub fn take_parts(&mut self) -> PartSetHeader {
        self.parts.take().unwrap_or_else(|| PartSetHeader::new())
    }

    pub fn get_parts(&self) -> &PartSetHeader {
        self.parts.as_ref().unwrap_or_else(|| PartSetHeader::default_instance())
    }

    fn get_parts_for_reflect(&self) -> &::protobuf::SingularPtrField<PartSetHeader> {
        &self.parts
    }

    fn mut_parts_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PartSetHeader> {
        &mut self.parts
    }
}

impl ::protobuf::Message for BlockID {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.parts)?;
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
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.hash);
        };
        if let Some(v) = self.parts.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.hash.is_empty() {
            os.write_bytes(1, &self.hash)?;
        };
        if let Some(v) = self.parts.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for BlockID {
    fn new() -> BlockID {
        BlockID::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockID>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    BlockID::get_hash_for_reflect,
                    BlockID::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PartSetHeader>>(
                    "parts",
                    BlockID::get_parts_for_reflect,
                    BlockID::mut_parts_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockID>(
                    "BlockID",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockID {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_parts();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockID {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PartSetHeader {
    // message fields
    pub total: u64,
    pub hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PartSetHeader {}

impl PartSetHeader {
    pub fn new() -> PartSetHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PartSetHeader {
        static mut instance: ::protobuf::lazy::Lazy<PartSetHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PartSetHeader,
        };
        unsafe {
            instance.get(PartSetHeader::new)
        }
    }

    // uint64 total = 1;

    pub fn clear_total(&mut self) {
        self.total = 0;
    }

    // Param is passed by value, moved
    pub fn set_total(&mut self, v: u64) {
        self.total = v;
    }

    pub fn get_total(&self) -> u64 {
        self.total
    }

    fn get_total_for_reflect(&self) -> &u64 {
        &self.total
    }

    fn mut_total_for_reflect(&mut self) -> &mut u64 {
        &mut self.total
    }

    // bytes hash = 2;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    fn get_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }
}

impl ::protobuf::Message for PartSetHeader {
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
                    };
                    let tmp = is.read_uint64()?;
                    self.total = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
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
        if self.total != 0 {
            my_size += ::protobuf::rt::value_size(1, self.total, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.hash);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.total != 0 {
            os.write_uint64(1, self.total)?;
        };
        if !self.hash.is_empty() {
            os.write_bytes(2, &self.hash)?;
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

impl ::protobuf::MessageStatic for PartSetHeader {
    fn new() -> PartSetHeader {
        PartSetHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<PartSetHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "total",
                    PartSetHeader::get_total_for_reflect,
                    PartSetHeader::mut_total_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    PartSetHeader::get_hash_for_reflect,
                    PartSetHeader::mut_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PartSetHeader>(
                    "PartSetHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PartSetHeader {
    fn clear(&mut self) {
        self.clear_total();
        self.clear_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PartSetHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PartSetHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Validator {
    // message fields
    pub pubKey: ::std::vec::Vec<u8>,
    pub power: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Validator {}

impl Validator {
    pub fn new() -> Validator {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Validator {
        static mut instance: ::protobuf::lazy::Lazy<Validator> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Validator,
        };
        unsafe {
            instance.get(Validator::new)
        }
    }

    // bytes pubKey = 1;

    pub fn clear_pubKey(&mut self) {
        self.pubKey.clear();
    }

    // Param is passed by value, moved
    pub fn set_pubKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.pubKey = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pubKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.pubKey
    }

    // Take field
    pub fn take_pubKey(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.pubKey, ::std::vec::Vec::new())
    }

    pub fn get_pubKey(&self) -> &[u8] {
        &self.pubKey
    }

    fn get_pubKey_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.pubKey
    }

    fn mut_pubKey_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.pubKey
    }

    // uint64 power = 2;

    pub fn clear_power(&mut self) {
        self.power = 0;
    }

    // Param is passed by value, moved
    pub fn set_power(&mut self, v: u64) {
        self.power = v;
    }

    pub fn get_power(&self) -> u64 {
        self.power
    }

    fn get_power_for_reflect(&self) -> &u64 {
        &self.power
    }

    fn mut_power_for_reflect(&mut self) -> &mut u64 {
        &mut self.power
    }
}

impl ::protobuf::Message for Validator {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.pubKey)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.power = tmp;
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
        if !self.pubKey.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.pubKey);
        };
        if self.power != 0 {
            my_size += ::protobuf::rt::value_size(2, self.power, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.pubKey.is_empty() {
            os.write_bytes(1, &self.pubKey)?;
        };
        if self.power != 0 {
            os.write_uint64(2, self.power)?;
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

impl ::protobuf::MessageStatic for Validator {
    fn new() -> Validator {
        Validator::new()
    }

    fn descriptor_static(_: ::std::option::Option<Validator>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "pubKey",
                    Validator::get_pubKey_for_reflect,
                    Validator::mut_pubKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "power",
                    Validator::get_power_for_reflect,
                    Validator::mut_power_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Validator>(
                    "Validator",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Validator {
    fn clear(&mut self) {
        self.clear_pubKey();
        self.clear_power();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Validator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Validator {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MessageType {
    NullMessage = 0,
    Echo = 1,
    Flush = 2,
    Info = 3,
    SetOption = 4,
    Exception = 5,
    DeliverTx = 17,
    CheckTx = 18,
    Commit = 19,
    Query = 20,
    InitChain = 21,
    BeginBlock = 22,
    EndBlock = 23,
}

impl ::protobuf::ProtobufEnum for MessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MessageType> {
        match value {
            0 => ::std::option::Option::Some(MessageType::NullMessage),
            1 => ::std::option::Option::Some(MessageType::Echo),
            2 => ::std::option::Option::Some(MessageType::Flush),
            3 => ::std::option::Option::Some(MessageType::Info),
            4 => ::std::option::Option::Some(MessageType::SetOption),
            5 => ::std::option::Option::Some(MessageType::Exception),
            17 => ::std::option::Option::Some(MessageType::DeliverTx),
            18 => ::std::option::Option::Some(MessageType::CheckTx),
            19 => ::std::option::Option::Some(MessageType::Commit),
            20 => ::std::option::Option::Some(MessageType::Query),
            21 => ::std::option::Option::Some(MessageType::InitChain),
            22 => ::std::option::Option::Some(MessageType::BeginBlock),
            23 => ::std::option::Option::Some(MessageType::EndBlock),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MessageType] = &[
            MessageType::NullMessage,
            MessageType::Echo,
            MessageType::Flush,
            MessageType::Info,
            MessageType::SetOption,
            MessageType::Exception,
            MessageType::DeliverTx,
            MessageType::CheckTx,
            MessageType::Commit,
            MessageType::Query,
            MessageType::InitChain,
            MessageType::BeginBlock,
            MessageType::EndBlock,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<MessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MessageType {
}

impl ::std::default::Default for MessageType {
    fn default() -> Self {
        MessageType::NullMessage
    }
}

impl ::protobuf::reflect::ProtobufValue for MessageType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CodeType {
    OK = 0,
    InternalError = 1,
    EncodingError = 2,
    BadNonce = 3,
    Unauthorized = 4,
    InsufficientFunds = 5,
    UnknownRequest = 6,
    BaseDuplicateAddress = 101,
    BaseEncodingError = 102,
    BaseInsufficientFees = 103,
    BaseInsufficientFunds = 104,
    BaseInsufficientGasPrice = 105,
    BaseInvalidInput = 106,
    BaseInvalidOutput = 107,
    BaseInvalidPubKey = 108,
    BaseInvalidSequence = 109,
    BaseInvalidSignature = 110,
    BaseUnknownAddress = 111,
    BaseUnknownPubKey = 112,
    BaseUnknownPlugin = 113,
    GovUnknownEntity = 201,
    GovUnknownGroup = 202,
    GovUnknownProposal = 203,
    GovDuplicateGroup = 204,
    GovDuplicateMember = 205,
    GovDuplicateProposal = 206,
    GovDuplicateVote = 207,
    GovInvalidMember = 208,
    GovInvalidVote = 209,
    GovInvalidVotingPower = 210,
}

impl ::protobuf::ProtobufEnum for CodeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CodeType> {
        match value {
            0 => ::std::option::Option::Some(CodeType::OK),
            1 => ::std::option::Option::Some(CodeType::InternalError),
            2 => ::std::option::Option::Some(CodeType::EncodingError),
            3 => ::std::option::Option::Some(CodeType::BadNonce),
            4 => ::std::option::Option::Some(CodeType::Unauthorized),
            5 => ::std::option::Option::Some(CodeType::InsufficientFunds),
            6 => ::std::option::Option::Some(CodeType::UnknownRequest),
            101 => ::std::option::Option::Some(CodeType::BaseDuplicateAddress),
            102 => ::std::option::Option::Some(CodeType::BaseEncodingError),
            103 => ::std::option::Option::Some(CodeType::BaseInsufficientFees),
            104 => ::std::option::Option::Some(CodeType::BaseInsufficientFunds),
            105 => ::std::option::Option::Some(CodeType::BaseInsufficientGasPrice),
            106 => ::std::option::Option::Some(CodeType::BaseInvalidInput),
            107 => ::std::option::Option::Some(CodeType::BaseInvalidOutput),
            108 => ::std::option::Option::Some(CodeType::BaseInvalidPubKey),
            109 => ::std::option::Option::Some(CodeType::BaseInvalidSequence),
            110 => ::std::option::Option::Some(CodeType::BaseInvalidSignature),
            111 => ::std::option::Option::Some(CodeType::BaseUnknownAddress),
            112 => ::std::option::Option::Some(CodeType::BaseUnknownPubKey),
            113 => ::std::option::Option::Some(CodeType::BaseUnknownPlugin),
            201 => ::std::option::Option::Some(CodeType::GovUnknownEntity),
            202 => ::std::option::Option::Some(CodeType::GovUnknownGroup),
            203 => ::std::option::Option::Some(CodeType::GovUnknownProposal),
            204 => ::std::option::Option::Some(CodeType::GovDuplicateGroup),
            205 => ::std::option::Option::Some(CodeType::GovDuplicateMember),
            206 => ::std::option::Option::Some(CodeType::GovDuplicateProposal),
            207 => ::std::option::Option::Some(CodeType::GovDuplicateVote),
            208 => ::std::option::Option::Some(CodeType::GovInvalidMember),
            209 => ::std::option::Option::Some(CodeType::GovInvalidVote),
            210 => ::std::option::Option::Some(CodeType::GovInvalidVotingPower),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CodeType] = &[
            CodeType::OK,
            CodeType::InternalError,
            CodeType::EncodingError,
            CodeType::BadNonce,
            CodeType::Unauthorized,
            CodeType::InsufficientFunds,
            CodeType::UnknownRequest,
            CodeType::BaseDuplicateAddress,
            CodeType::BaseEncodingError,
            CodeType::BaseInsufficientFees,
            CodeType::BaseInsufficientFunds,
            CodeType::BaseInsufficientGasPrice,
            CodeType::BaseInvalidInput,
            CodeType::BaseInvalidOutput,
            CodeType::BaseInvalidPubKey,
            CodeType::BaseInvalidSequence,
            CodeType::BaseInvalidSignature,
            CodeType::BaseUnknownAddress,
            CodeType::BaseUnknownPubKey,
            CodeType::BaseUnknownPlugin,
            CodeType::GovUnknownEntity,
            CodeType::GovUnknownGroup,
            CodeType::GovUnknownProposal,
            CodeType::GovDuplicateGroup,
            CodeType::GovDuplicateMember,
            CodeType::GovDuplicateProposal,
            CodeType::GovDuplicateVote,
            CodeType::GovInvalidMember,
            CodeType::GovInvalidVote,
            CodeType::GovInvalidVotingPower,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CodeType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CodeType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CodeType {
}

impl ::std::default::Default for CodeType {
    fn default() -> Self {
        CodeType::OK
    }
}

impl ::protobuf::reflect::ProtobufValue for CodeType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0b, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x74,
    0x79, 0x70, 0x65, 0x73, 0x22, 0xc6, 0x04, 0x0a, 0x07, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x28, 0x0a, 0x04, 0x65, 0x63, 0x68, 0x6f, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12,
    0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x45, 0x63,
    0x68, 0x6f, 0x48, 0x00, 0x52, 0x04, 0x65, 0x63, 0x68, 0x6f, 0x12, 0x2b, 0x0a, 0x05, 0x66, 0x6c,
    0x75, 0x73, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x74, 0x79, 0x70, 0x65,
    0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x46, 0x6c, 0x75, 0x73, 0x68, 0x48, 0x00,
    0x52, 0x05, 0x66, 0x6c, 0x75, 0x73, 0x68, 0x12, 0x28, 0x0a, 0x04, 0x69, 0x6e, 0x66, 0x6f, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x48, 0x00, 0x52, 0x04, 0x69, 0x6e, 0x66,
    0x6f, 0x12, 0x38, 0x0a, 0x0a, 0x73, 0x65, 0x74, 0x5f, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x53, 0x65, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00,
    0x52, 0x09, 0x73, 0x65, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x38, 0x0a, 0x0a, 0x64,
    0x65, 0x6c, 0x69, 0x76, 0x65, 0x72, 0x5f, 0x74, 0x78, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x17, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x44,
    0x65, 0x6c, 0x69, 0x76, 0x65, 0x72, 0x54, 0x78, 0x48, 0x00, 0x52, 0x09, 0x64, 0x65, 0x6c, 0x69,
    0x76, 0x65, 0x72, 0x54, 0x78, 0x12, 0x32, 0x0a, 0x08, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x5f, 0x74,
    0x78, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x54, 0x78, 0x48, 0x00,
    0x52, 0x07, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x54, 0x78, 0x12, 0x2e, 0x0a, 0x06, 0x63, 0x6f, 0x6d,
    0x6d, 0x69, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x74, 0x79, 0x70, 0x65,
    0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x48,
    0x00, 0x52, 0x06, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x12, 0x2b, 0x0a, 0x05, 0x71, 0x75, 0x65,
    0x72, 0x79, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73,
    0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x51, 0x75, 0x65, 0x72, 0x79, 0x48, 0x00, 0x52,
    0x05, 0x71, 0x75, 0x65, 0x72, 0x79, 0x12, 0x38, 0x0a, 0x0a, 0x69, 0x6e, 0x69, 0x74, 0x5f, 0x63,
    0x68, 0x61, 0x69, 0x6e, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x74, 0x79, 0x70,
    0x65, 0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x49, 0x6e, 0x69, 0x74, 0x43, 0x68,
    0x61, 0x69, 0x6e, 0x48, 0x00, 0x52, 0x09, 0x69, 0x6e, 0x69, 0x74, 0x43, 0x68, 0x61, 0x69, 0x6e,
    0x12, 0x3b, 0x0a, 0x0b, 0x62, 0x65, 0x67, 0x69, 0x6e, 0x5f, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x42, 0x65, 0x67, 0x69, 0x6e, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x48,
    0x00, 0x52, 0x0a, 0x62, 0x65, 0x67, 0x69, 0x6e, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x35, 0x0a,
    0x09, 0x65, 0x6e, 0x64, 0x5f, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x16, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x45, 0x6e, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x48, 0x00, 0x52, 0x08, 0x65, 0x6e, 0x64, 0x42,
    0x6c, 0x6f, 0x63, 0x6b, 0x42, 0x07, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x27, 0x0a,
    0x0b, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x45, 0x63, 0x68, 0x6f, 0x12, 0x18, 0x0a, 0x07,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x0e, 0x0a, 0x0c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x46, 0x6c, 0x75, 0x73, 0x68, 0x22, 0x0d, 0x0a, 0x0b, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x3a, 0x0a, 0x10, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x53, 0x65, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x22, 0x22, 0x0a, 0x10, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x44, 0x65, 0x6c, 0x69,
    0x76, 0x65, 0x72, 0x54, 0x78, 0x12, 0x0e, 0x0a, 0x02, 0x74, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x02, 0x74, 0x78, 0x22, 0x20, 0x0a, 0x0e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x43, 0x68, 0x65, 0x63, 0x6b, 0x54, 0x78, 0x12, 0x0e, 0x0a, 0x02, 0x74, 0x78, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x02, 0x74, 0x78, 0x22, 0x64, 0x0a, 0x0c, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x51, 0x75, 0x65, 0x72, 0x79, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x12, 0x12, 0x0a, 0x04, 0x70,
    0x61, 0x74, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12,
    0x16, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x72, 0x6f, 0x76, 0x65,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x05, 0x70, 0x72, 0x6f, 0x76, 0x65, 0x22, 0x0f, 0x0a,
    0x0d, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x22, 0x44,
    0x0a, 0x10, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x49, 0x6e, 0x69, 0x74, 0x43, 0x68, 0x61,
    0x69, 0x6e, 0x12, 0x30, 0x0a, 0x0a, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x73,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x56,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x52, 0x0a, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61,
    0x74, 0x6f, 0x72, 0x73, 0x22, 0x4e, 0x0a, 0x11, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x42,
    0x65, 0x67, 0x69, 0x6e, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x12, 0x0a, 0x04, 0x68, 0x61, 0x73,
    0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x68, 0x61, 0x73, 0x68, 0x12, 0x25, 0x0a,
    0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e,
    0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x22, 0x29, 0x0a, 0x0f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x45,
    0x6e, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x16, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68,
    0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x22,
    0x8c, 0x05, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x38, 0x0a, 0x09,
    0x65, 0x78, 0x63, 0x65, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x18, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x45, 0x78, 0x63, 0x65, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x09, 0x65, 0x78, 0x63,
    0x65, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x29, 0x0a, 0x04, 0x65, 0x63, 0x68, 0x6f, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x45, 0x63, 0x68, 0x6f, 0x48, 0x00, 0x52, 0x04, 0x65, 0x63, 0x68,
    0x6f, 0x12, 0x2c, 0x0a, 0x05, 0x66, 0x6c, 0x75, 0x73, 0x68, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x14, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x46, 0x6c, 0x75, 0x73, 0x68, 0x48, 0x00, 0x52, 0x05, 0x66, 0x6c, 0x75, 0x73, 0x68, 0x12,
    0x29, 0x0a, 0x04, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e,
    0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x49, 0x6e,
    0x66, 0x6f, 0x48, 0x00, 0x52, 0x04, 0x69, 0x6e, 0x66, 0x6f, 0x12, 0x39, 0x0a, 0x0a, 0x73, 0x65,
    0x74, 0x5f, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18,
    0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x53,
    0x65, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x09, 0x73, 0x65, 0x74, 0x4f,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x39, 0x0a, 0x0a, 0x64, 0x65, 0x6c, 0x69, 0x76, 0x65, 0x72,
    0x5f, 0x74, 0x78, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x74, 0x79, 0x70, 0x65,
    0x73, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x44, 0x65, 0x6c, 0x69, 0x76, 0x65,
    0x72, 0x54, 0x78, 0x48, 0x00, 0x52, 0x09, 0x64, 0x65, 0x6c, 0x69, 0x76, 0x65, 0x72, 0x54, 0x78,
    0x12, 0x33, 0x0a, 0x08, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x5f, 0x74, 0x78, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x16, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x54, 0x78, 0x48, 0x00, 0x52, 0x07, 0x63, 0x68,
    0x65, 0x63, 0x6b, 0x54, 0x78, 0x12, 0x2f, 0x0a, 0x06, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x18,
    0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x48, 0x00, 0x52, 0x06,
    0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x12, 0x2c, 0x0a, 0x05, 0x71, 0x75, 0x65, 0x72, 0x79, 0x18,
    0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x51, 0x75, 0x65, 0x72, 0x79, 0x48, 0x00, 0x52, 0x05, 0x71,
    0x75, 0x65, 0x72, 0x79, 0x12, 0x39, 0x0a, 0x0a, 0x69, 0x6e, 0x69, 0x74, 0x5f, 0x63, 0x68, 0x61,
    0x69, 0x6e, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73,
    0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x49, 0x6e, 0x69, 0x74, 0x43, 0x68, 0x61,
    0x69, 0x6e, 0x48, 0x00, 0x52, 0x09, 0x69, 0x6e, 0x69, 0x74, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x12,
    0x3c, 0x0a, 0x0b, 0x62, 0x65, 0x67, 0x69, 0x6e, 0x5f, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x0b,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x65, 0x67, 0x69, 0x6e, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x48,
    0x00, 0x52, 0x0a, 0x62, 0x65, 0x67, 0x69, 0x6e, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x36, 0x0a,
    0x09, 0x65, 0x6e, 0x64, 0x5f, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x17, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x45, 0x6e, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x48, 0x00, 0x52, 0x08, 0x65, 0x6e, 0x64,
    0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x42, 0x07, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x29,
    0x0a, 0x11, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x45, 0x78, 0x63, 0x65, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x22, 0x28, 0x0a, 0x0c, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x45, 0x63, 0x68, 0x6f, 0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x22, 0x0f, 0x0a, 0x0d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x46,
    0x6c, 0x75, 0x73, 0x68, 0x22, 0x97, 0x01, 0x0a, 0x0c, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x12, 0x2a, 0x0a, 0x11, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x62, 0x6c, 0x6f, 0x63,
    0x6b, 0x5f, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0f,
    0x6c, 0x61, 0x73, 0x74, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x48, 0x65, 0x69, 0x67, 0x68, 0x74, 0x12,
    0x2d, 0x0a, 0x13, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x61, 0x70,
    0x70, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x10, 0x6c, 0x61,
    0x73, 0x74, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x41, 0x70, 0x70, 0x48, 0x61, 0x73, 0x68, 0x22, 0x25,
    0x0a, 0x11, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x53, 0x65, 0x74, 0x4f, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x12, 0x10, 0x0a, 0x03, 0x6c, 0x6f, 0x67, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x03, 0x6c, 0x6f, 0x67, 0x22, 0x5e, 0x0a, 0x11, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x44, 0x65, 0x6c, 0x69, 0x76, 0x65, 0x72, 0x54, 0x78, 0x12, 0x23, 0x0a, 0x04, 0x63, 0x6f,
    0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0f, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73,
    0x2e, 0x43, 0x6f, 0x64, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x12,
    0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64,
    0x61, 0x74, 0x61, 0x12, 0x10, 0x0a, 0x03, 0x6c, 0x6f, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x03, 0x6c, 0x6f, 0x67, 0x22, 0x5c, 0x0a, 0x0f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x54, 0x78, 0x12, 0x23, 0x0a, 0x04, 0x63, 0x6f, 0x64, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0f, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x43,
    0x6f, 0x64, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x12, 0x12, 0x0a,
    0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74,
    0x61, 0x12, 0x10, 0x0a, 0x03, 0x6c, 0x6f, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03,
    0x6c, 0x6f, 0x67, 0x22, 0xb2, 0x01, 0x0a, 0x0d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x51, 0x75, 0x65, 0x72, 0x79, 0x12, 0x23, 0x0a, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x0f, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x43, 0x6f, 0x64, 0x65,
    0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78,
    0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b,
    0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x72, 0x6f, 0x6f,
    0x66, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x70, 0x72, 0x6f, 0x6f, 0x66, 0x12, 0x16,
    0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x52, 0x06,
    0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6c, 0x6f, 0x67, 0x18, 0x07, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x03, 0x6c, 0x6f, 0x67, 0x22, 0x5b, 0x0a, 0x0e, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x12, 0x23, 0x0a, 0x04, 0x63, 0x6f,
    0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0f, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73,
    0x2e, 0x43, 0x6f, 0x64, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x12,
    0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64,
    0x61, 0x74, 0x61, 0x12, 0x10, 0x0a, 0x03, 0x6c, 0x6f, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x03, 0x6c, 0x6f, 0x67, 0x22, 0x13, 0x0a, 0x11, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x49, 0x6e, 0x69, 0x74, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x22, 0x14, 0x0a, 0x12, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x65, 0x67, 0x69, 0x6e, 0x42, 0x6c, 0x6f, 0x63, 0x6b,
    0x22, 0x3a, 0x0a, 0x10, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x45, 0x6e, 0x64, 0x42,
    0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x26, 0x0a, 0x05, 0x64, 0x69, 0x66, 0x66, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x56, 0x61, 0x6c, 0x69,
    0x64, 0x61, 0x74, 0x6f, 0x72, 0x52, 0x05, 0x64, 0x69, 0x66, 0x66, 0x73, 0x22, 0xa7, 0x02, 0x0a,
    0x06, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x19, 0x0a, 0x08, 0x63, 0x68, 0x61, 0x69, 0x6e,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x63, 0x68, 0x61, 0x69, 0x6e,
    0x49, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x69,
    0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x12, 0x17,
    0x0a, 0x07, 0x6e, 0x75, 0x6d, 0x5f, 0x74, 0x78, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x06, 0x6e, 0x75, 0x6d, 0x54, 0x78, 0x73, 0x12, 0x32, 0x0a, 0x0d, 0x6c, 0x61, 0x73, 0x74, 0x5f,
    0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e,
    0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x49, 0x44, 0x52, 0x0b,
    0x6c, 0x61, 0x73, 0x74, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x49, 0x64, 0x12, 0x28, 0x0a, 0x10, 0x6c,
    0x61, 0x73, 0x74, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0e, 0x6c, 0x61, 0x73, 0x74, 0x43, 0x6f, 0x6d, 0x6d, 0x69,
    0x74, 0x48, 0x61, 0x73, 0x68, 0x12, 0x1b, 0x0a, 0x09, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x68, 0x61,
    0x73, 0x68, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x08, 0x64, 0x61, 0x74, 0x61, 0x48, 0x61,
    0x73, 0x68, 0x12, 0x27, 0x0a, 0x0f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x73,
    0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0e, 0x76, 0x61, 0x6c,
    0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x48, 0x61, 0x73, 0x68, 0x12, 0x19, 0x0a, 0x08, 0x61,
    0x70, 0x70, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x61,
    0x70, 0x70, 0x48, 0x61, 0x73, 0x68, 0x22, 0x49, 0x0a, 0x07, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x49,
    0x44, 0x12, 0x12, 0x0a, 0x04, 0x68, 0x61, 0x73, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52,
    0x04, 0x68, 0x61, 0x73, 0x68, 0x12, 0x2a, 0x0a, 0x05, 0x70, 0x61, 0x72, 0x74, 0x73, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x50, 0x61, 0x72,
    0x74, 0x53, 0x65, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x05, 0x70, 0x61, 0x72, 0x74,
    0x73, 0x22, 0x39, 0x0a, 0x0d, 0x50, 0x61, 0x72, 0x74, 0x53, 0x65, 0x74, 0x48, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x05, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x12, 0x12, 0x0a, 0x04, 0x68, 0x61, 0x73, 0x68,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x68, 0x61, 0x73, 0x68, 0x22, 0x39, 0x0a, 0x09,
    0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x12, 0x16, 0x0a, 0x06, 0x70, 0x75, 0x62,
    0x4b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x70, 0x75, 0x62, 0x4b, 0x65,
    0x79, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x6f, 0x77, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x05, 0x70, 0x6f, 0x77, 0x65, 0x72, 0x2a, 0xbb, 0x01, 0x0a, 0x0b, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0f, 0x0a, 0x0b, 0x4e, 0x75, 0x6c, 0x6c, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x45, 0x63, 0x68, 0x6f,
    0x10, 0x01, 0x12, 0x09, 0x0a, 0x05, 0x46, 0x6c, 0x75, 0x73, 0x68, 0x10, 0x02, 0x12, 0x08, 0x0a,
    0x04, 0x49, 0x6e, 0x66, 0x6f, 0x10, 0x03, 0x12, 0x0d, 0x0a, 0x09, 0x53, 0x65, 0x74, 0x4f, 0x70,
    0x74, 0x69, 0x6f, 0x6e, 0x10, 0x04, 0x12, 0x0d, 0x0a, 0x09, 0x45, 0x78, 0x63, 0x65, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x10, 0x05, 0x12, 0x0d, 0x0a, 0x09, 0x44, 0x65, 0x6c, 0x69, 0x76, 0x65, 0x72,
    0x54, 0x78, 0x10, 0x11, 0x12, 0x0b, 0x0a, 0x07, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x54, 0x78, 0x10,
    0x12, 0x12, 0x0a, 0x0a, 0x06, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x10, 0x13, 0x12, 0x09, 0x0a,
    0x05, 0x51, 0x75, 0x65, 0x72, 0x79, 0x10, 0x14, 0x12, 0x0d, 0x0a, 0x09, 0x49, 0x6e, 0x69, 0x74,
    0x43, 0x68, 0x61, 0x69, 0x6e, 0x10, 0x15, 0x12, 0x0e, 0x0a, 0x0a, 0x42, 0x65, 0x67, 0x69, 0x6e,
    0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x10, 0x16, 0x12, 0x0c, 0x0a, 0x08, 0x45, 0x6e, 0x64, 0x42, 0x6c,
    0x6f, 0x63, 0x6b, 0x10, 0x17, 0x2a, 0xb5, 0x05, 0x0a, 0x08, 0x43, 0x6f, 0x64, 0x65, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x06, 0x0a, 0x02, 0x4f, 0x4b, 0x10, 0x00, 0x12, 0x11, 0x0a, 0x0d, 0x49, 0x6e,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x10, 0x01, 0x12, 0x11, 0x0a,
    0x0d, 0x45, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x10, 0x02,
    0x12, 0x0c, 0x0a, 0x08, 0x42, 0x61, 0x64, 0x4e, 0x6f, 0x6e, 0x63, 0x65, 0x10, 0x03, 0x12, 0x10,
    0x0a, 0x0c, 0x55, 0x6e, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x65, 0x64, 0x10, 0x04,
    0x12, 0x15, 0x0a, 0x11, 0x49, 0x6e, 0x73, 0x75, 0x66, 0x66, 0x69, 0x63, 0x69, 0x65, 0x6e, 0x74,
    0x46, 0x75, 0x6e, 0x64, 0x73, 0x10, 0x05, 0x12, 0x12, 0x0a, 0x0e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f,
    0x77, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x06, 0x12, 0x18, 0x0a, 0x14, 0x42,
    0x61, 0x73, 0x65, 0x44, 0x75, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x65, 0x41, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x10, 0x65, 0x12, 0x15, 0x0a, 0x11, 0x42, 0x61, 0x73, 0x65, 0x45, 0x6e, 0x63,
    0x6f, 0x64, 0x69, 0x6e, 0x67, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x10, 0x66, 0x12, 0x18, 0x0a, 0x14,
    0x42, 0x61, 0x73, 0x65, 0x49, 0x6e, 0x73, 0x75, 0x66, 0x66, 0x69, 0x63, 0x69, 0x65, 0x6e, 0x74,
    0x46, 0x65, 0x65, 0x73, 0x10, 0x67, 0x12, 0x19, 0x0a, 0x15, 0x42, 0x61, 0x73, 0x65, 0x49, 0x6e,
    0x73, 0x75, 0x66, 0x66, 0x69, 0x63, 0x69, 0x65, 0x6e, 0x74, 0x46, 0x75, 0x6e, 0x64, 0x73, 0x10,
    0x68, 0x12, 0x1c, 0x0a, 0x18, 0x42, 0x61, 0x73, 0x65, 0x49, 0x6e, 0x73, 0x75, 0x66, 0x66, 0x69,
    0x63, 0x69, 0x65, 0x6e, 0x74, 0x47, 0x61, 0x73, 0x50, 0x72, 0x69, 0x63, 0x65, 0x10, 0x69, 0x12,
    0x14, 0x0a, 0x10, 0x42, 0x61, 0x73, 0x65, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x49, 0x6e,
    0x70, 0x75, 0x74, 0x10, 0x6a, 0x12, 0x15, 0x0a, 0x11, 0x42, 0x61, 0x73, 0x65, 0x49, 0x6e, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x10, 0x6b, 0x12, 0x15, 0x0a, 0x11,
    0x42, 0x61, 0x73, 0x65, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x50, 0x75, 0x62, 0x4b, 0x65,
    0x79, 0x10, 0x6c, 0x12, 0x17, 0x0a, 0x13, 0x42, 0x61, 0x73, 0x65, 0x49, 0x6e, 0x76, 0x61, 0x6c,
    0x69, 0x64, 0x53, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x10, 0x6d, 0x12, 0x18, 0x0a, 0x14,
    0x42, 0x61, 0x73, 0x65, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x53, 0x69, 0x67, 0x6e, 0x61,
    0x74, 0x75, 0x72, 0x65, 0x10, 0x6e, 0x12, 0x16, 0x0a, 0x12, 0x42, 0x61, 0x73, 0x65, 0x55, 0x6e,
    0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x10, 0x6f, 0x12, 0x15,
    0x0a, 0x11, 0x42, 0x61, 0x73, 0x65, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x50, 0x75, 0x62,
    0x4b, 0x65, 0x79, 0x10, 0x70, 0x12, 0x15, 0x0a, 0x11, 0x42, 0x61, 0x73, 0x65, 0x55, 0x6e, 0x6b,
    0x6e, 0x6f, 0x77, 0x6e, 0x50, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x10, 0x71, 0x12, 0x15, 0x0a, 0x10,
    0x47, 0x6f, 0x76, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x10, 0xc9, 0x01, 0x12, 0x14, 0x0a, 0x0f, 0x47, 0x6f, 0x76, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77,
    0x6e, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x10, 0xca, 0x01, 0x12, 0x17, 0x0a, 0x12, 0x47, 0x6f, 0x76,
    0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x50, 0x72, 0x6f, 0x70, 0x6f, 0x73, 0x61, 0x6c, 0x10,
    0xcb, 0x01, 0x12, 0x16, 0x0a, 0x11, 0x47, 0x6f, 0x76, 0x44, 0x75, 0x70, 0x6c, 0x69, 0x63, 0x61,
    0x74, 0x65, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x10, 0xcc, 0x01, 0x12, 0x17, 0x0a, 0x12, 0x47, 0x6f,
    0x76, 0x44, 0x75, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x65, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x10, 0xcd, 0x01, 0x12, 0x19, 0x0a, 0x14, 0x47, 0x6f, 0x76, 0x44, 0x75, 0x70, 0x6c, 0x69, 0x63,
    0x61, 0x74, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x6f, 0x73, 0x61, 0x6c, 0x10, 0xce, 0x01, 0x12, 0x15,
    0x0a, 0x10, 0x47, 0x6f, 0x76, 0x44, 0x75, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x65, 0x56, 0x6f,
    0x74, 0x65, 0x10, 0xcf, 0x01, 0x12, 0x15, 0x0a, 0x10, 0x47, 0x6f, 0x76, 0x49, 0x6e, 0x76, 0x61,
    0x6c, 0x69, 0x64, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x10, 0xd0, 0x01, 0x12, 0x13, 0x0a, 0x0e,
    0x47, 0x6f, 0x76, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x56, 0x6f, 0x74, 0x65, 0x10, 0xd1,
    0x01, 0x12, 0x1a, 0x0a, 0x15, 0x47, 0x6f, 0x76, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x56,
    0x6f, 0x74, 0x69, 0x6e, 0x67, 0x50, 0x6f, 0x77, 0x65, 0x72, 0x10, 0xd2, 0x01, 0x32, 0x8c, 0x05,
    0x0a, 0x0f, 0x41, 0x42, 0x43, 0x49, 0x41, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x2f, 0x0a, 0x04, 0x45, 0x63, 0x68, 0x6f, 0x12, 0x12, 0x2e, 0x74, 0x79, 0x70, 0x65,
    0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x45, 0x63, 0x68, 0x6f, 0x1a, 0x13, 0x2e,
    0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x45, 0x63,
    0x68, 0x6f, 0x12, 0x32, 0x0a, 0x05, 0x46, 0x6c, 0x75, 0x73, 0x68, 0x12, 0x13, 0x2e, 0x74, 0x79,
    0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x46, 0x6c, 0x75, 0x73, 0x68,
    0x1a, 0x14, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x46, 0x6c, 0x75, 0x73, 0x68, 0x12, 0x2f, 0x0a, 0x04, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x12,
    0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x49, 0x6e,
    0x66, 0x6f, 0x1a, 0x13, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x3e, 0x0a, 0x09, 0x53, 0x65, 0x74, 0x4f, 0x70,
    0x74, 0x69, 0x6f, 0x6e, 0x12, 0x17, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x53, 0x65, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x1a, 0x18, 0x2e,
    0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x53, 0x65,
    0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x3e, 0x0a, 0x09, 0x44, 0x65, 0x6c, 0x69, 0x76,
    0x65, 0x72, 0x54, 0x78, 0x12, 0x17, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x44, 0x65, 0x6c, 0x69, 0x76, 0x65, 0x72, 0x54, 0x78, 0x1a, 0x18, 0x2e,
    0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x44, 0x65,
    0x6c, 0x69, 0x76, 0x65, 0x72, 0x54, 0x78, 0x12, 0x38, 0x0a, 0x07, 0x43, 0x68, 0x65, 0x63, 0x6b,
    0x54, 0x78, 0x12, 0x15, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x54, 0x78, 0x1a, 0x16, 0x2e, 0x74, 0x79, 0x70, 0x65,
    0x73, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x54,
    0x78, 0x12, 0x32, 0x0a, 0x05, 0x51, 0x75, 0x65, 0x72, 0x79, 0x12, 0x13, 0x2e, 0x74, 0x79, 0x70,
    0x65, 0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x51, 0x75, 0x65, 0x72, 0x79, 0x1a,
    0x14, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x51, 0x75, 0x65, 0x72, 0x79, 0x12, 0x35, 0x0a, 0x06, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x12,
    0x14, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x43,
    0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x1a, 0x15, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x12, 0x3e, 0x0a, 0x09,
    0x49, 0x6e, 0x69, 0x74, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x12, 0x17, 0x2e, 0x74, 0x79, 0x70, 0x65,
    0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x49, 0x6e, 0x69, 0x74, 0x43, 0x68, 0x61,
    0x69, 0x6e, 0x1a, 0x18, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x49, 0x6e, 0x69, 0x74, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x12, 0x41, 0x0a, 0x0a,
    0x42, 0x65, 0x67, 0x69, 0x6e, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x18, 0x2e, 0x74, 0x79, 0x70,
    0x65, 0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x42, 0x65, 0x67, 0x69, 0x6e, 0x42,
    0x6c, 0x6f, 0x63, 0x6b, 0x1a, 0x19, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x65, 0x67, 0x69, 0x6e, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12,
    0x3b, 0x0a, 0x08, 0x45, 0x6e, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x16, 0x2e, 0x74, 0x79,
    0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x45, 0x6e, 0x64, 0x42, 0x6c,
    0x6f, 0x63, 0x6b, 0x1a, 0x17, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x45, 0x6e, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x4a, 0xef, 0x47, 0x0a,
    0x07, 0x12, 0x05, 0x00, 0x00, 0x88, 0x02, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x0d, 0x0a, 0x92, 0x02, 0x0a,
    0x02, 0x05, 0x00, 0x12, 0x04, 0x0c, 0x00, 0x1b, 0x01, 0x1a, 0x8d, 0x01, 0x20, 0x4e, 0x6f, 0x74,
    0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x75, 0x73, 0x65, 0x64, 0x0a, 0x20, 0x43, 0x6f, 0x75,
    0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x61, 0x64, 0x64, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a,
    0x20, 0x73, 0x6f, 0x20, 0x77, 0x65, 0x20, 0x64, 0x6f, 0x6e, 0x27, 0x74, 0x20, 0x68, 0x61, 0x76,
    0x65, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x73, 0x77, 0x69, 0x74, 0x63, 0x68,
    0x0a, 0x20, 0x28, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x74, 0x77, 0x69, 0x63,
    0x65, 0x20, 0x61, 0x73, 0x20, 0x66, 0x61, 0x73, 0x74, 0x2c, 0x20, 0x62, 0x75, 0x74, 0x20, 0x77,
    0x65, 0x27, 0x72, 0x65, 0x20, 0x74, 0x61, 0x6c, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x62, 0x6f,
    0x75, 0x74, 0x20, 0x31, 0x35, 0x6e, 0x73, 0x29, 0x0a, 0x32, 0x3c, 0x20, 0x54, 0x68, 0x69, 0x73,
    0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x70, 0x69, 0x65, 0x64, 0x20,
    0x66, 0x72, 0x6f, 0x6d, 0x20, 0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x67, 0x69, 0x74, 0x68,
    0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x6d, 0x69, 0x6e,
    0x74, 0x2f, 0x61, 0x62, 0x63, 0x69, 0x0a, 0x32, 0x38, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d,
    0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d,
    0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d,
    0x2d, 0x0a, 0x20, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x05, 0x10, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x0d, 0x16, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x0f, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x08,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0f, 0x1a, 0x1e, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x10, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x10, 0x1a, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x11, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x11, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x11, 0x1a,
    0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x12, 0x08, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x12, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x12, 0x1a, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x05, 0x12, 0x03, 0x13, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x13, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03,
    0x13, 0x1a, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x14, 0x08, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x14, 0x08, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x14, 0x1a, 0x1e, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x07, 0x12, 0x03, 0x15, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x07, 0x01, 0x12, 0x03, 0x15, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02,
    0x12, 0x03, 0x15, 0x1a, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x16,
    0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x16, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x16, 0x1a, 0x1e, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x09, 0x12, 0x03, 0x17, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x17, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x09, 0x02, 0x12, 0x03, 0x17, 0x1a, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a, 0x12,
    0x03, 0x18, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x18,
    0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x18, 0x1a, 0x1e,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x19, 0x08, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x19, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x0b, 0x02, 0x12, 0x03, 0x19, 0x1a, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x0c, 0x12, 0x03, 0x1a, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x01, 0x12,
    0x03, 0x1a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x1a,
    0x1a, 0x1e, 0x0a, 0x41, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x20, 0x00, 0x46, 0x01, 0x32, 0x35,
    0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d,
    0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d,
    0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x0a, 0x20, 0x43, 0x6f, 0x64, 0x65, 0x20, 0x74,
    0x79, 0x70, 0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x20, 0x05,
    0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x21, 0x08, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x08, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x21, 0x20, 0x21, 0x0a, 0x2d, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x01, 0x12, 0x03, 0x24, 0x08, 0x22, 0x1a, 0x20, 0x20, 0x47, 0x65, 0x6e, 0x65, 0x72, 0x61,
    0x6c, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x73,
    0x2c, 0x20, 0x30, 0x20, 0x7e, 0x20, 0x39, 0x39, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x24, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02,
    0x12, 0x03, 0x24, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03, 0x25,
    0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x25, 0x08, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x25, 0x20, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x01, 0x02, 0x03, 0x12, 0x03, 0x26, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x26, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x03, 0x02, 0x12, 0x03, 0x26, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x04, 0x12,
    0x03, 0x27, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x27,
    0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x04, 0x02, 0x12, 0x03, 0x27, 0x20, 0x21,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x05, 0x12, 0x03, 0x28, 0x08, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x28, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x05, 0x02, 0x12, 0x03, 0x28, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02,
    0x06, 0x12, 0x03, 0x29, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x29, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x06, 0x02, 0x12, 0x03, 0x29,
    0x20, 0x21, 0x0a, 0x2f, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x07, 0x12, 0x03, 0x2c, 0x08, 0x24, 0x1a,
    0x22, 0x20, 0x52, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x62,
    0x61, 0x73, 0x65, 0x63, 0x6f, 0x69, 0x6e, 0x2c, 0x20, 0x31, 0x30, 0x30, 0x20, 0x7e, 0x20, 0x31,
    0x39, 0x39, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x2c, 0x08,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x07, 0x02, 0x12, 0x03, 0x2c, 0x20, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x08, 0x12, 0x03, 0x2d, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x08, 0x01, 0x12, 0x03, 0x2d, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x08, 0x02, 0x12, 0x03, 0x2d, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x09,
    0x12, 0x03, 0x2e, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x09, 0x01, 0x12, 0x03,
    0x2e, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x09, 0x02, 0x12, 0x03, 0x2e, 0x20,
    0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x0a, 0x12, 0x03, 0x2f, 0x08, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x2f, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x0b, 0x12, 0x03, 0x30, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0b, 0x01,
    0x12, 0x03, 0x30, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0b, 0x02, 0x12, 0x03,
    0x30, 0x23, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x0c, 0x12, 0x03, 0x31, 0x08, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x31, 0x08, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x31, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x01, 0x02, 0x0d, 0x12, 0x03, 0x32, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x0d, 0x01, 0x12, 0x03, 0x32, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0d, 0x02,
    0x12, 0x03, 0x32, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x0e, 0x12, 0x03, 0x33,
    0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x33, 0x08, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0e, 0x02, 0x12, 0x03, 0x33, 0x20, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x01, 0x02, 0x0f, 0x12, 0x03, 0x34, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x34, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x0f, 0x02, 0x12, 0x03, 0x34, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x10, 0x12,
    0x03, 0x35, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x10, 0x01, 0x12, 0x03, 0x35,
    0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x10, 0x02, 0x12, 0x03, 0x35, 0x20, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x11, 0x12, 0x03, 0x36, 0x08, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x11, 0x01, 0x12, 0x03, 0x36, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x11, 0x02, 0x12, 0x03, 0x36, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02,
    0x12, 0x12, 0x03, 0x37, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x12, 0x01, 0x12,
    0x03, 0x37, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x12, 0x02, 0x12, 0x03, 0x37,
    0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x13, 0x12, 0x03, 0x38, 0x08, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x13, 0x01, 0x12, 0x03, 0x38, 0x08, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x13, 0x02, 0x12, 0x03, 0x38, 0x20, 0x23, 0x0a, 0x31, 0x0a, 0x04, 0x05,
    0x01, 0x02, 0x14, 0x12, 0x03, 0x3b, 0x08, 0x24, 0x1a, 0x24, 0x20, 0x52, 0x65, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x67, 0x6f, 0x76, 0x65, 0x72, 0x6e, 0x61, 0x6e,
    0x63, 0x65, 0x2c, 0x20, 0x32, 0x30, 0x30, 0x20, 0x7e, 0x20, 0x32, 0x39, 0x39, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x14, 0x01, 0x12, 0x03, 0x3b, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x14, 0x02, 0x12, 0x03, 0x3b, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x15, 0x12, 0x03, 0x3c, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x15, 0x01,
    0x12, 0x03, 0x3c, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x15, 0x02, 0x12, 0x03,
    0x3c, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x16, 0x12, 0x03, 0x3d, 0x08, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x16, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x16, 0x02, 0x12, 0x03, 0x3d, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x01, 0x02, 0x17, 0x12, 0x03, 0x3e, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x17, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x17, 0x02,
    0x12, 0x03, 0x3e, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x18, 0x12, 0x03, 0x3f,
    0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x18, 0x01, 0x12, 0x03, 0x3f, 0x08, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x18, 0x02, 0x12, 0x03, 0x3f, 0x20, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x01, 0x02, 0x19, 0x12, 0x03, 0x40, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x19, 0x01, 0x12, 0x03, 0x40, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x19, 0x02, 0x12, 0x03, 0x40, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x1a, 0x12,
    0x03, 0x41, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x1a, 0x01, 0x12, 0x03, 0x41,
    0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x1a, 0x02, 0x12, 0x03, 0x41, 0x20, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x1b, 0x12, 0x03, 0x42, 0x08, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x1b, 0x01, 0x12, 0x03, 0x42, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x1b, 0x02, 0x12, 0x03, 0x42, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02,
    0x1c, 0x12, 0x03, 0x43, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x1c, 0x01, 0x12,
    0x03, 0x43, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x1c, 0x02, 0x12, 0x03, 0x43,
    0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x1d, 0x12, 0x03, 0x44, 0x08, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x1d, 0x01, 0x12, 0x03, 0x44, 0x08, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x1d, 0x02, 0x12, 0x03, 0x44, 0x20, 0x23, 0x0a, 0x44, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x4b, 0x00, 0x59, 0x01, 0x32, 0x38, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d,
    0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d,
    0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d,
    0x2d, 0x0a, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x4b, 0x08, 0x0f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x00, 0x08, 0x00, 0x12, 0x04, 0x4c, 0x08, 0x58, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x08, 0x00, 0x01, 0x12, 0x03, 0x4c, 0x0e, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x4d, 0x10, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x4d, 0x10, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4d,
    0x1c, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4d, 0x23, 0x24,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x4e, 0x10, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x4e, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4e, 0x1d, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x4e, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x4f, 0x10, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x4f,
    0x10, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4f, 0x1c, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4f, 0x23, 0x24, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x50, 0x10, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x50, 0x10, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x50, 0x21, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x50, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x51,
    0x10, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x51, 0x10, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x51, 0x21, 0x2b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x51, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x52, 0x10, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x06, 0x12, 0x03, 0x52, 0x10, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x52, 0x1f, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03,
    0x52, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x53, 0x10, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x53, 0x10, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x53, 0x1e, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x53, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x07, 0x12, 0x03, 0x54, 0x10, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x06,
    0x12, 0x03, 0x54, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x54, 0x1d, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x54, 0x25,
    0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x55, 0x10, 0x30, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x06, 0x12, 0x03, 0x55, 0x10, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x55, 0x21, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x08, 0x03, 0x12, 0x03, 0x55, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09,
    0x12, 0x03, 0x56, 0x10, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x06, 0x12, 0x03,
    0x56, 0x10, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x56, 0x22,
    0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x56, 0x30, 0x32, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x57, 0x10, 0x2f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0a, 0x06, 0x12, 0x03, 0x57, 0x10, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0a, 0x01, 0x12, 0x03, 0x57, 0x20, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a,
    0x03, 0x12, 0x03, 0x57, 0x2c, 0x2e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x5b, 0x00,
    0x5d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x5b, 0x08, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x5c, 0x08, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x5c, 0x08, 0x5b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x5c, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x5c, 0x0f, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x5c, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x5f, 0x00, 0x60, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x5f, 0x08, 0x14, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x62, 0x00, 0x63, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x03, 0x62, 0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x65, 0x00, 0x68, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x65, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x66, 0x08, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x66, 0x08, 0x65, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x66, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x66, 0x0f, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x66,
    0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x67, 0x08, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x04, 0x67, 0x08, 0x66, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x67, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x67, 0x0f, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x67, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04,
    0x6a, 0x00, 0x6c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x6a, 0x08, 0x18,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x6b, 0x08, 0x15, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x6b, 0x08, 0x6a, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6b, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x6b, 0x0e, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x6b, 0x13, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x6e, 0x00,
    0x70, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x6e, 0x08, 0x16, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x6f, 0x08, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x04, 0x12, 0x04, 0x6f, 0x08, 0x6e, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x6f, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x6f, 0x0e, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x6f, 0x13, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x72, 0x00, 0x77, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x72, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x73, 0x08, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x73, 0x08, 0x72, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x73, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x73, 0x0e, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x73,
    0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x74, 0x08, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04, 0x74, 0x08, 0x73, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x74, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x74, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x74, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02,
    0x12, 0x03, 0x75, 0x08, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x75, 0x08, 0x74, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05, 0x12, 0x03, 0x75,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x75, 0x0f, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x75, 0x18, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x03, 0x12, 0x03, 0x76, 0x08, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x03, 0x04, 0x12, 0x04, 0x76, 0x08, 0x75, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x76, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x76, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x76, 0x15, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x79, 0x00, 0x7a, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x79, 0x08, 0x15, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x09, 0x12, 0x04, 0x7c, 0x00, 0x7e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12,
    0x03, 0x7c, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x7d, 0x08,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7d, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x03, 0x7d, 0x11, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7d, 0x1b, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7d, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0a, 0x12,
    0x06, 0x80, 0x01, 0x00, 0x83, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x04,
    0x80, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x04, 0x81, 0x01,
    0x08, 0x17, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x06, 0x81, 0x01, 0x08,
    0x80, 0x01, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x04, 0x81, 0x01,
    0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0x81, 0x01, 0x0e,
    0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x04, 0x81, 0x01, 0x15, 0x16,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x04, 0x82, 0x01, 0x08, 0x1a, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x06, 0x82, 0x01, 0x08, 0x81, 0x01, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x06, 0x12, 0x04, 0x82, 0x01, 0x08, 0x0e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x04, 0x82, 0x01, 0x0f, 0x15, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x04, 0x82, 0x01, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x0b, 0x12, 0x06, 0x85, 0x01, 0x00, 0x87, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b,
    0x01, 0x12, 0x04, 0x85, 0x01, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12,
    0x04, 0x86, 0x01, 0x08, 0x1a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x06,
    0x86, 0x01, 0x08, 0x85, 0x01, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12,
    0x04, 0x86, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x04,
    0x86, 0x01, 0x0f, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x04, 0x86,
    0x01, 0x18, 0x19, 0x0a, 0x47, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0x8d, 0x01, 0x00, 0x9c, 0x01,
    0x01, 0x32, 0x39, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d,
    0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d,
    0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x0a, 0x20, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x0a, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x0c, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0c, 0x08,
    0x00, 0x12, 0x06, 0x8e, 0x01, 0x08, 0x9b, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x08,
    0x00, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x0e, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00,
    0x12, 0x04, 0x8f, 0x01, 0x10, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x06, 0x12,
    0x04, 0x8f, 0x01, 0x10, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x04,
    0x8f, 0x01, 0x22, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8f,
    0x01, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x04, 0x90, 0x01, 0x10,
    0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x06, 0x12, 0x04, 0x90, 0x01, 0x10, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x04, 0x90, 0x01, 0x1d, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x04, 0x90, 0x01, 0x24, 0x25, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x04, 0x91, 0x01, 0x10, 0x28, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x02, 0x06, 0x12, 0x04, 0x91, 0x01, 0x10, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x02, 0x01, 0x12, 0x04, 0x91, 0x01, 0x1e, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x02, 0x03, 0x12, 0x04, 0x91, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02,
    0x03, 0x12, 0x04, 0x92, 0x01, 0x10, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x06,
    0x12, 0x04, 0x92, 0x01, 0x10, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x01, 0x12,
    0x04, 0x92, 0x01, 0x1d, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x03, 0x12, 0x04,
    0x92, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x04, 0x12, 0x04, 0x93, 0x01,
    0x10, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x06, 0x12, 0x04, 0x93, 0x01, 0x10,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x01, 0x12, 0x04, 0x93, 0x01, 0x22, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x03, 0x12, 0x04, 0x93, 0x01, 0x2f, 0x30, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x05, 0x12, 0x04, 0x94, 0x01, 0x10, 0x31, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x05, 0x06, 0x12, 0x04, 0x94, 0x01, 0x10, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x05, 0x01, 0x12, 0x04, 0x94, 0x01, 0x22, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x05, 0x03, 0x12, 0x04, 0x94, 0x01, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c,
    0x02, 0x06, 0x12, 0x04, 0x95, 0x01, 0x10, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06,
    0x06, 0x12, 0x04, 0x95, 0x01, 0x10, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x01,
    0x12, 0x04, 0x95, 0x01, 0x20, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x03, 0x12,
    0x04, 0x95, 0x01, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x07, 0x12, 0x04, 0x96,
    0x01, 0x10, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x07, 0x06, 0x12, 0x04, 0x96, 0x01,
    0x10, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x07, 0x01, 0x12, 0x04, 0x96, 0x01, 0x1f,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x07, 0x03, 0x12, 0x04, 0x96, 0x01, 0x28, 0x29,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x08, 0x12, 0x04, 0x97, 0x01, 0x10, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x08, 0x06, 0x12, 0x04, 0x97, 0x01, 0x10, 0x1d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x08, 0x01, 0x12, 0x04, 0x97, 0x01, 0x1e, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x08, 0x03, 0x12, 0x04, 0x97, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0c, 0x02, 0x09, 0x12, 0x04, 0x98, 0x01, 0x10, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x09, 0x06, 0x12, 0x04, 0x98, 0x01, 0x10, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x09,
    0x01, 0x12, 0x04, 0x98, 0x01, 0x22, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x09, 0x03,
    0x12, 0x04, 0x98, 0x01, 0x2f, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x0a, 0x12, 0x04,
    0x99, 0x01, 0x10, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x99,
    0x01, 0x10, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0a, 0x01, 0x12, 0x04, 0x99, 0x01,
    0x23, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0a, 0x03, 0x12, 0x04, 0x99, 0x01, 0x31,
    0x33, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x0b, 0x12, 0x04, 0x9a, 0x01, 0x10, 0x30, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0b, 0x06, 0x12, 0x04, 0x9a, 0x01, 0x10, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0b, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x21, 0x2a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x0b, 0x03, 0x12, 0x04, 0x9a, 0x01, 0x2d, 0x2f, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x0d, 0x12, 0x06, 0x9e, 0x01, 0x00, 0xa0, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d,
    0x01, 0x12, 0x04, 0x9e, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12,
    0x04, 0x9f, 0x01, 0x08, 0x19, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x06,
    0x9f, 0x01, 0x08, 0x9e, 0x01, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12,
    0x04, 0x9f, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04,
    0x9f, 0x01, 0x0f, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9f,
    0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0xa2, 0x01, 0x00, 0xa4, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x14, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x1b, 0x0a, 0x0f, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x06, 0xa3, 0x01, 0x08, 0xa2, 0x01, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x0f, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa3, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f,
    0x12, 0x06, 0xa6, 0x01, 0x00, 0xa7, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12,
    0x04, 0xa6, 0x01, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0xa9, 0x01, 0x00,
    0xae, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x14,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0xaa, 0x01, 0x08, 0x18, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x06, 0xaa, 0x01, 0x08, 0xa9, 0x01, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x05, 0x12, 0x04, 0xaa, 0x01, 0x08, 0x0e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x0f, 0x13, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0xaa, 0x01, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x10, 0x02, 0x01, 0x12, 0x04, 0xab, 0x01, 0x08, 0x1b, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x01, 0x04, 0x12, 0x06, 0xab, 0x01, 0x08, 0xaa, 0x01, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x01, 0x05, 0x12, 0x04, 0xab, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xab, 0x01, 0x0f, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xab, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x02,
    0x12, 0x04, 0xac, 0x01, 0x08, 0x25, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x04, 0x12,
    0x06, 0xac, 0x01, 0x08, 0xab, 0x01, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x05,
    0x12, 0x04, 0xac, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xac, 0x01, 0x0f, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xac, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x03, 0x12, 0x04, 0xad, 0x01,
    0x08, 0x26, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x04, 0x12, 0x06, 0xad, 0x01, 0x08,
    0xac, 0x01, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x05, 0x12, 0x04, 0xad, 0x01,
    0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x01, 0x12, 0x04, 0xad, 0x01, 0x0e,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x03, 0x12, 0x04, 0xad, 0x01, 0x24, 0x25,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x06, 0xb0, 0x01, 0x00, 0xb2, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x11, 0x02, 0x00, 0x12, 0x04, 0xb1, 0x01, 0x08, 0x17, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x00, 0x04, 0x12, 0x06, 0xb1, 0x01, 0x08, 0xb0, 0x01, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xb1, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xb1, 0x01, 0x0f, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xb1, 0x01, 0x15, 0x16, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0xb4,
    0x01, 0x00, 0xb8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0xb4, 0x01,
    0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0xb5, 0x01, 0x08, 0x2a,
    0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x06, 0xb5, 0x01, 0x08, 0xb4, 0x01,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb5, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb5, 0x01, 0x1a, 0x1e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb5, 0x01, 0x28, 0x29, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x2a, 0x0a, 0x0f, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x06, 0xb6, 0x01, 0x08, 0xb5, 0x01, 0x2a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x1a, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb6, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12,
    0x02, 0x02, 0x12, 0x04, 0xb7, 0x01, 0x08, 0x2a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02,
    0x04, 0x12, 0x06, 0xb7, 0x01, 0x08, 0xb6, 0x01, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x02, 0x05, 0x12, 0x04, 0xb7, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xb7, 0x01, 0x1a, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xb7, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0xba, 0x01,
    0x00, 0xbe, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0xba, 0x01, 0x08,
    0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x2a, 0x0a,
    0x0f, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x06, 0xbb, 0x01, 0x08, 0xba, 0x01, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x06, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x1a, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbb, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x04, 0xbc, 0x01, 0x08, 0x2a, 0x0a, 0x0f, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x01, 0x04, 0x12, 0x06, 0xbc, 0x01, 0x08, 0xbb, 0x01, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x01, 0x05, 0x12, 0x04, 0xbc, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x01, 0x01, 0x12, 0x04, 0xbc, 0x01, 0x1a, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xbc, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02,
    0x02, 0x12, 0x04, 0xbd, 0x01, 0x08, 0x2a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x04,
    0x12, 0x06, 0xbd, 0x01, 0x08, 0xbc, 0x01, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02,
    0x05, 0x12, 0x04, 0xbd, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xbd, 0x01, 0x1a, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xbd, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xc0, 0x01, 0x00,
    0xc8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xc0, 0x01, 0x08, 0x15,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0xc1, 0x01, 0x08, 0x2a, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x06, 0xc1, 0x01, 0x08, 0xc0, 0x01, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x06, 0x12, 0x04, 0xc1, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc1, 0x01, 0x1a, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc1, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x14, 0x02, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x08, 0x2a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x01, 0x04, 0x12, 0x06, 0xc2, 0x01, 0x08, 0xc1, 0x01, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc2, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x1a, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xc2, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x02,
    0x12, 0x04, 0xc3, 0x01, 0x08, 0x2a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x04, 0x12,
    0x06, 0xc3, 0x01, 0x08, 0xc2, 0x01, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x05,
    0x12, 0x04, 0xc3, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xc3, 0x01, 0x1a, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xc3, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x03, 0x12, 0x04, 0xc4, 0x01,
    0x08, 0x2a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x04, 0x12, 0x06, 0xc4, 0x01, 0x08,
    0xc3, 0x01, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x05, 0x12, 0x04, 0xc4, 0x01,
    0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x1a,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x03, 0x12, 0x04, 0xc4, 0x01, 0x28, 0x29,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x04, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x2a, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x04, 0x12, 0x06, 0xc5, 0x01, 0x08, 0xc4, 0x01, 0x2a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x05, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x0d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x01, 0x12, 0x04, 0xc5, 0x01, 0x1a, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x04, 0x03, 0x12, 0x04, 0xc5, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x14, 0x02, 0x05, 0x12, 0x04, 0xc6, 0x01, 0x08, 0x2a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x05, 0x04, 0x12, 0x06, 0xc6, 0x01, 0x08, 0xc5, 0x01, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x05, 0x05, 0x12, 0x04, 0xc6, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x05, 0x01, 0x12, 0x04, 0xc6, 0x01, 0x1a, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x05, 0x03, 0x12, 0x04, 0xc6, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x06,
    0x12, 0x04, 0xc7, 0x01, 0x08, 0x2a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x06, 0x04, 0x12,
    0x06, 0xc7, 0x01, 0x08, 0xc6, 0x01, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x06, 0x05,
    0x12, 0x04, 0xc7, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x06, 0x01, 0x12,
    0x04, 0xc7, 0x01, 0x1a, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x06, 0x03, 0x12, 0x04,
    0xc7, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xca, 0x01, 0x00, 0xce,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xca, 0x01, 0x08, 0x16, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0xcb, 0x01, 0x08, 0x2a, 0x0a, 0x0f, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x06, 0xcb, 0x01, 0x08, 0xca, 0x01, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x06, 0x12, 0x04, 0xcb, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcb, 0x01, 0x1a, 0x1e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0xcb, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x15, 0x02, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x08, 0x2a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x01, 0x04, 0x12, 0x06, 0xcc, 0x01, 0x08, 0xcb, 0x01, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xcc, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x1a, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xcc, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x02, 0x12,
    0x04, 0xcd, 0x01, 0x08, 0x2a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x04, 0x12, 0x06,
    0xcd, 0x01, 0x08, 0xcc, 0x01, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x05, 0x12,
    0x04, 0xcd, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x01, 0x12, 0x04,
    0xcd, 0x01, 0x1a, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x03, 0x12, 0x04, 0xcd,
    0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0xd1, 0x01, 0x00, 0xd2, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0xd1, 0x01, 0x08, 0x19, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0xd4, 0x01, 0x00, 0xd5, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x17, 0x01, 0x12, 0x04, 0xd4, 0x01, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12,
    0x06, 0xd7, 0x01, 0x00, 0xd9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04,
    0xd7, 0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xd8, 0x01,
    0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd8, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x06, 0x12, 0x04, 0xd8, 0x01, 0x11, 0x1a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd8, 0x01, 0x1b, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd8, 0x01, 0x23, 0x24, 0x0a, 0x49,
    0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xde, 0x01, 0x00, 0xe8, 0x01, 0x01, 0x32, 0x3b, 0x2d, 0x2d,
    0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d,
    0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d,
    0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x0a, 0x20, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x63, 0x68, 0x61,
    0x69, 0x6e, 0x20, 0x54, 0x79, 0x70, 0x65, 0x73, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01,
    0x12, 0x04, 0xde, 0x01, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04,
    0xdf, 0x01, 0x08, 0x1c, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x06, 0xdf,
    0x01, 0x08, 0xde, 0x01, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xdf, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdf,
    0x01, 0x0f, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x04, 0xdf, 0x01,
    0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x08, 0x1a,
    0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x04, 0x12, 0x06, 0xe0, 0x01, 0x08, 0xdf, 0x01,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe0, 0x01, 0x08, 0x0e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x0f, 0x15, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe0, 0x01, 0x18, 0x19, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x19, 0x02, 0x02, 0x12, 0x04, 0xe1, 0x01, 0x08, 0x18, 0x0a, 0x0f, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x02, 0x04, 0x12, 0x06, 0xe1, 0x01, 0x08, 0xe0, 0x01, 0x1a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x02, 0x05, 0x12, 0x04, 0xe1, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe1, 0x01, 0x0f, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x02, 0x03, 0x12, 0x04, 0xe1, 0x01, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19,
    0x02, 0x03, 0x12, 0x04, 0xe2, 0x01, 0x08, 0x1b, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03,
    0x04, 0x12, 0x06, 0xe2, 0x01, 0x08, 0xe1, 0x01, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x03, 0x05, 0x12, 0x04, 0xe2, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03,
    0x01, 0x12, 0x04, 0xe2, 0x01, 0x0f, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x03,
    0x12, 0x04, 0xe2, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x04, 0x12, 0x04,
    0xe3, 0x01, 0x08, 0x22, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x04, 0x04, 0x12, 0x06, 0xe3,
    0x01, 0x08, 0xe2, 0x01, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x04, 0x06, 0x12, 0x04,
    0xe3, 0x01, 0x08, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x04, 0x01, 0x12, 0x04, 0xe3,
    0x01, 0x10, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x04, 0x03, 0x12, 0x04, 0xe3, 0x01,
    0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x05, 0x12, 0x04, 0xe4, 0x01, 0x08, 0x23,
    0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x05, 0x04, 0x12, 0x06, 0xe4, 0x01, 0x08, 0xe3, 0x01,
    0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x05, 0x05, 0x12, 0x04, 0xe4, 0x01, 0x08, 0x0d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x05, 0x01, 0x12, 0x04, 0xe4, 0x01, 0x0e, 0x1e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x05, 0x03, 0x12, 0x04, 0xe4, 0x01, 0x21, 0x22, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x19, 0x02, 0x06, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x1c, 0x0a, 0x0f, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x06, 0x04, 0x12, 0x06, 0xe5, 0x01, 0x08, 0xe4, 0x01, 0x23, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x06, 0x05, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x06, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x0e, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x06, 0x03, 0x12, 0x04, 0xe5, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19,
    0x02, 0x07, 0x12, 0x04, 0xe6, 0x01, 0x08, 0x22, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x07,
    0x04, 0x12, 0x06, 0xe6, 0x01, 0x08, 0xe5, 0x01, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x07, 0x05, 0x12, 0x04, 0xe6, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x07,
    0x01, 0x12, 0x04, 0xe6, 0x01, 0x0e, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x07, 0x03,
    0x12, 0x04, 0xe6, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x08, 0x12, 0x04,
    0xe7, 0x01, 0x08, 0x1b, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x08, 0x04, 0x12, 0x06, 0xe7,
    0x01, 0x08, 0xe6, 0x01, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x08, 0x05, 0x12, 0x04,
    0xe7, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x08, 0x01, 0x12, 0x04, 0xe7,
    0x01, 0x0e, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x08, 0x03, 0x12, 0x04, 0xe7, 0x01,
    0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06, 0xea, 0x01, 0x00, 0xed, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0xea, 0x01, 0x08, 0x0f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0xeb, 0x01, 0x08, 0x17, 0x0a, 0x0f, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x00, 0x04, 0x12, 0x06, 0xeb, 0x01, 0x08, 0xea, 0x01, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x02, 0x00, 0x05, 0x12, 0x04, 0xeb, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x0e, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xeb, 0x01, 0x15, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02,
    0x01, 0x12, 0x04, 0xec, 0x01, 0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x04,
    0x12, 0x06, 0xec, 0x01, 0x08, 0xeb, 0x01, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01,
    0x06, 0x12, 0x04, 0xec, 0x01, 0x08, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xec, 0x01, 0x16, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xec, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0xef, 0x01, 0x00,
    0xf2, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0xef, 0x01, 0x08, 0x15,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0xf0, 0x01, 0x08, 0x19, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x06, 0xf0, 0x01, 0x08, 0xef, 0x01, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x05, 0x12, 0x04, 0xf0, 0x01, 0x08, 0x0e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf0, 0x01, 0x0f, 0x14, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf0, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1b, 0x02, 0x01, 0x12, 0x04, 0xf1, 0x01, 0x08, 0x17, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x01, 0x04, 0x12, 0x06, 0xf1, 0x01, 0x08, 0xf0, 0x01, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x01, 0x05, 0x12, 0x04, 0xf1, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xf1, 0x01, 0x0e, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xf1, 0x01, 0x15, 0x16, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1c, 0x12, 0x06,
    0xf4, 0x01, 0x00, 0xf7, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04, 0xf4,
    0x01, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0xf5, 0x01, 0x08,
    0x19, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x04, 0x12, 0x06, 0xf5, 0x01, 0x08, 0xf4,
    0x01, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x05, 0x12, 0x04, 0xf5, 0x01, 0x08,
    0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf5, 0x01, 0x0e, 0x14,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf5, 0x01, 0x17, 0x18, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x01, 0x12, 0x04, 0xf6, 0x01, 0x08, 0x19, 0x0a, 0x0f, 0x0a,
    0x05, 0x04, 0x1c, 0x02, 0x01, 0x04, 0x12, 0x06, 0xf6, 0x01, 0x08, 0xf5, 0x01, 0x19, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x05, 0x12, 0x04, 0xf6, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1c, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf6, 0x01, 0x0f, 0x14, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1c, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf6, 0x01, 0x17, 0x18, 0x0a, 0x4b, 0x0a, 0x02, 0x06,
    0x00, 0x12, 0x06, 0xfc, 0x01, 0x00, 0x88, 0x02, 0x01, 0x32, 0x3d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d,
    0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d,
    0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d, 0x2d,
    0x2d, 0x2d, 0x2d, 0x0a, 0x20, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x44, 0x65, 0x66,
    0x69, 0x6e, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12,
    0x04, 0xfc, 0x01, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x04, 0xfd,
    0x01, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfd, 0x01,
    0x0c, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xfd, 0x01, 0x11,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfd, 0x01, 0x27, 0x33,
    0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x04, 0xfe, 0x01, 0x08, 0x38, 0x0a, 0x0d,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xfe, 0x01, 0x0c, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xfe, 0x01, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xfe, 0x01, 0x29, 0x36, 0x0a, 0x0c, 0x0a, 0x04, 0x06,
    0x00, 0x02, 0x02, 0x12, 0x04, 0xff, 0x01, 0x08, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x04, 0xff, 0x01, 0x0c, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02,
    0x02, 0x12, 0x04, 0xff, 0x01, 0x11, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xff, 0x01, 0x27, 0x33, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x03, 0x12, 0x04,
    0x80, 0x02, 0x08, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x80,
    0x02, 0x0c, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0x80, 0x02,
    0x16, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x03, 0x12, 0x04, 0x80, 0x02, 0x31,
    0x42, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x04, 0x12, 0x04, 0x81, 0x02, 0x08, 0x44, 0x0a,
    0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0x81, 0x02, 0x0c, 0x15, 0x0a, 0x0d,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0x81, 0x02, 0x16, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x04, 0x03, 0x12, 0x04, 0x81, 0x02, 0x31, 0x42, 0x0a, 0x0c, 0x0a, 0x04,
    0x06, 0x00, 0x02, 0x05, 0x12, 0x04, 0x82, 0x02, 0x08, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x05, 0x01, 0x12, 0x04, 0x82, 0x02, 0x0c, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x05, 0x02, 0x12, 0x04, 0x82, 0x02, 0x14, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x05,
    0x03, 0x12, 0x04, 0x82, 0x02, 0x2d, 0x3c, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x06, 0x12,
    0x04, 0x83, 0x02, 0x08, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x06, 0x01, 0x12, 0x04,
    0x83, 0x02, 0x0c, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x06, 0x02, 0x12, 0x04, 0x83,
    0x02, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x06, 0x03, 0x12, 0x04, 0x83, 0x02,
    0x29, 0x36, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x07, 0x12, 0x04, 0x84, 0x02, 0x08, 0x3b,
    0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x07, 0x01, 0x12, 0x04, 0x84, 0x02, 0x0c, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x07, 0x02, 0x12, 0x04, 0x84, 0x02, 0x13, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x07, 0x03, 0x12, 0x04, 0x84, 0x02, 0x2b, 0x39, 0x0a, 0x0c, 0x0a,
    0x04, 0x06, 0x00, 0x02, 0x08, 0x12, 0x04, 0x85, 0x02, 0x08, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x08, 0x01, 0x12, 0x04, 0x85, 0x02, 0x0c, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x08, 0x02, 0x12, 0x04, 0x85, 0x02, 0x16, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x08, 0x03, 0x12, 0x04, 0x85, 0x02, 0x31, 0x42, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x09,
    0x12, 0x04, 0x86, 0x02, 0x08, 0x47, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x09, 0x01, 0x12,
    0x04, 0x86, 0x02, 0x0c, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x09, 0x02, 0x12, 0x04,
    0x86, 0x02, 0x17, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x09, 0x03, 0x12, 0x04, 0x86,
    0x02, 0x33, 0x45, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x0a, 0x12, 0x04, 0x87, 0x02, 0x08,
    0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x04, 0x87, 0x02, 0x0c, 0x14,
    0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x04, 0x87, 0x02, 0x15, 0x24, 0x0a,
    0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x04, 0x87, 0x02, 0x2f, 0x3f, 0x62, 0x06,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];

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
