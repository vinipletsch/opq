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
pub struct Measurement {
    // message fields
    pub box_id: u32,
    pub timestamp_ms: u64,
    pub metrics: ::std::collections::HashMap<::std::string::String, f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Measurement {}

impl Measurement {
    pub fn new() -> Measurement {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Measurement {
        static mut instance: ::protobuf::lazy::Lazy<Measurement> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Measurement,
        };
        unsafe {
            instance.get(Measurement::new)
        }
    }

    // uint32 box_id = 1;

    pub fn clear_box_id(&mut self) {
        self.box_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_box_id(&mut self, v: u32) {
        self.box_id = v;
    }

    pub fn get_box_id(&self) -> u32 {
        self.box_id
    }

    fn get_box_id_for_reflect(&self) -> &u32 {
        &self.box_id
    }

    fn mut_box_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.box_id
    }

    // uint64 timestamp_ms = 2;

    pub fn clear_timestamp_ms(&mut self) {
        self.timestamp_ms = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp_ms(&mut self, v: u64) {
        self.timestamp_ms = v;
    }

    pub fn get_timestamp_ms(&self) -> u64 {
        self.timestamp_ms
    }

    fn get_timestamp_ms_for_reflect(&self) -> &u64 {
        &self.timestamp_ms
    }

    fn mut_timestamp_ms_for_reflect(&mut self) -> &mut u64 {
        &mut self.timestamp_ms
    }

    // repeated .opq.proto3.Measurement.MetricsEntry metrics = 3;

    pub fn clear_metrics(&mut self) {
        self.metrics.clear();
    }

    // Param is passed by value, moved
    pub fn set_metrics(&mut self, v: ::std::collections::HashMap<::std::string::String, f32>) {
        self.metrics = v;
    }

    // Mutable pointer to the field.
    pub fn mut_metrics(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, f32> {
        &mut self.metrics
    }

    // Take field
    pub fn take_metrics(&mut self) -> ::std::collections::HashMap<::std::string::String, f32> {
        ::std::mem::replace(&mut self.metrics, ::std::collections::HashMap::new())
    }

    pub fn get_metrics(&self) -> &::std::collections::HashMap<::std::string::String, f32> {
        &self.metrics
    }

    fn get_metrics_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, f32> {
        &self.metrics
    }

    fn mut_metrics_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, f32> {
        &mut self.metrics
    }
}

impl ::protobuf::Message for Measurement {
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
                    self.box_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp_ms = tmp;
                },
                3 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeFloat>(wire_type, is, &mut self.metrics)?;
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
        if self.box_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.box_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.timestamp_ms != 0 {
            my_size += ::protobuf::rt::value_size(2, self.timestamp_ms, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeFloat>(3, &self.metrics);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.box_id != 0 {
            os.write_uint32(1, self.box_id)?;
        }
        if self.timestamp_ms != 0 {
            os.write_uint64(2, self.timestamp_ms)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeFloat>(3, &self.metrics, os)?;
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

impl ::protobuf::MessageStatic for Measurement {
    fn new() -> Measurement {
        Measurement::new()
    }

    fn descriptor_static(_: ::std::option::Option<Measurement>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "box_id",
                    Measurement::get_box_id_for_reflect,
                    Measurement::mut_box_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp_ms",
                    Measurement::get_timestamp_ms_for_reflect,
                    Measurement::mut_timestamp_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeFloat>(
                    "metrics",
                    Measurement::get_metrics_for_reflect,
                    Measurement::mut_metrics_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Measurement>(
                    "Measurement",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Measurement {
    fn clear(&mut self) {
        self.clear_box_id();
        self.clear_timestamp_ms();
        self.clear_metrics();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Measurement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Measurement {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetInfoCommand {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetInfoCommand {}

impl GetInfoCommand {
    pub fn new() -> GetInfoCommand {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetInfoCommand {
        static mut instance: ::protobuf::lazy::Lazy<GetInfoCommand> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetInfoCommand,
        };
        unsafe {
            instance.get(GetInfoCommand::new)
        }
    }
}

impl ::protobuf::Message for GetInfoCommand {
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

impl ::protobuf::MessageStatic for GetInfoCommand {
    fn new() -> GetInfoCommand {
        GetInfoCommand::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetInfoCommand>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetInfoCommand>(
                    "GetInfoCommand",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetInfoCommand {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetInfoCommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetInfoCommand {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetDataCommand {
    // message fields
    pub start_ms: u64,
    pub end_ms: u64,
    pub wait: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetDataCommand {}

impl GetDataCommand {
    pub fn new() -> GetDataCommand {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetDataCommand {
        static mut instance: ::protobuf::lazy::Lazy<GetDataCommand> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetDataCommand,
        };
        unsafe {
            instance.get(GetDataCommand::new)
        }
    }

    // uint64 start_ms = 1;

    pub fn clear_start_ms(&mut self) {
        self.start_ms = 0;
    }

    // Param is passed by value, moved
    pub fn set_start_ms(&mut self, v: u64) {
        self.start_ms = v;
    }

    pub fn get_start_ms(&self) -> u64 {
        self.start_ms
    }

    fn get_start_ms_for_reflect(&self) -> &u64 {
        &self.start_ms
    }

    fn mut_start_ms_for_reflect(&mut self) -> &mut u64 {
        &mut self.start_ms
    }

    // uint64 end_ms = 2;

    pub fn clear_end_ms(&mut self) {
        self.end_ms = 0;
    }

    // Param is passed by value, moved
    pub fn set_end_ms(&mut self, v: u64) {
        self.end_ms = v;
    }

    pub fn get_end_ms(&self) -> u64 {
        self.end_ms
    }

    fn get_end_ms_for_reflect(&self) -> &u64 {
        &self.end_ms
    }

    fn mut_end_ms_for_reflect(&mut self) -> &mut u64 {
        &mut self.end_ms
    }

    // bool wait = 3;

    pub fn clear_wait(&mut self) {
        self.wait = false;
    }

    // Param is passed by value, moved
    pub fn set_wait(&mut self, v: bool) {
        self.wait = v;
    }

    pub fn get_wait(&self) -> bool {
        self.wait
    }

    fn get_wait_for_reflect(&self) -> &bool {
        &self.wait
    }

    fn mut_wait_for_reflect(&mut self) -> &mut bool {
        &mut self.wait
    }
}

impl ::protobuf::Message for GetDataCommand {
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
                    let tmp = is.read_uint64()?;
                    self.start_ms = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.end_ms = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.wait = tmp;
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
        if self.start_ms != 0 {
            my_size += ::protobuf::rt::value_size(1, self.start_ms, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.end_ms != 0 {
            my_size += ::protobuf::rt::value_size(2, self.end_ms, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.wait != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.start_ms != 0 {
            os.write_uint64(1, self.start_ms)?;
        }
        if self.end_ms != 0 {
            os.write_uint64(2, self.end_ms)?;
        }
        if self.wait != false {
            os.write_bool(3, self.wait)?;
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

impl ::protobuf::MessageStatic for GetDataCommand {
    fn new() -> GetDataCommand {
        GetDataCommand::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetDataCommand>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_ms",
                    GetDataCommand::get_start_ms_for_reflect,
                    GetDataCommand::mut_start_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "end_ms",
                    GetDataCommand::get_end_ms_for_reflect,
                    GetDataCommand::mut_end_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "wait",
                    GetDataCommand::get_wait_for_reflect,
                    GetDataCommand::mut_wait_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetDataCommand>(
                    "GetDataCommand",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetDataCommand {
    fn clear(&mut self) {
        self.clear_start_ms();
        self.clear_end_ms();
        self.clear_wait();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetDataCommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetDataCommand {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetMeasturementRateCommand {
    // message fields
    pub measurement_window_cycles: u32,
    pub duration_ms: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetMeasturementRateCommand {}

impl SetMeasturementRateCommand {
    pub fn new() -> SetMeasturementRateCommand {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetMeasturementRateCommand {
        static mut instance: ::protobuf::lazy::Lazy<SetMeasturementRateCommand> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetMeasturementRateCommand,
        };
        unsafe {
            instance.get(SetMeasturementRateCommand::new)
        }
    }

    // uint32 measurement_window_cycles = 1;

    pub fn clear_measurement_window_cycles(&mut self) {
        self.measurement_window_cycles = 0;
    }

    // Param is passed by value, moved
    pub fn set_measurement_window_cycles(&mut self, v: u32) {
        self.measurement_window_cycles = v;
    }

    pub fn get_measurement_window_cycles(&self) -> u32 {
        self.measurement_window_cycles
    }

    fn get_measurement_window_cycles_for_reflect(&self) -> &u32 {
        &self.measurement_window_cycles
    }

    fn mut_measurement_window_cycles_for_reflect(&mut self) -> &mut u32 {
        &mut self.measurement_window_cycles
    }

    // uint32 duration_ms = 2;

    pub fn clear_duration_ms(&mut self) {
        self.duration_ms = 0;
    }

    // Param is passed by value, moved
    pub fn set_duration_ms(&mut self, v: u32) {
        self.duration_ms = v;
    }

    pub fn get_duration_ms(&self) -> u32 {
        self.duration_ms
    }

    fn get_duration_ms_for_reflect(&self) -> &u32 {
        &self.duration_ms
    }

    fn mut_duration_ms_for_reflect(&mut self) -> &mut u32 {
        &mut self.duration_ms
    }
}

impl ::protobuf::Message for SetMeasturementRateCommand {
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
                    self.measurement_window_cycles = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.duration_ms = tmp;
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
        if self.measurement_window_cycles != 0 {
            my_size += ::protobuf::rt::value_size(1, self.measurement_window_cycles, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.duration_ms != 0 {
            my_size += ::protobuf::rt::value_size(2, self.duration_ms, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.measurement_window_cycles != 0 {
            os.write_uint32(1, self.measurement_window_cycles)?;
        }
        if self.duration_ms != 0 {
            os.write_uint32(2, self.duration_ms)?;
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

impl ::protobuf::MessageStatic for SetMeasturementRateCommand {
    fn new() -> SetMeasturementRateCommand {
        SetMeasturementRateCommand::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetMeasturementRateCommand>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "measurement_window_cycles",
                    SetMeasturementRateCommand::get_measurement_window_cycles_for_reflect,
                    SetMeasturementRateCommand::mut_measurement_window_cycles_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "duration_ms",
                    SetMeasturementRateCommand::get_duration_ms_for_reflect,
                    SetMeasturementRateCommand::mut_duration_ms_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetMeasturementRateCommand>(
                    "SetMeasturementRateCommand",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetMeasturementRateCommand {
    fn clear(&mut self) {
        self.clear_measurement_window_cycles();
        self.clear_duration_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetMeasturementRateCommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetMeasturementRateCommand {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Command {
    // message fields
    pub seq: u32,
    pub timestamp_ms: u64,
    // message oneof groups
    pub command: ::std::option::Option<Command_oneof_command>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Command {}

#[derive(Clone,PartialEq)]
pub enum Command_oneof_command {
    info_command(GetInfoCommand),
    data_command(GetDataCommand),
    sampling_rate_command(SetMeasturementRateCommand),
}

impl Command {
    pub fn new() -> Command {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Command {
        static mut instance: ::protobuf::lazy::Lazy<Command> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Command,
        };
        unsafe {
            instance.get(Command::new)
        }
    }

    // uint32 seq = 1;

    pub fn clear_seq(&mut self) {
        self.seq = 0;
    }

    // Param is passed by value, moved
    pub fn set_seq(&mut self, v: u32) {
        self.seq = v;
    }

    pub fn get_seq(&self) -> u32 {
        self.seq
    }

    fn get_seq_for_reflect(&self) -> &u32 {
        &self.seq
    }

    fn mut_seq_for_reflect(&mut self) -> &mut u32 {
        &mut self.seq
    }

    // uint64 timestamp_ms = 2;

    pub fn clear_timestamp_ms(&mut self) {
        self.timestamp_ms = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp_ms(&mut self, v: u64) {
        self.timestamp_ms = v;
    }

    pub fn get_timestamp_ms(&self) -> u64 {
        self.timestamp_ms
    }

    fn get_timestamp_ms_for_reflect(&self) -> &u64 {
        &self.timestamp_ms
    }

    fn mut_timestamp_ms_for_reflect(&mut self) -> &mut u64 {
        &mut self.timestamp_ms
    }

    // .opq.proto3.GetInfoCommand info_command = 3;

    pub fn clear_info_command(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_info_command(&self) -> bool {
        match self.command {
            ::std::option::Option::Some(Command_oneof_command::info_command(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_info_command(&mut self, v: GetInfoCommand) {
        self.command = ::std::option::Option::Some(Command_oneof_command::info_command(v))
    }

    // Mutable pointer to the field.
    pub fn mut_info_command(&mut self) -> &mut GetInfoCommand {
        if let ::std::option::Option::Some(Command_oneof_command::info_command(_)) = self.command {
        } else {
            self.command = ::std::option::Option::Some(Command_oneof_command::info_command(GetInfoCommand::new()));
        }
        match self.command {
            ::std::option::Option::Some(Command_oneof_command::info_command(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_info_command(&mut self) -> GetInfoCommand {
        if self.has_info_command() {
            match self.command.take() {
                ::std::option::Option::Some(Command_oneof_command::info_command(v)) => v,
                _ => panic!(),
            }
        } else {
            GetInfoCommand::new()
        }
    }

    pub fn get_info_command(&self) -> &GetInfoCommand {
        match self.command {
            ::std::option::Option::Some(Command_oneof_command::info_command(ref v)) => v,
            _ => GetInfoCommand::default_instance(),
        }
    }

    // .opq.proto3.GetDataCommand data_command = 4;

    pub fn clear_data_command(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_data_command(&self) -> bool {
        match self.command {
            ::std::option::Option::Some(Command_oneof_command::data_command(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_data_command(&mut self, v: GetDataCommand) {
        self.command = ::std::option::Option::Some(Command_oneof_command::data_command(v))
    }

    // Mutable pointer to the field.
    pub fn mut_data_command(&mut self) -> &mut GetDataCommand {
        if let ::std::option::Option::Some(Command_oneof_command::data_command(_)) = self.command {
        } else {
            self.command = ::std::option::Option::Some(Command_oneof_command::data_command(GetDataCommand::new()));
        }
        match self.command {
            ::std::option::Option::Some(Command_oneof_command::data_command(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_data_command(&mut self) -> GetDataCommand {
        if self.has_data_command() {
            match self.command.take() {
                ::std::option::Option::Some(Command_oneof_command::data_command(v)) => v,
                _ => panic!(),
            }
        } else {
            GetDataCommand::new()
        }
    }

    pub fn get_data_command(&self) -> &GetDataCommand {
        match self.command {
            ::std::option::Option::Some(Command_oneof_command::data_command(ref v)) => v,
            _ => GetDataCommand::default_instance(),
        }
    }

    // .opq.proto3.SetMeasturementRateCommand sampling_rate_command = 5;

    pub fn clear_sampling_rate_command(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_sampling_rate_command(&self) -> bool {
        match self.command {
            ::std::option::Option::Some(Command_oneof_command::sampling_rate_command(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_sampling_rate_command(&mut self, v: SetMeasturementRateCommand) {
        self.command = ::std::option::Option::Some(Command_oneof_command::sampling_rate_command(v))
    }

    // Mutable pointer to the field.
    pub fn mut_sampling_rate_command(&mut self) -> &mut SetMeasturementRateCommand {
        if let ::std::option::Option::Some(Command_oneof_command::sampling_rate_command(_)) = self.command {
        } else {
            self.command = ::std::option::Option::Some(Command_oneof_command::sampling_rate_command(SetMeasturementRateCommand::new()));
        }
        match self.command {
            ::std::option::Option::Some(Command_oneof_command::sampling_rate_command(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_sampling_rate_command(&mut self) -> SetMeasturementRateCommand {
        if self.has_sampling_rate_command() {
            match self.command.take() {
                ::std::option::Option::Some(Command_oneof_command::sampling_rate_command(v)) => v,
                _ => panic!(),
            }
        } else {
            SetMeasturementRateCommand::new()
        }
    }

    pub fn get_sampling_rate_command(&self) -> &SetMeasturementRateCommand {
        match self.command {
            ::std::option::Option::Some(Command_oneof_command::sampling_rate_command(ref v)) => v,
            _ => SetMeasturementRateCommand::default_instance(),
        }
    }
}

impl ::protobuf::Message for Command {
    fn is_initialized(&self) -> bool {
        if let Some(Command_oneof_command::info_command(ref v)) = self.command {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Command_oneof_command::data_command(ref v)) = self.command {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Command_oneof_command::sampling_rate_command(ref v)) = self.command {
            if !v.is_initialized() {
                return false;
            }
        }
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
                    self.seq = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp_ms = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.command = ::std::option::Option::Some(Command_oneof_command::info_command(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.command = ::std::option::Option::Some(Command_oneof_command::data_command(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.command = ::std::option::Option::Some(Command_oneof_command::sampling_rate_command(is.read_message()?));
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
        if self.seq != 0 {
            my_size += ::protobuf::rt::value_size(1, self.seq, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.timestamp_ms != 0 {
            my_size += ::protobuf::rt::value_size(2, self.timestamp_ms, ::protobuf::wire_format::WireTypeVarint);
        }
        if let ::std::option::Option::Some(ref v) = self.command {
            match v {
                &Command_oneof_command::info_command(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Command_oneof_command::data_command(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Command_oneof_command::sampling_rate_command(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.seq != 0 {
            os.write_uint32(1, self.seq)?;
        }
        if self.timestamp_ms != 0 {
            os.write_uint64(2, self.timestamp_ms)?;
        }
        if let ::std::option::Option::Some(ref v) = self.command {
            match v {
                &Command_oneof_command::info_command(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Command_oneof_command::data_command(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Command_oneof_command::sampling_rate_command(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for Command {
    fn new() -> Command {
        Command::new()
    }

    fn descriptor_static(_: ::std::option::Option<Command>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq",
                    Command::get_seq_for_reflect,
                    Command::mut_seq_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp_ms",
                    Command::get_timestamp_ms_for_reflect,
                    Command::mut_timestamp_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetInfoCommand>(
                    "info_command",
                    Command::has_info_command,
                    Command::get_info_command,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetDataCommand>(
                    "data_command",
                    Command::has_data_command,
                    Command::get_data_command,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetMeasturementRateCommand>(
                    "sampling_rate_command",
                    Command::has_sampling_rate_command,
                    Command::get_sampling_rate_command,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Command>(
                    "Command",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Command {
    fn clear(&mut self) {
        self.clear_seq();
        self.clear_timestamp_ms();
        self.clear_info_command();
        self.clear_data_command();
        self.clear_sampling_rate_command();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Command {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Command {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetInfoResponse {
    // message fields
    pub mac_addr: ::std::string::String,
    pub wifi_network: ::std::string::String,
    pub ip: ::std::string::String,
    pub uptime: u64,
    pub calibration_constant: u64,
    pub pub_key: ::std::string::String,
    pub measurement_rate: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetInfoResponse {}

impl GetInfoResponse {
    pub fn new() -> GetInfoResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetInfoResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetInfoResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetInfoResponse,
        };
        unsafe {
            instance.get(GetInfoResponse::new)
        }
    }

    // string mac_addr = 1;

    pub fn clear_mac_addr(&mut self) {
        self.mac_addr.clear();
    }

    // Param is passed by value, moved
    pub fn set_mac_addr(&mut self, v: ::std::string::String) {
        self.mac_addr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mac_addr(&mut self) -> &mut ::std::string::String {
        &mut self.mac_addr
    }

    // Take field
    pub fn take_mac_addr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.mac_addr, ::std::string::String::new())
    }

    pub fn get_mac_addr(&self) -> &str {
        &self.mac_addr
    }

    fn get_mac_addr_for_reflect(&self) -> &::std::string::String {
        &self.mac_addr
    }

    fn mut_mac_addr_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.mac_addr
    }

    // string wifi_network = 2;

    pub fn clear_wifi_network(&mut self) {
        self.wifi_network.clear();
    }

    // Param is passed by value, moved
    pub fn set_wifi_network(&mut self, v: ::std::string::String) {
        self.wifi_network = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_wifi_network(&mut self) -> &mut ::std::string::String {
        &mut self.wifi_network
    }

    // Take field
    pub fn take_wifi_network(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.wifi_network, ::std::string::String::new())
    }

    pub fn get_wifi_network(&self) -> &str {
        &self.wifi_network
    }

    fn get_wifi_network_for_reflect(&self) -> &::std::string::String {
        &self.wifi_network
    }

    fn mut_wifi_network_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.wifi_network
    }

    // string ip = 3;

    pub fn clear_ip(&mut self) {
        self.ip.clear();
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: ::std::string::String) {
        self.ip = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip(&mut self) -> &mut ::std::string::String {
        &mut self.ip
    }

    // Take field
    pub fn take_ip(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ip, ::std::string::String::new())
    }

    pub fn get_ip(&self) -> &str {
        &self.ip
    }

    fn get_ip_for_reflect(&self) -> &::std::string::String {
        &self.ip
    }

    fn mut_ip_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.ip
    }

    // uint64 uptime = 4;

    pub fn clear_uptime(&mut self) {
        self.uptime = 0;
    }

    // Param is passed by value, moved
    pub fn set_uptime(&mut self, v: u64) {
        self.uptime = v;
    }

    pub fn get_uptime(&self) -> u64 {
        self.uptime
    }

    fn get_uptime_for_reflect(&self) -> &u64 {
        &self.uptime
    }

    fn mut_uptime_for_reflect(&mut self) -> &mut u64 {
        &mut self.uptime
    }

    // uint64 calibration_constant = 5;

    pub fn clear_calibration_constant(&mut self) {
        self.calibration_constant = 0;
    }

    // Param is passed by value, moved
    pub fn set_calibration_constant(&mut self, v: u64) {
        self.calibration_constant = v;
    }

    pub fn get_calibration_constant(&self) -> u64 {
        self.calibration_constant
    }

    fn get_calibration_constant_for_reflect(&self) -> &u64 {
        &self.calibration_constant
    }

    fn mut_calibration_constant_for_reflect(&mut self) -> &mut u64 {
        &mut self.calibration_constant
    }

    // string pub_key = 6;

    pub fn clear_pub_key(&mut self) {
        self.pub_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_pub_key(&mut self, v: ::std::string::String) {
        self.pub_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pub_key(&mut self) -> &mut ::std::string::String {
        &mut self.pub_key
    }

    // Take field
    pub fn take_pub_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.pub_key, ::std::string::String::new())
    }

    pub fn get_pub_key(&self) -> &str {
        &self.pub_key
    }

    fn get_pub_key_for_reflect(&self) -> &::std::string::String {
        &self.pub_key
    }

    fn mut_pub_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.pub_key
    }

    // uint32 measurement_rate = 7;

    pub fn clear_measurement_rate(&mut self) {
        self.measurement_rate = 0;
    }

    // Param is passed by value, moved
    pub fn set_measurement_rate(&mut self, v: u32) {
        self.measurement_rate = v;
    }

    pub fn get_measurement_rate(&self) -> u32 {
        self.measurement_rate
    }

    fn get_measurement_rate_for_reflect(&self) -> &u32 {
        &self.measurement_rate
    }

    fn mut_measurement_rate_for_reflect(&mut self) -> &mut u32 {
        &mut self.measurement_rate
    }
}

impl ::protobuf::Message for GetInfoResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.mac_addr)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.wifi_network)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ip)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.uptime = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.calibration_constant = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.pub_key)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.measurement_rate = tmp;
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
        if !self.mac_addr.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.mac_addr);
        }
        if !self.wifi_network.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.wifi_network);
        }
        if !self.ip.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.ip);
        }
        if self.uptime != 0 {
            my_size += ::protobuf::rt::value_size(4, self.uptime, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.calibration_constant != 0 {
            my_size += ::protobuf::rt::value_size(5, self.calibration_constant, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.pub_key.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.pub_key);
        }
        if self.measurement_rate != 0 {
            my_size += ::protobuf::rt::value_size(7, self.measurement_rate, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.mac_addr.is_empty() {
            os.write_string(1, &self.mac_addr)?;
        }
        if !self.wifi_network.is_empty() {
            os.write_string(2, &self.wifi_network)?;
        }
        if !self.ip.is_empty() {
            os.write_string(3, &self.ip)?;
        }
        if self.uptime != 0 {
            os.write_uint64(4, self.uptime)?;
        }
        if self.calibration_constant != 0 {
            os.write_uint64(5, self.calibration_constant)?;
        }
        if !self.pub_key.is_empty() {
            os.write_string(6, &self.pub_key)?;
        }
        if self.measurement_rate != 0 {
            os.write_uint32(7, self.measurement_rate)?;
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

impl ::protobuf::MessageStatic for GetInfoResponse {
    fn new() -> GetInfoResponse {
        GetInfoResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetInfoResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "mac_addr",
                    GetInfoResponse::get_mac_addr_for_reflect,
                    GetInfoResponse::mut_mac_addr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "wifi_network",
                    GetInfoResponse::get_wifi_network_for_reflect,
                    GetInfoResponse::mut_wifi_network_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ip",
                    GetInfoResponse::get_ip_for_reflect,
                    GetInfoResponse::mut_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "uptime",
                    GetInfoResponse::get_uptime_for_reflect,
                    GetInfoResponse::mut_uptime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "calibration_constant",
                    GetInfoResponse::get_calibration_constant_for_reflect,
                    GetInfoResponse::mut_calibration_constant_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pub_key",
                    GetInfoResponse::get_pub_key_for_reflect,
                    GetInfoResponse::mut_pub_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "measurement_rate",
                    GetInfoResponse::get_measurement_rate_for_reflect,
                    GetInfoResponse::mut_measurement_rate_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetInfoResponse>(
                    "GetInfoResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetInfoResponse {
    fn clear(&mut self) {
        self.clear_mac_addr();
        self.clear_wifi_network();
        self.clear_ip();
        self.clear_uptime();
        self.clear_calibration_constant();
        self.clear_pub_key();
        self.clear_measurement_rate();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetInfoResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetInfoResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetMeasurementRateResponse {
    // message fields
    pub old_rate_cycles: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetMeasurementRateResponse {}

impl SetMeasurementRateResponse {
    pub fn new() -> SetMeasurementRateResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetMeasurementRateResponse {
        static mut instance: ::protobuf::lazy::Lazy<SetMeasurementRateResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetMeasurementRateResponse,
        };
        unsafe {
            instance.get(SetMeasurementRateResponse::new)
        }
    }

    // uint32 old_rate_cycles = 1;

    pub fn clear_old_rate_cycles(&mut self) {
        self.old_rate_cycles = 0;
    }

    // Param is passed by value, moved
    pub fn set_old_rate_cycles(&mut self, v: u32) {
        self.old_rate_cycles = v;
    }

    pub fn get_old_rate_cycles(&self) -> u32 {
        self.old_rate_cycles
    }

    fn get_old_rate_cycles_for_reflect(&self) -> &u32 {
        &self.old_rate_cycles
    }

    fn mut_old_rate_cycles_for_reflect(&mut self) -> &mut u32 {
        &mut self.old_rate_cycles
    }
}

impl ::protobuf::Message for SetMeasurementRateResponse {
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
                    self.old_rate_cycles = tmp;
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
        if self.old_rate_cycles != 0 {
            my_size += ::protobuf::rt::value_size(1, self.old_rate_cycles, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.old_rate_cycles != 0 {
            os.write_uint32(1, self.old_rate_cycles)?;
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

impl ::protobuf::MessageStatic for SetMeasurementRateResponse {
    fn new() -> SetMeasurementRateResponse {
        SetMeasurementRateResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetMeasurementRateResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "old_rate_cycles",
                    SetMeasurementRateResponse::get_old_rate_cycles_for_reflect,
                    SetMeasurementRateResponse::mut_old_rate_cycles_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetMeasurementRateResponse>(
                    "SetMeasurementRateResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetMeasurementRateResponse {
    fn clear(&mut self) {
        self.clear_old_rate_cycles();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetMeasurementRateResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetMeasurementRateResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetDateResponseHeader {
    // message fields
    pub start_ts: u64,
    pub end_ts: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetDateResponseHeader {}

impl GetDateResponseHeader {
    pub fn new() -> GetDateResponseHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetDateResponseHeader {
        static mut instance: ::protobuf::lazy::Lazy<GetDateResponseHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetDateResponseHeader,
        };
        unsafe {
            instance.get(GetDateResponseHeader::new)
        }
    }

    // uint64 start_ts = 1;

    pub fn clear_start_ts(&mut self) {
        self.start_ts = 0;
    }

    // Param is passed by value, moved
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = v;
    }

    pub fn get_start_ts(&self) -> u64 {
        self.start_ts
    }

    fn get_start_ts_for_reflect(&self) -> &u64 {
        &self.start_ts
    }

    fn mut_start_ts_for_reflect(&mut self) -> &mut u64 {
        &mut self.start_ts
    }

    // uint64 end_ts = 2;

    pub fn clear_end_ts(&mut self) {
        self.end_ts = 0;
    }

    // Param is passed by value, moved
    pub fn set_end_ts(&mut self, v: u64) {
        self.end_ts = v;
    }

    pub fn get_end_ts(&self) -> u64 {
        self.end_ts
    }

    fn get_end_ts_for_reflect(&self) -> &u64 {
        &self.end_ts
    }

    fn mut_end_ts_for_reflect(&mut self) -> &mut u64 {
        &mut self.end_ts
    }
}

impl ::protobuf::Message for GetDateResponseHeader {
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
                    let tmp = is.read_uint64()?;
                    self.start_ts = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.end_ts = tmp;
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
        if self.start_ts != 0 {
            my_size += ::protobuf::rt::value_size(1, self.start_ts, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.end_ts != 0 {
            my_size += ::protobuf::rt::value_size(2, self.end_ts, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.start_ts != 0 {
            os.write_uint64(1, self.start_ts)?;
        }
        if self.end_ts != 0 {
            os.write_uint64(2, self.end_ts)?;
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

impl ::protobuf::MessageStatic for GetDateResponseHeader {
    fn new() -> GetDateResponseHeader {
        GetDateResponseHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetDateResponseHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_ts",
                    GetDateResponseHeader::get_start_ts_for_reflect,
                    GetDateResponseHeader::mut_start_ts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "end_ts",
                    GetDateResponseHeader::get_end_ts_for_reflect,
                    GetDateResponseHeader::mut_end_ts_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetDateResponseHeader>(
                    "GetDateResponseHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetDateResponseHeader {
    fn clear(&mut self) {
        self.clear_start_ts();
        self.clear_end_ts();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetDateResponseHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetDateResponseHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message fields
    pub box_id: i32,
    pub seq: u32,
    pub timestamp_ms: u64,
    // message oneof groups
    pub response: ::std::option::Option<Response_oneof_response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

#[derive(Clone,PartialEq)]
pub enum Response_oneof_response {
    info_response(GetInfoResponse),
    message_rate_reponse(SetMeasurementRateResponse),
    get_data_response(GetDateResponseHeader),
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

    // int32 box_id = 1;

    pub fn clear_box_id(&mut self) {
        self.box_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_box_id(&mut self, v: i32) {
        self.box_id = v;
    }

    pub fn get_box_id(&self) -> i32 {
        self.box_id
    }

    fn get_box_id_for_reflect(&self) -> &i32 {
        &self.box_id
    }

    fn mut_box_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.box_id
    }

    // uint32 seq = 2;

    pub fn clear_seq(&mut self) {
        self.seq = 0;
    }

    // Param is passed by value, moved
    pub fn set_seq(&mut self, v: u32) {
        self.seq = v;
    }

    pub fn get_seq(&self) -> u32 {
        self.seq
    }

    fn get_seq_for_reflect(&self) -> &u32 {
        &self.seq
    }

    fn mut_seq_for_reflect(&mut self) -> &mut u32 {
        &mut self.seq
    }

    // uint64 timestamp_ms = 3;

    pub fn clear_timestamp_ms(&mut self) {
        self.timestamp_ms = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp_ms(&mut self, v: u64) {
        self.timestamp_ms = v;
    }

    pub fn get_timestamp_ms(&self) -> u64 {
        self.timestamp_ms
    }

    fn get_timestamp_ms_for_reflect(&self) -> &u64 {
        &self.timestamp_ms
    }

    fn mut_timestamp_ms_for_reflect(&mut self) -> &mut u64 {
        &mut self.timestamp_ms
    }

    // .opq.proto3.GetInfoResponse info_response = 4;

    pub fn clear_info_response(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_info_response(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::info_response(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_info_response(&mut self, v: GetInfoResponse) {
        self.response = ::std::option::Option::Some(Response_oneof_response::info_response(v))
    }

    // Mutable pointer to the field.
    pub fn mut_info_response(&mut self) -> &mut GetInfoResponse {
        if let ::std::option::Option::Some(Response_oneof_response::info_response(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::info_response(GetInfoResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::info_response(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_info_response(&mut self) -> GetInfoResponse {
        if self.has_info_response() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::info_response(v)) => v,
                _ => panic!(),
            }
        } else {
            GetInfoResponse::new()
        }
    }

    pub fn get_info_response(&self) -> &GetInfoResponse {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::info_response(ref v)) => v,
            _ => GetInfoResponse::default_instance(),
        }
    }

    // .opq.proto3.SetMeasurementRateResponse message_rate_reponse = 5;

    pub fn clear_message_rate_reponse(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_message_rate_reponse(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::message_rate_reponse(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_message_rate_reponse(&mut self, v: SetMeasurementRateResponse) {
        self.response = ::std::option::Option::Some(Response_oneof_response::message_rate_reponse(v))
    }

    // Mutable pointer to the field.
    pub fn mut_message_rate_reponse(&mut self) -> &mut SetMeasurementRateResponse {
        if let ::std::option::Option::Some(Response_oneof_response::message_rate_reponse(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::message_rate_reponse(SetMeasurementRateResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::message_rate_reponse(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_message_rate_reponse(&mut self) -> SetMeasurementRateResponse {
        if self.has_message_rate_reponse() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::message_rate_reponse(v)) => v,
                _ => panic!(),
            }
        } else {
            SetMeasurementRateResponse::new()
        }
    }

    pub fn get_message_rate_reponse(&self) -> &SetMeasurementRateResponse {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::message_rate_reponse(ref v)) => v,
            _ => SetMeasurementRateResponse::default_instance(),
        }
    }

    // .opq.proto3.GetDateResponseHeader get_data_response = 6;

    pub fn clear_get_data_response(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_get_data_response(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::get_data_response(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_data_response(&mut self, v: GetDateResponseHeader) {
        self.response = ::std::option::Option::Some(Response_oneof_response::get_data_response(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_data_response(&mut self) -> &mut GetDateResponseHeader {
        if let ::std::option::Option::Some(Response_oneof_response::get_data_response(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::get_data_response(GetDateResponseHeader::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::get_data_response(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_data_response(&mut self) -> GetDateResponseHeader {
        if self.has_get_data_response() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::get_data_response(v)) => v,
                _ => panic!(),
            }
        } else {
            GetDateResponseHeader::new()
        }
    }

    pub fn get_get_data_response(&self) -> &GetDateResponseHeader {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::get_data_response(ref v)) => v,
            _ => GetDateResponseHeader::default_instance(),
        }
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        if let Some(Response_oneof_response::info_response(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::message_rate_reponse(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::get_data_response(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
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
                    let tmp = is.read_int32()?;
                    self.box_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seq = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp_ms = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::info_response(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::message_rate_reponse(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::get_data_response(is.read_message()?));
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
        if self.box_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.box_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.seq != 0 {
            my_size += ::protobuf::rt::value_size(2, self.seq, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.timestamp_ms != 0 {
            my_size += ::protobuf::rt::value_size(3, self.timestamp_ms, ::protobuf::wire_format::WireTypeVarint);
        }
        if let ::std::option::Option::Some(ref v) = self.response {
            match v {
                &Response_oneof_response::info_response(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::message_rate_reponse(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::get_data_response(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.box_id != 0 {
            os.write_int32(1, self.box_id)?;
        }
        if self.seq != 0 {
            os.write_uint32(2, self.seq)?;
        }
        if self.timestamp_ms != 0 {
            os.write_uint64(3, self.timestamp_ms)?;
        }
        if let ::std::option::Option::Some(ref v) = self.response {
            match v {
                &Response_oneof_response::info_response(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::message_rate_reponse(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::get_data_response(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "box_id",
                    Response::get_box_id_for_reflect,
                    Response::mut_box_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq",
                    Response::get_seq_for_reflect,
                    Response::mut_seq_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp_ms",
                    Response::get_timestamp_ms_for_reflect,
                    Response::mut_timestamp_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetInfoResponse>(
                    "info_response",
                    Response::has_info_response,
                    Response::get_info_response,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetMeasurementRateResponse>(
                    "message_rate_reponse",
                    Response::has_message_rate_reponse,
                    Response::get_message_rate_reponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetDateResponseHeader>(
                    "get_data_response",
                    Response::has_get_data_response,
                    Response::get_get_data_response,
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
        self.clear_box_id();
        self.clear_seq();
        self.clear_timestamp_ms();
        self.clear_info_response();
        self.clear_message_rate_reponse();
        self.clear_get_data_response();
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

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\ropqbox3.proto\x12\nopq.proto3\x1a\x0frustproto.proto\"\xc3\x01\n\x0b\
    Measurement\x12\x15\n\x06box_id\x18\x01\x20\x01(\rR\x05boxId\x12!\n\x0ct\
    imestamp_ms\x18\x02\x20\x01(\x04R\x0btimestampMs\x12>\n\x07metrics\x18\
    \x03\x20\x03(\x0b2$.opq.proto3.Measurement.MetricsEntryR\x07metrics\x1a:\
    \n\x0cMetricsEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\
    \x05value\x18\x02\x20\x01(\x02R\x05value:\x028\x01\"\x10\n\x0eGetInfoCom\
    mand\"V\n\x0eGetDataCommand\x12\x19\n\x08start_ms\x18\x01\x20\x01(\x04R\
    \x07startMs\x12\x15\n\x06end_ms\x18\x02\x20\x01(\x04R\x05endMs\x12\x12\n\
    \x04wait\x18\x03\x20\x01(\x08R\x04wait\"y\n\x1aSetMeasturementRateComman\
    d\x12:\n\x19measurement_window_cycles\x18\x01\x20\x01(\rR\x17measurement\
    WindowCycles\x12\x1f\n\x0bduration_ms\x18\x02\x20\x01(\rR\ndurationMs\"\
    \xa9\x02\n\x07Command\x12\x10\n\x03seq\x18\x01\x20\x01(\rR\x03seq\x12!\n\
    \x0ctimestamp_ms\x18\x02\x20\x01(\x04R\x0btimestampMs\x12?\n\x0cinfo_com\
    mand\x18\x03\x20\x01(\x0b2\x1a.opq.proto3.GetInfoCommandH\0R\x0binfoComm\
    and\x12?\n\x0cdata_command\x18\x04\x20\x01(\x0b2\x1a.opq.proto3.GetDataC\
    ommandH\0R\x0bdataCommand\x12\\\n\x15sampling_rate_command\x18\x05\x20\
    \x01(\x0b2&.opq.proto3.SetMeasturementRateCommandH\0R\x13samplingRateCom\
    mandB\t\n\x07command\"\xee\x01\n\x0fGetInfoResponse\x12\x19\n\x08mac_add\
    r\x18\x01\x20\x01(\tR\x07macAddr\x12!\n\x0cwifi_network\x18\x02\x20\x01(\
    \tR\x0bwifiNetwork\x12\x0e\n\x02ip\x18\x03\x20\x01(\tR\x02ip\x12\x16\n\
    \x06uptime\x18\x04\x20\x01(\x04R\x06uptime\x121\n\x14calibration_constan\
    t\x18\x05\x20\x01(\x04R\x13calibrationConstant\x12\x17\n\x07pub_key\x18\
    \x06\x20\x01(\tR\x06pubKey\x12)\n\x10measurement_rate\x18\x07\x20\x01(\r\
    R\x0fmeasurementRate\"D\n\x1aSetMeasurementRateResponse\x12&\n\x0fold_ra\
    te_cycles\x18\x01\x20\x01(\rR\roldRateCycles\"I\n\x15GetDateResponseHead\
    er\x12\x19\n\x08start_ts\x18\x01\x20\x01(\x04R\x07startTs\x12\x15\n\x06e\
    nd_ts\x18\x02\x20\x01(\x04R\x05endTs\"\xd3\x02\n\x08Response\x12\x15\n\
    \x06box_id\x18\x01\x20\x01(\x05R\x05boxId\x12\x10\n\x03seq\x18\x02\x20\
    \x01(\rR\x03seq\x12!\n\x0ctimestamp_ms\x18\x03\x20\x01(\x04R\x0btimestam\
    pMs\x12B\n\rinfo_response\x18\x04\x20\x01(\x0b2\x1b.opq.proto3.GetInfoRe\
    sponseH\0R\x0cinfoResponse\x12Z\n\x14message_rate_reponse\x18\x05\x20\
    \x01(\x0b2&.opq.proto3.SetMeasurementRateResponseH\0R\x12messageRateRepo\
    nse\x12O\n\x11get_data_response\x18\x06\x20\x01(\x0b2!.opq.proto3.GetDat\
    eResponseHeaderH\0R\x0fgetDataResponseB\n\n\x08responseB\x04\xc8\xa6\x08\
    \x01b\x06proto3\
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