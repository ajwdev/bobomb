// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
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
pub struct OkReply {
    // message fields
    pub message: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OkReply {}

impl OkReply {
    pub fn new() -> OkReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OkReply {
        static mut instance: ::protobuf::lazy::Lazy<OkReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OkReply,
        };
        unsafe {
            instance.get(OkReply::new)
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

impl ::protobuf::Message for OkReply {
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
        if self.message != ::std::string::String::new() {
            my_size += ::protobuf::rt::string_size(1, &self.message);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.message != ::std::string::String::new() {
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

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OkReply {
    fn new() -> OkReply {
        OkReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<OkReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    OkReply::get_message_for_reflect,
                    OkReply::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OkReply>(
                    "OkReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OkReply {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OkReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OkReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StopRequest {
    // message fields
    pub program_counter: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StopRequest {}

impl StopRequest {
    pub fn new() -> StopRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StopRequest {
        static mut instance: ::protobuf::lazy::Lazy<StopRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StopRequest,
        };
        unsafe {
            instance.get(StopRequest::new)
        }
    }

    // uint32 program_counter = 1;

    pub fn clear_program_counter(&mut self) {
        self.program_counter = 0;
    }

    // Param is passed by value, moved
    pub fn set_program_counter(&mut self, v: u32) {
        self.program_counter = v;
    }

    pub fn get_program_counter(&self) -> u32 {
        self.program_counter
    }

    fn get_program_counter_for_reflect(&self) -> &u32 {
        &self.program_counter
    }

    fn mut_program_counter_for_reflect(&mut self) -> &mut u32 {
        &mut self.program_counter
    }
}

impl ::protobuf::Message for StopRequest {
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
                    let tmp = is.read_uint32()?;
                    self.program_counter = tmp;
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
        if self.program_counter != 0 {
            my_size += ::protobuf::rt::value_size(1, self.program_counter, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.program_counter != 0 {
            os.write_uint32(1, self.program_counter)?;
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

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StopRequest {
    fn new() -> StopRequest {
        StopRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<StopRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "program_counter",
                    StopRequest::get_program_counter_for_reflect,
                    StopRequest::mut_program_counter_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StopRequest>(
                    "StopRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StopRequest {
    fn clear(&mut self) {
        self.clear_program_counter();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StopRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StopRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ContinueRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ContinueRequest {}

impl ContinueRequest {
    pub fn new() -> ContinueRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ContinueRequest {
        static mut instance: ::protobuf::lazy::Lazy<ContinueRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ContinueRequest,
        };
        unsafe {
            instance.get(ContinueRequest::new)
        }
    }
}

impl ::protobuf::Message for ContinueRequest {
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

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ContinueRequest {
    fn new() -> ContinueRequest {
        ContinueRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ContinueRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ContinueRequest>(
                    "ContinueRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ContinueRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ContinueRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ContinueRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PingRequest {
    // message fields
    pub message: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PingRequest {}

impl PingRequest {
    pub fn new() -> PingRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PingRequest {
        static mut instance: ::protobuf::lazy::Lazy<PingRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PingRequest,
        };
        unsafe {
            instance.get(PingRequest::new)
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

impl ::protobuf::Message for PingRequest {
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
        if self.message != ::std::string::String::new() {
            my_size += ::protobuf::rt::string_size(1, &self.message);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.message != ::std::string::String::new() {
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

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PingRequest {
    fn new() -> PingRequest {
        PingRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<PingRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    PingRequest::get_message_for_reflect,
                    PingRequest::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PingRequest>(
                    "PingRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PingRequest {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PingRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DisassembleRequest {
    // message fields
    pub address: u32,
    pub count: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DisassembleRequest {}

impl DisassembleRequest {
    pub fn new() -> DisassembleRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DisassembleRequest {
        static mut instance: ::protobuf::lazy::Lazy<DisassembleRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DisassembleRequest,
        };
        unsafe {
            instance.get(DisassembleRequest::new)
        }
    }

    // uint32 address = 1;

    pub fn clear_address(&mut self) {
        self.address = 0;
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: u32) {
        self.address = v;
    }

    pub fn get_address(&self) -> u32 {
        self.address
    }

    fn get_address_for_reflect(&self) -> &u32 {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut u32 {
        &mut self.address
    }

    // uint64 count = 2;

    pub fn clear_count(&mut self) {
        self.count = 0;
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u64) {
        self.count = v;
    }

    pub fn get_count(&self) -> u64 {
        self.count
    }

    fn get_count_for_reflect(&self) -> &u64 {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut u64 {
        &mut self.count
    }
}

impl ::protobuf::Message for DisassembleRequest {
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
                    let tmp = is.read_uint32()?;
                    self.address = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.count = tmp;
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
        if self.address != 0 {
            my_size += ::protobuf::rt::value_size(1, self.address, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.count != 0 {
            my_size += ::protobuf::rt::value_size(2, self.count, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.address != 0 {
            os.write_uint32(1, self.address)?;
        };
        if self.count != 0 {
            os.write_uint64(2, self.count)?;
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

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DisassembleRequest {
    fn new() -> DisassembleRequest {
        DisassembleRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DisassembleRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "address",
                    DisassembleRequest::get_address_for_reflect,
                    DisassembleRequest::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "count",
                    DisassembleRequest::get_count_for_reflect,
                    DisassembleRequest::mut_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DisassembleRequest>(
                    "DisassembleRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DisassembleRequest {
    fn clear(&mut self) {
        self.clear_address();
        self.clear_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DisassembleRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DisassembleRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DisassembleMsg {
    // message fields
    pub line: ::std::string::String,
    pub instruction_width: u32,
    pub address: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DisassembleMsg {}

impl DisassembleMsg {
    pub fn new() -> DisassembleMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DisassembleMsg {
        static mut instance: ::protobuf::lazy::Lazy<DisassembleMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DisassembleMsg,
        };
        unsafe {
            instance.get(DisassembleMsg::new)
        }
    }

    // string line = 1;

    pub fn clear_line(&mut self) {
        self.line.clear();
    }

    // Param is passed by value, moved
    pub fn set_line(&mut self, v: ::std::string::String) {
        self.line = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_line(&mut self) -> &mut ::std::string::String {
        &mut self.line
    }

    // Take field
    pub fn take_line(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.line, ::std::string::String::new())
    }

    pub fn get_line(&self) -> &str {
        &self.line
    }

    fn get_line_for_reflect(&self) -> &::std::string::String {
        &self.line
    }

    fn mut_line_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.line
    }

    // uint32 instruction_width = 2;

    pub fn clear_instruction_width(&mut self) {
        self.instruction_width = 0;
    }

    // Param is passed by value, moved
    pub fn set_instruction_width(&mut self, v: u32) {
        self.instruction_width = v;
    }

    pub fn get_instruction_width(&self) -> u32 {
        self.instruction_width
    }

    fn get_instruction_width_for_reflect(&self) -> &u32 {
        &self.instruction_width
    }

    fn mut_instruction_width_for_reflect(&mut self) -> &mut u32 {
        &mut self.instruction_width
    }

    // uint32 address = 3;

    pub fn clear_address(&mut self) {
        self.address = 0;
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: u32) {
        self.address = v;
    }

    pub fn get_address(&self) -> u32 {
        self.address
    }

    fn get_address_for_reflect(&self) -> &u32 {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut u32 {
        &mut self.address
    }
}

impl ::protobuf::Message for DisassembleMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.line)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.instruction_width = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.address = tmp;
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
        if self.line != ::std::string::String::new() {
            my_size += ::protobuf::rt::string_size(1, &self.line);
        };
        if self.instruction_width != 0 {
            my_size += ::protobuf::rt::value_size(2, self.instruction_width, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.address != 0 {
            my_size += ::protobuf::rt::value_size(3, self.address, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.line != ::std::string::String::new() {
            os.write_string(1, &self.line)?;
        };
        if self.instruction_width != 0 {
            os.write_uint32(2, self.instruction_width)?;
        };
        if self.address != 0 {
            os.write_uint32(3, self.address)?;
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

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DisassembleMsg {
    fn new() -> DisassembleMsg {
        DisassembleMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<DisassembleMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "line",
                    DisassembleMsg::get_line_for_reflect,
                    DisassembleMsg::mut_line_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "instruction_width",
                    DisassembleMsg::get_instruction_width_for_reflect,
                    DisassembleMsg::mut_instruction_width_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "address",
                    DisassembleMsg::get_address_for_reflect,
                    DisassembleMsg::mut_address_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DisassembleMsg>(
                    "DisassembleMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DisassembleMsg {
    fn clear(&mut self) {
        self.clear_line();
        self.clear_instruction_width();
        self.clear_address();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DisassembleMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DisassembleMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DisassembleReply {
    // message fields
    pub length: u64,
    disassembly: ::protobuf::RepeatedField<DisassembleMsg>,
    pub last_error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DisassembleReply {}

impl DisassembleReply {
    pub fn new() -> DisassembleReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DisassembleReply {
        static mut instance: ::protobuf::lazy::Lazy<DisassembleReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DisassembleReply,
        };
        unsafe {
            instance.get(DisassembleReply::new)
        }
    }

    // uint64 length = 1;

    pub fn clear_length(&mut self) {
        self.length = 0;
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u64) {
        self.length = v;
    }

    pub fn get_length(&self) -> u64 {
        self.length
    }

    fn get_length_for_reflect(&self) -> &u64 {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut u64 {
        &mut self.length
    }

    // repeated .DisassembleMsg disassembly = 2;

    pub fn clear_disassembly(&mut self) {
        self.disassembly.clear();
    }

    // Param is passed by value, moved
    pub fn set_disassembly(&mut self, v: ::protobuf::RepeatedField<DisassembleMsg>) {
        self.disassembly = v;
    }

    // Mutable pointer to the field.
    pub fn mut_disassembly(&mut self) -> &mut ::protobuf::RepeatedField<DisassembleMsg> {
        &mut self.disassembly
    }

    // Take field
    pub fn take_disassembly(&mut self) -> ::protobuf::RepeatedField<DisassembleMsg> {
        ::std::mem::replace(&mut self.disassembly, ::protobuf::RepeatedField::new())
    }

    pub fn get_disassembly(&self) -> &[DisassembleMsg] {
        &self.disassembly
    }

    fn get_disassembly_for_reflect(&self) -> &::protobuf::RepeatedField<DisassembleMsg> {
        &self.disassembly
    }

    fn mut_disassembly_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DisassembleMsg> {
        &mut self.disassembly
    }

    // string last_error = 5;

    pub fn clear_last_error(&mut self) {
        self.last_error.clear();
    }

    // Param is passed by value, moved
    pub fn set_last_error(&mut self, v: ::std::string::String) {
        self.last_error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_error(&mut self) -> &mut ::std::string::String {
        &mut self.last_error
    }

    // Take field
    pub fn take_last_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.last_error, ::std::string::String::new())
    }

    pub fn get_last_error(&self) -> &str {
        &self.last_error
    }

    fn get_last_error_for_reflect(&self) -> &::std::string::String {
        &self.last_error
    }

    fn mut_last_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.last_error
    }
}

impl ::protobuf::Message for DisassembleReply {
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
                    self.length = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.disassembly)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.last_error)?;
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
        if self.length != 0 {
            my_size += ::protobuf::rt::value_size(1, self.length, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.disassembly {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.last_error != ::std::string::String::new() {
            my_size += ::protobuf::rt::string_size(5, &self.last_error);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.length != 0 {
            os.write_uint64(1, self.length)?;
        };
        for v in &self.disassembly {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.last_error != ::std::string::String::new() {
            os.write_string(5, &self.last_error)?;
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

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DisassembleReply {
    fn new() -> DisassembleReply {
        DisassembleReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<DisassembleReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "length",
                    DisassembleReply::get_length_for_reflect,
                    DisassembleReply::mut_length_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DisassembleMsg>>(
                    "disassembly",
                    DisassembleReply::get_disassembly_for_reflect,
                    DisassembleReply::mut_disassembly_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "last_error",
                    DisassembleReply::get_last_error_for_reflect,
                    DisassembleReply::mut_last_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DisassembleReply>(
                    "DisassembleReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DisassembleReply {
    fn clear(&mut self) {
        self.clear_length();
        self.clear_disassembly();
        self.clear_last_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DisassembleReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DisassembleReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BreakpointRequest {
    // message fields
    addresses: ::std::vec::Vec<u32>,
    pub action: BreakpointRequest_Action,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BreakpointRequest {}

impl BreakpointRequest {
    pub fn new() -> BreakpointRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BreakpointRequest {
        static mut instance: ::protobuf::lazy::Lazy<BreakpointRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BreakpointRequest,
        };
        unsafe {
            instance.get(BreakpointRequest::new)
        }
    }

    // repeated uint32 addresses = 1;

    pub fn clear_addresses(&mut self) {
        self.addresses.clear();
    }

    // Param is passed by value, moved
    pub fn set_addresses(&mut self, v: ::std::vec::Vec<u32>) {
        self.addresses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_addresses(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.addresses
    }

    // Take field
    pub fn take_addresses(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.addresses, ::std::vec::Vec::new())
    }

    pub fn get_addresses(&self) -> &[u32] {
        &self.addresses
    }

    fn get_addresses_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.addresses
    }

    fn mut_addresses_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.addresses
    }

    // .BreakpointRequest.Action action = 2;

    pub fn clear_action(&mut self) {
        self.action = BreakpointRequest_Action::SET;
    }

    // Param is passed by value, moved
    pub fn set_action(&mut self, v: BreakpointRequest_Action) {
        self.action = v;
    }

    pub fn get_action(&self) -> BreakpointRequest_Action {
        self.action
    }

    fn get_action_for_reflect(&self) -> &BreakpointRequest_Action {
        &self.action
    }

    fn mut_action_for_reflect(&mut self) -> &mut BreakpointRequest_Action {
        &mut self.action
    }
}

impl ::protobuf::Message for BreakpointRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.addresses)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.action = tmp;
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
        for value in &self.addresses {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.action != BreakpointRequest_Action::SET {
            my_size += ::protobuf::rt::enum_size(2, self.action);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.addresses {
            os.write_uint32(1, *v)?;
        };
        if self.action != BreakpointRequest_Action::SET {
            os.write_enum(2, self.action.value())?;
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

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BreakpointRequest {
    fn new() -> BreakpointRequest {
        BreakpointRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<BreakpointRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "addresses",
                    BreakpointRequest::get_addresses_for_reflect,
                    BreakpointRequest::mut_addresses_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<BreakpointRequest_Action>>(
                    "action",
                    BreakpointRequest::get_action_for_reflect,
                    BreakpointRequest::mut_action_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BreakpointRequest>(
                    "BreakpointRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BreakpointRequest {
    fn clear(&mut self) {
        self.clear_addresses();
        self.clear_action();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BreakpointRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BreakpointRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum BreakpointRequest_Action {
    SET = 0,
    CLEAR = 1,
}

impl ::protobuf::ProtobufEnum for BreakpointRequest_Action {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<BreakpointRequest_Action> {
        match value {
            0 => ::std::option::Option::Some(BreakpointRequest_Action::SET),
            1 => ::std::option::Option::Some(BreakpointRequest_Action::CLEAR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [BreakpointRequest_Action] = &[
            BreakpointRequest_Action::SET,
            BreakpointRequest_Action::CLEAR,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<BreakpointRequest_Action>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("BreakpointRequest_Action", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for BreakpointRequest_Action {
}

impl ::std::default::Default for BreakpointRequest_Action {
    fn default() -> Self {
        BreakpointRequest_Action::SET
    }
}

impl ::protobuf::reflect::ProtobufValue for BreakpointRequest_Action {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x26, 0x73, 0x72, 0x63, 0x2f, 0x6e, 0x65, 0x73, 0x2f, 0x64, 0x65, 0x62, 0x75, 0x67, 0x67,
    0x65, 0x72, 0x2f, 0x64, 0x65, 0x62, 0x75, 0x67, 0x67, 0x65, 0x72, 0x5f, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x23, 0x0a, 0x07, 0x4f, 0x6b, 0x52, 0x65,
    0x70, 0x6c, 0x79, 0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x36, 0x0a,
    0x0b, 0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x27, 0x0a, 0x0f,
    0x70, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6f,
    0x75, 0x6e, 0x74, 0x65, 0x72, 0x22, 0x11, 0x0a, 0x0f, 0x43, 0x6f, 0x6e, 0x74, 0x69, 0x6e, 0x75,
    0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x27, 0x0a, 0x0b, 0x50, 0x69, 0x6e, 0x67,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x22, 0x44, 0x0a, 0x12, 0x44, 0x69, 0x73, 0x61, 0x73, 0x73, 0x65, 0x6d, 0x62, 0x6c, 0x65,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73,
    0x73, 0x12, 0x14, 0x0a, 0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x22, 0x6b, 0x0a, 0x0e, 0x44, 0x69, 0x73, 0x61, 0x73,
    0x73, 0x65, 0x6d, 0x62, 0x6c, 0x65, 0x4d, 0x73, 0x67, 0x12, 0x12, 0x0a, 0x04, 0x6c, 0x69, 0x6e,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6c, 0x69, 0x6e, 0x65, 0x12, 0x2b, 0x0a,
    0x11, 0x69, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x77, 0x69, 0x64,
    0x74, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x69, 0x6e, 0x73, 0x74, 0x72, 0x75,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x57, 0x69, 0x64, 0x74, 0x68, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x61, 0x64, 0x64,
    0x72, 0x65, 0x73, 0x73, 0x22, 0x7c, 0x0a, 0x10, 0x44, 0x69, 0x73, 0x61, 0x73, 0x73, 0x65, 0x6d,
    0x62, 0x6c, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x16, 0x0a, 0x06, 0x6c, 0x65, 0x6e, 0x67,
    0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68,
    0x12, 0x31, 0x0a, 0x0b, 0x64, 0x69, 0x73, 0x61, 0x73, 0x73, 0x65, 0x6d, 0x62, 0x6c, 0x79, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x44, 0x69, 0x73, 0x61, 0x73, 0x73, 0x65, 0x6d,
    0x62, 0x6c, 0x65, 0x4d, 0x73, 0x67, 0x52, 0x0b, 0x64, 0x69, 0x73, 0x61, 0x73, 0x73, 0x65, 0x6d,
    0x62, 0x6c, 0x79, 0x12, 0x1d, 0x0a, 0x0a, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x6c, 0x61, 0x73, 0x74, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x22, 0x82, 0x01, 0x0a, 0x11, 0x42, 0x72, 0x65, 0x61, 0x6b, 0x70, 0x6f, 0x69, 0x6e,
    0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1c, 0x0a, 0x09, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x64, 0x64,
    0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x12, 0x31, 0x0a, 0x06, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x42, 0x72, 0x65, 0x61, 0x6b, 0x70, 0x6f,
    0x69, 0x6e, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x41, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x52, 0x06, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x1c, 0x0a, 0x06, 0x41, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x12, 0x07, 0x0a, 0x03, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x09, 0x0a, 0x05,
    0x43, 0x4c, 0x45, 0x41, 0x52, 0x10, 0x01, 0x32, 0xdf, 0x01, 0x0a, 0x08, 0x44, 0x65, 0x62, 0x75,
    0x67, 0x67, 0x65, 0x72, 0x12, 0x20, 0x0a, 0x04, 0x50, 0x69, 0x6e, 0x67, 0x12, 0x0c, 0x2e, 0x50,
    0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x08, 0x2e, 0x4f, 0x6b, 0x52,
    0x65, 0x70, 0x6c, 0x79, 0x22, 0x00, 0x12, 0x20, 0x0a, 0x04, 0x53, 0x74, 0x6f, 0x70, 0x12, 0x0c,
    0x2e, 0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x08, 0x2e, 0x4f,
    0x6b, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x00, 0x12, 0x28, 0x0a, 0x08, 0x43, 0x6f, 0x6e, 0x74,
    0x69, 0x6e, 0x75, 0x65, 0x12, 0x10, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x69, 0x6e, 0x75, 0x65, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x08, 0x2e, 0x4f, 0x6b, 0x52, 0x65, 0x70, 0x6c, 0x79,
    0x22, 0x00, 0x12, 0x37, 0x0a, 0x0b, 0x44, 0x69, 0x73, 0x61, 0x73, 0x73, 0x65, 0x6d, 0x62, 0x6c,
    0x65, 0x12, 0x13, 0x2e, 0x44, 0x69, 0x73, 0x61, 0x73, 0x73, 0x65, 0x6d, 0x62, 0x6c, 0x65, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x11, 0x2e, 0x44, 0x69, 0x73, 0x61, 0x73, 0x73, 0x65,
    0x6d, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x00, 0x12, 0x2c, 0x0a, 0x0a, 0x42,
    0x72, 0x65, 0x61, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x12, 0x12, 0x2e, 0x42, 0x72, 0x65, 0x61,
    0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x08, 0x2e,
    0x4f, 0x6b, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x00, 0x4a, 0xa2, 0x0c, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x35, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a,
    0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x02, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00,
    0x01, 0x12, 0x03, 0x02, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x03, 0x04, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x03, 0x0e, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x03, 0x24, 0x2b, 0x0a, 0x0b, 0x0a,
    0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x03, 0x05, 0x04, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x05, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x05, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x05, 0x24, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x02, 0x12, 0x03, 0x06, 0x04,
    0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x06, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x06, 0x12, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x06, 0x2c, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x06,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x08, 0x04, 0x46, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x08, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x02, 0x12,
    0x03, 0x08, 0x15, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x08,
    0x32, 0x42, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x04, 0x12, 0x03, 0x09, 0x04, 0x3b, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x09, 0x08, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x09, 0x14, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x09, 0x30, 0x37, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x0c, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x08,
    0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x04, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0d, 0x04, 0x0c, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x0d, 0x15, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x10,
    0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x10, 0x08, 0x13, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x11, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x11, 0x04, 0x10, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x11, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x11, 0x0b, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x11, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x14, 0x00, 0x15,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x14, 0x08, 0x17, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x17, 0x00, 0x19, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x17, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x18,
    0x04, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x18, 0x04, 0x17,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x18, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x0b, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x15, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x04, 0x12, 0x04, 0x1c, 0x00, 0x1f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03,
    0x1c, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x04, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x1d, 0x04, 0x1c, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1d, 0x04, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1d, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x1e, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x1e, 0x04, 0x1d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x1e, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1e, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1e, 0x13, 0x14, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x21, 0x00, 0x26, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x05, 0x01, 0x12, 0x03, 0x21, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12,
    0x03, 0x22, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x22,
    0x04, 0x21, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x22, 0x04,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x0b, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x22, 0x12, 0x13, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x23, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x23, 0x04, 0x22, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x23, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x23, 0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x23, 0x1f, 0x20, 0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x24, 0x04, 0x17,
    0x22, 0x1d, 0x20, 0x43, 0x6f, 0x6e, 0x73, 0x69, 0x64, 0x65, 0x72, 0x20, 0x61, 0x64, 0x64, 0x69,
    0x6e, 0x67, 0x20, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x4d, 0x6f, 0x64, 0x65, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x04, 0x24, 0x04, 0x23, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x24, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x24, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x24, 0x15, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04,
    0x28, 0x00, 0x2c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x28, 0x08, 0x18,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x29, 0x04, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04, 0x29, 0x04, 0x28, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x29, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x29, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03,
    0x2a, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2a, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x2a, 0x0d, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x1c, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x02, 0x12, 0x03, 0x2b, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02,
    0x04, 0x12, 0x04, 0x2b, 0x04, 0x2a, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x2b, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x2b, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2b, 0x18,
    0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x2e, 0x00, 0x35, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x00, 0x12, 0x03, 0x2f, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x2f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2f,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x14, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2f, 0x20, 0x21, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x07, 0x04, 0x00, 0x12, 0x04, 0x30, 0x04, 0x33, 0x05, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x04, 0x00, 0x01, 0x12, 0x03, 0x30, 0x09, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x31, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x08, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x31, 0x0e, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x32, 0x08, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x32, 0x08, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x03, 0x32, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01,
    0x12, 0x03, 0x34, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x34, 0x04, 0x33, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x34,
    0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x34, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x34, 0x14, 0x15, 0x62, 0x06,
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
