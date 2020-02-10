// Copyright 2020 Materialize, Inc. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.
// This file is generated by rust-protobuf 2.10.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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
//! Generated file from `resources/billing.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_10_1;

#[derive(PartialEq,Clone,Default)]
pub struct ResourceInfo {
    // message fields
    pub cpu_num: i32,
    pub memory_gb: i32,
    pub disk_gb: i32,
    pub client_id: i32,
    pub vm_id: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ResourceInfo {
    fn default() -> &'a ResourceInfo {
        <ResourceInfo as ::protobuf::Message>::default_instance()
    }
}

impl ResourceInfo {
    pub fn new() -> ResourceInfo {
        ::std::default::Default::default()
    }

    // int32 cpu_num = 1;


    pub fn get_cpu_num(&self) -> i32 {
        self.cpu_num
    }
    pub fn clear_cpu_num(&mut self) {
        self.cpu_num = 0;
    }

    // Param is passed by value, moved
    pub fn set_cpu_num(&mut self, v: i32) {
        self.cpu_num = v;
    }

    // int32 memory_gb = 2;


    pub fn get_memory_gb(&self) -> i32 {
        self.memory_gb
    }
    pub fn clear_memory_gb(&mut self) {
        self.memory_gb = 0;
    }

    // Param is passed by value, moved
    pub fn set_memory_gb(&mut self, v: i32) {
        self.memory_gb = v;
    }

    // int32 disk_gb = 3;


    pub fn get_disk_gb(&self) -> i32 {
        self.disk_gb
    }
    pub fn clear_disk_gb(&mut self) {
        self.disk_gb = 0;
    }

    // Param is passed by value, moved
    pub fn set_disk_gb(&mut self, v: i32) {
        self.disk_gb = v;
    }

    // int32 client_id = 4;


    pub fn get_client_id(&self) -> i32 {
        self.client_id
    }
    pub fn clear_client_id(&mut self) {
        self.client_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_client_id(&mut self, v: i32) {
        self.client_id = v;
    }

    // int32 vm_id = 5;


    pub fn get_vm_id(&self) -> i32 {
        self.vm_id
    }
    pub fn clear_vm_id(&mut self) {
        self.vm_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_vm_id(&mut self, v: i32) {
        self.vm_id = v;
    }
}

impl ::protobuf::Message for ResourceInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.cpu_num = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.memory_gb = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.disk_gb = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.client_id = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.vm_id = tmp;
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
        if self.cpu_num != 0 {
            my_size += ::protobuf::rt::value_size(1, self.cpu_num, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.memory_gb != 0 {
            my_size += ::protobuf::rt::value_size(2, self.memory_gb, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.disk_gb != 0 {
            my_size += ::protobuf::rt::value_size(3, self.disk_gb, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.client_id != 0 {
            my_size += ::protobuf::rt::value_size(4, self.client_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.vm_id != 0 {
            my_size += ::protobuf::rt::value_size(5, self.vm_id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.cpu_num != 0 {
            os.write_int32(1, self.cpu_num)?;
        }
        if self.memory_gb != 0 {
            os.write_int32(2, self.memory_gb)?;
        }
        if self.disk_gb != 0 {
            os.write_int32(3, self.disk_gb)?;
        }
        if self.client_id != 0 {
            os.write_int32(4, self.client_id)?;
        }
        if self.vm_id != 0 {
            os.write_int32(5, self.vm_id)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ResourceInfo {
        ResourceInfo::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "cpu_num",
                    |m: &ResourceInfo| { &m.cpu_num },
                    |m: &mut ResourceInfo| { &mut m.cpu_num },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "memory_gb",
                    |m: &ResourceInfo| { &m.memory_gb },
                    |m: &mut ResourceInfo| { &mut m.memory_gb },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "disk_gb",
                    |m: &ResourceInfo| { &m.disk_gb },
                    |m: &mut ResourceInfo| { &mut m.disk_gb },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "client_id",
                    |m: &ResourceInfo| { &m.client_id },
                    |m: &mut ResourceInfo| { &mut m.client_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "vm_id",
                    |m: &ResourceInfo| { &m.vm_id },
                    |m: &mut ResourceInfo| { &mut m.vm_id },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResourceInfo>(
                    "ResourceInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ResourceInfo {
        static mut instance: ::protobuf::lazy::Lazy<ResourceInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResourceInfo,
        };
        unsafe {
            instance.get(ResourceInfo::new)
        }
    }
}

impl ::protobuf::Clear for ResourceInfo {
    fn clear(&mut self) {
        self.cpu_num = 0;
        self.memory_gb = 0;
        self.disk_gb = 0;
        self.client_id = 0;
        self.vm_id = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResourceInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResourceInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Record {
    // message fields
    pub id: ::std::string::String,
    pub interval_start: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    pub interval_end: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    pub meter: ::std::string::String,
    pub value: u32,
    pub info: ::protobuf::SingularPtrField<ResourceInfo>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Record {
    fn default() -> &'a Record {
        <Record as ::protobuf::Message>::default_instance()
    }
}

impl Record {
    pub fn new() -> Record {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // .google.protobuf.Timestamp interval_start = 2;


    pub fn get_interval_start(&self) -> &::protobuf::well_known_types::Timestamp {
        self.interval_start.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::default_instance())
    }
    pub fn clear_interval_start(&mut self) {
        self.interval_start.clear();
    }

    pub fn has_interval_start(&self) -> bool {
        self.interval_start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_interval_start(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.interval_start = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_interval_start(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.interval_start.is_none() {
            self.interval_start.set_default();
        }
        self.interval_start.as_mut().unwrap()
    }

    // Take field
    pub fn take_interval_start(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.interval_start.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    // .google.protobuf.Timestamp interval_end = 3;


    pub fn get_interval_end(&self) -> &::protobuf::well_known_types::Timestamp {
        self.interval_end.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::default_instance())
    }
    pub fn clear_interval_end(&mut self) {
        self.interval_end.clear();
    }

    pub fn has_interval_end(&self) -> bool {
        self.interval_end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_interval_end(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.interval_end = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_interval_end(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.interval_end.is_none() {
            self.interval_end.set_default();
        }
        self.interval_end.as_mut().unwrap()
    }

    // Take field
    pub fn take_interval_end(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.interval_end.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    // string meter = 4;


    pub fn get_meter(&self) -> &str {
        &self.meter
    }
    pub fn clear_meter(&mut self) {
        self.meter.clear();
    }

    // Param is passed by value, moved
    pub fn set_meter(&mut self, v: ::std::string::String) {
        self.meter = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_meter(&mut self) -> &mut ::std::string::String {
        &mut self.meter
    }

    // Take field
    pub fn take_meter(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.meter, ::std::string::String::new())
    }

    // uint32 value = 5;


    pub fn get_value(&self) -> u32 {
        self.value
    }
    pub fn clear_value(&mut self) {
        self.value = 0;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: u32) {
        self.value = v;
    }

    // .billing.ResourceInfo info = 6;


    pub fn get_info(&self) -> &ResourceInfo {
        self.info.as_ref().unwrap_or_else(|| ResourceInfo::default_instance())
    }
    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: ResourceInfo) {
        self.info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info(&mut self) -> &mut ResourceInfo {
        if self.info.is_none() {
            self.info.set_default();
        }
        self.info.as_mut().unwrap()
    }

    // Take field
    pub fn take_info(&mut self) -> ResourceInfo {
        self.info.take().unwrap_or_else(|| ResourceInfo::new())
    }
}

impl ::protobuf::Message for Record {
    fn is_initialized(&self) -> bool {
        for v in &self.interval_start {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.interval_end {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.info {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.interval_start)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.interval_end)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.meter)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.value = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.info)?;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if let Some(ref v) = self.interval_start.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.interval_end.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.meter.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.meter);
        }
        if self.value != 0 {
            my_size += ::protobuf::rt::value_size(5, self.value, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if let Some(ref v) = self.interval_start.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.interval_end.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.meter.is_empty() {
            os.write_string(4, &self.meter)?;
        }
        if self.value != 0 {
            os.write_uint32(5, self.value)?;
        }
        if let Some(ref v) = self.info.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Record {
        Record::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    |m: &Record| { &m.id },
                    |m: &mut Record| { &mut m.id },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                    "interval_start",
                    |m: &Record| { &m.interval_start },
                    |m: &mut Record| { &mut m.interval_start },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                    "interval_end",
                    |m: &Record| { &m.interval_end },
                    |m: &mut Record| { &mut m.interval_end },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "meter",
                    |m: &Record| { &m.meter },
                    |m: &mut Record| { &mut m.meter },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "value",
                    |m: &Record| { &m.value },
                    |m: &mut Record| { &mut m.value },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResourceInfo>>(
                    "info",
                    |m: &Record| { &m.info },
                    |m: &mut Record| { &mut m.info },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Record>(
                    "Record",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Record {
        static mut instance: ::protobuf::lazy::Lazy<Record> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Record,
        };
        unsafe {
            instance.get(Record::new)
        }
    }
}

impl ::protobuf::Clear for Record {
    fn clear(&mut self) {
        self.id.clear();
        self.interval_start.clear();
        self.interval_end.clear();
        self.meter.clear();
        self.value = 0;
        self.info.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Record {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Record {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Batch {
    // message fields
    pub id: ::std::string::String,
    pub interval_start: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    pub interval_end: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    pub records: ::protobuf::RepeatedField<Record>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Batch {
    fn default() -> &'a Batch {
        <Batch as ::protobuf::Message>::default_instance()
    }
}

impl Batch {
    pub fn new() -> Batch {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // .google.protobuf.Timestamp interval_start = 3;


    pub fn get_interval_start(&self) -> &::protobuf::well_known_types::Timestamp {
        self.interval_start.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::default_instance())
    }
    pub fn clear_interval_start(&mut self) {
        self.interval_start.clear();
    }

    pub fn has_interval_start(&self) -> bool {
        self.interval_start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_interval_start(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.interval_start = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_interval_start(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.interval_start.is_none() {
            self.interval_start.set_default();
        }
        self.interval_start.as_mut().unwrap()
    }

    // Take field
    pub fn take_interval_start(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.interval_start.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    // .google.protobuf.Timestamp interval_end = 4;


    pub fn get_interval_end(&self) -> &::protobuf::well_known_types::Timestamp {
        self.interval_end.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::default_instance())
    }
    pub fn clear_interval_end(&mut self) {
        self.interval_end.clear();
    }

    pub fn has_interval_end(&self) -> bool {
        self.interval_end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_interval_end(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.interval_end = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_interval_end(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.interval_end.is_none() {
            self.interval_end.set_default();
        }
        self.interval_end.as_mut().unwrap()
    }

    // Take field
    pub fn take_interval_end(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.interval_end.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    // repeated .billing.Record records = 7;


    pub fn get_records(&self) -> &[Record] {
        &self.records
    }
    pub fn clear_records(&mut self) {
        self.records.clear();
    }

    // Param is passed by value, moved
    pub fn set_records(&mut self, v: ::protobuf::RepeatedField<Record>) {
        self.records = v;
    }

    // Mutable pointer to the field.
    pub fn mut_records(&mut self) -> &mut ::protobuf::RepeatedField<Record> {
        &mut self.records
    }

    // Take field
    pub fn take_records(&mut self) -> ::protobuf::RepeatedField<Record> {
        ::std::mem::replace(&mut self.records, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Batch {
    fn is_initialized(&self) -> bool {
        for v in &self.interval_start {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.interval_end {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.records {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.interval_start)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.interval_end)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.records)?;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if let Some(ref v) = self.interval_start.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.interval_end.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.records {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if let Some(ref v) = self.interval_start.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.interval_end.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.records {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Batch {
        Batch::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    |m: &Batch| { &m.id },
                    |m: &mut Batch| { &mut m.id },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                    "interval_start",
                    |m: &Batch| { &m.interval_start },
                    |m: &mut Batch| { &mut m.interval_start },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                    "interval_end",
                    |m: &Batch| { &m.interval_end },
                    |m: &mut Batch| { &mut m.interval_end },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Record>>(
                    "records",
                    |m: &Batch| { &m.records },
                    |m: &mut Batch| { &mut m.records },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Batch>(
                    "Batch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Batch {
        static mut instance: ::protobuf::lazy::Lazy<Batch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Batch,
        };
        unsafe {
            instance.get(Batch::new)
        }
    }
}

impl ::protobuf::Clear for Batch {
    fn clear(&mut self) {
        self.id.clear();
        self.interval_start.clear();
        self.interval_end.clear();
        self.records.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Batch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Batch {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17resources/billing.proto\x12\x07billing\x1a\x1fgoogle/protobuf/time\
    stamp.proto\"\x8f\x01\n\x0cResourceInfo\x12\x17\n\x07cpu_num\x18\x01\x20\
    \x01(\x05R\x06cpuNum\x12\x1b\n\tmemory_gb\x18\x02\x20\x01(\x05R\x08memor\
    yGb\x12\x17\n\x07disk_gb\x18\x03\x20\x01(\x05R\x06diskGb\x12\x1b\n\tclie\
    nt_id\x18\x04\x20\x01(\x05R\x08clientId\x12\x13\n\x05vm_id\x18\x05\x20\
    \x01(\x05R\x04vmId\"\xf1\x01\n\x06Record\x12\x0e\n\x02id\x18\x01\x20\x01\
    (\tR\x02id\x12A\n\x0einterval_start\x18\x02\x20\x01(\x0b2\x1a.google.pro\
    tobuf.TimestampR\rintervalStart\x12=\n\x0cinterval_end\x18\x03\x20\x01(\
    \x0b2\x1a.google.protobuf.TimestampR\x0bintervalEnd\x12\x14\n\x05meter\
    \x18\x04\x20\x01(\tR\x05meter\x12\x14\n\x05value\x18\x05\x20\x01(\rR\x05\
    value\x12)\n\x04info\x18\x06\x20\x01(\x0b2\x15.billing.ResourceInfoR\x04\
    info\"\xc4\x01\n\x05Batch\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12A\
    \n\x0einterval_start\x18\x03\x20\x01(\x0b2\x1a.google.protobuf.Timestamp\
    R\rintervalStart\x12=\n\x0cinterval_end\x18\x04\x20\x01(\x0b2\x1a.google\
    .protobuf.TimestampR\x0bintervalEnd\x12)\n\x07records\x18\x07\x20\x03(\
    \x0b2\x0f.billing.RecordR\x07recordsb\x06proto3\
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
