// This file is generated by rust-protobuf 2.1.4. Do not edit
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

extern crate protobuf;

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default,Debug)]
pub struct glyph {
    // message fields
    id: ::std::option::Option<u32>,
    bitmap: protobuf::SingularField<::std::vec::Vec<u8>>,
    width: ::std::option::Option<u32>,
    height: ::std::option::Option<u32>,
    left: ::std::option::Option<i32>,
    top: ::std::option::Option<i32>,
    advance: ::std::option::Option<u32>,
    // special fields
    pub unknown_fields: protobuf::UnknownFields,
    pub cached_size: protobuf::CachedSize,
}

impl glyph {
    pub fn new() -> glyph {
        ::std::default::Default::default()
    }

    // required uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    // optional bytes bitmap = 2;

    pub fn clear_bitmap(&mut self) {
        self.bitmap.clear();
    }

    pub fn has_bitmap(&self) -> bool {
        self.bitmap.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bitmap(&mut self, v: ::std::vec::Vec<u8>) {
        self.bitmap = protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bitmap(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bitmap.is_none() {
            self.bitmap.set_default();
        }
        self.bitmap.as_mut().unwrap()
    }

    // Take field
    pub fn take_bitmap(&mut self) -> ::std::vec::Vec<u8> {
        self.bitmap.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bitmap(&self) -> &[u8] {
        match self.bitmap.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required uint32 width = 3;

    pub fn clear_width(&mut self) {
        self.width = ::std::option::Option::None;
    }

    pub fn has_width(&self) -> bool {
        self.width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: u32) {
        self.width = ::std::option::Option::Some(v);
    }

    pub fn get_width(&self) -> u32 {
        self.width.unwrap_or(0)
    }

    // required uint32 height = 4;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u32) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height(&self) -> u32 {
        self.height.unwrap_or(0)
    }

    // required sint32 left = 5;

    pub fn clear_left(&mut self) {
        self.left = ::std::option::Option::None;
    }

    pub fn has_left(&self) -> bool {
        self.left.is_some()
    }

    // Param is passed by value, moved
    pub fn set_left(&mut self, v: i32) {
        self.left = ::std::option::Option::Some(v);
    }

    pub fn get_left(&self) -> i32 {
        self.left.unwrap_or(0)
    }

    // required sint32 top = 6;

    pub fn clear_top(&mut self) {
        self.top = ::std::option::Option::None;
    }

    pub fn has_top(&self) -> bool {
        self.top.is_some()
    }

    // Param is passed by value, moved
    pub fn set_top(&mut self, v: i32) {
        self.top = ::std::option::Option::Some(v);
    }

    pub fn get_top(&self) -> i32 {
        self.top.unwrap_or(0)
    }

    // required uint32 advance = 7;

    pub fn clear_advance(&mut self) {
        self.advance = ::std::option::Option::None;
    }

    pub fn has_advance(&self) -> bool {
        self.advance.is_some()
    }

    // Param is passed by value, moved
    pub fn set_advance(&mut self, v: u32) {
        self.advance = ::std::option::Option::Some(v);
    }

    pub fn get_advance(&self) -> u32 {
        self.advance.unwrap_or(0)
    }
}

impl protobuf::Message for glyph {
    fn descriptor(&self) -> &'static protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        }
        if self.width.is_none() {
            return false;
        }
        if self.height.is_none() {
            return false;
        }
        if self.left.is_none() {
            return false;
        }
        if self.top.is_none() {
            return false;
        }
        if self.advance.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bitmap)?;
                },
                3 => {
                    if wire_type != protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.width = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.height = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.left = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.top = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.advance = ::std::option::Option::Some(tmp);
                },
                _ => {
                    protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.bitmap.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(v) = self.width {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.height {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.left {
            os.write_sint32(5, v)?;
        }
        if let Some(v) = self.top {
            os.write_sint32(6, v)?;
        }
        if let Some(v) = self.advance {
            os.write_uint32(7, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.id {
            my_size += protobuf::rt::value_size(1, v, protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.bitmap.as_ref() {
            my_size += protobuf::rt::bytes_size(2, &v);
        }
        if let Some(v) = self.width {
            my_size += protobuf::rt::value_size(3, v, protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.height {
            my_size += protobuf::rt::value_size(4, v, protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.left {
            my_size += protobuf::rt::value_varint_zigzag_size(5, v);
        }
        if let Some(v) = self.top {
            my_size += protobuf::rt::value_varint_zigzag_size(6, v);
        }
        if let Some(v) = self.advance {
            my_size += protobuf::rt::value_size(7, v, protobuf::wire_format::WireTypeVarint);
        }
        my_size += protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut protobuf::UnknownFields {
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

    fn new() -> glyph {
        glyph::new()
    }

    fn default_instance() -> &'static glyph {
        static mut instance: protobuf::lazy::Lazy<glyph> = protobuf::lazy::Lazy {
            lock: protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const glyph,
        };
        unsafe {
            instance.get(glyph::new)
        }
    }
}

impl protobuf::Clear for glyph {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_bitmap();
        self.clear_width();
        self.clear_height();
        self.clear_left();
        self.clear_top();
        self.clear_advance();
        self.unknown_fields.clear();
    }
}

impl protobuf::reflect::ProtobufValue for glyph {
    fn as_ref(&self) -> protobuf::reflect::ProtobufValueRef {
        protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
pub struct fontstack {
    // message fields
    name: protobuf::SingularField<::std::string::String>,
    range: protobuf::SingularField<::std::string::String>,
    glyphs: protobuf::RepeatedField<glyph>,
    // special fields
    pub unknown_fields: protobuf::UnknownFields,
    pub cached_size: protobuf::CachedSize,
}

impl fontstack {
    pub fn new() -> fontstack {
        ::std::default::Default::default()
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string range = 2;

    pub fn clear_range(&mut self) {
        self.range.clear();
    }

    pub fn has_range(&self) -> bool {
        self.range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range(&mut self, v: ::std::string::String) {
        self.range = protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range(&mut self) -> &mut ::std::string::String {
        if self.range.is_none() {
            self.range.set_default();
        }
        self.range.as_mut().unwrap()
    }

    // Take field
    pub fn take_range(&mut self) -> ::std::string::String {
        self.range.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_range(&self) -> &str {
        match self.range.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .llmr.glyphs.glyph glyphs = 3;

    pub fn clear_glyphs(&mut self) {
        self.glyphs.clear();
    }

    // Param is passed by value, moved
    pub fn set_glyphs(&mut self, v: protobuf::RepeatedField<glyph>) {
        self.glyphs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_glyphs(&mut self) -> &mut protobuf::RepeatedField<glyph> {
        &mut self.glyphs
    }

    // Take field
    pub fn take_glyphs(&mut self) -> protobuf::RepeatedField<glyph> {
        ::std::mem::replace(&mut self.glyphs, protobuf::RepeatedField::new())
    }

    pub fn get_glyphs(&self) -> &[glyph] {
        &self.glyphs
    }
}

impl protobuf::Message for fontstack {
    fn descriptor(&self) -> &'static protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        if self.range.is_none() {
            return false;
        }
        for v in &self.glyphs {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    protobuf::rt::read_singular_string_into(wire_type, is, &mut self.range)?;
                },
                3 => {
                    protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.glyphs)?;
                },
                _ => {
                    protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.range.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.glyphs {
            os.write_tag(3, protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.range.as_ref() {
            my_size += protobuf::rt::string_size(2, &v);
        }
        for value in &self.glyphs {
            let len = value.compute_size();
            my_size += 1 + protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut protobuf::UnknownFields {
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

    fn new() -> fontstack {
        fontstack::new()
    }

    fn default_instance() -> &'static fontstack {
        static mut instance: protobuf::lazy::Lazy<fontstack> = protobuf::lazy::Lazy {
            lock: protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const fontstack,
        };
        unsafe {
            instance.get(fontstack::new)
        }
    }
}

impl protobuf::Clear for fontstack {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_range();
        self.clear_glyphs();
        self.unknown_fields.clear();
    }
}

impl protobuf::reflect::ProtobufValue for fontstack {
    fn as_ref(&self) -> protobuf::reflect::ProtobufValueRef {
        protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
pub struct glyphs {
    // message fields
    stacks: protobuf::RepeatedField<fontstack>,
    // special fields
    pub unknown_fields: protobuf::UnknownFields,
    pub cached_size: protobuf::CachedSize,
}

impl glyphs {
    pub fn new() -> glyphs {
        ::std::default::Default::default()
    }

    // repeated .llmr.glyphs.fontstack stacks = 1;

    pub fn clear_stacks(&mut self) {
        self.stacks.clear();
    }

    // Param is passed by value, moved
    pub fn set_stacks(&mut self, v: protobuf::RepeatedField<fontstack>) {
        self.stacks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stacks(&mut self) -> &mut protobuf::RepeatedField<fontstack> {
        &mut self.stacks
    }

    // Take field
    pub fn take_stacks(&mut self) -> protobuf::RepeatedField<fontstack> {
        ::std::mem::replace(&mut self.stacks, protobuf::RepeatedField::new())
    }

    pub fn get_stacks(&self) -> &[fontstack] {
        &self.stacks
    }
}

impl protobuf::Message for glyphs {
    fn descriptor(&self) -> &'static protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn is_initialized(&self) -> bool {
        for v in &self.stacks {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.stacks)?;
                },
                _ => {
                    protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::ProtobufResult<()> {
        for v in &self.stacks {
            os.write_tag(1, protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.stacks {
            let len = value.compute_size();
            my_size += 1 + protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut protobuf::UnknownFields {
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

    fn new() -> glyphs {
        glyphs::new()
    }

    fn default_instance() -> &'static glyphs {
        static mut instance: protobuf::lazy::Lazy<glyphs> = protobuf::lazy::Lazy {
            lock: protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const glyphs,
        };
        unsafe {
            instance.get(glyphs::new)
        }
    }
}

impl protobuf::Clear for glyphs {
    fn clear(&mut self) {
        self.clear_stacks();
        self.unknown_fields.clear();
    }
}

impl protobuf::reflect::ProtobufValue for glyphs {
    fn as_ref(&self) -> protobuf::reflect::ProtobufValueRef {
        protobuf::reflect::ProtobufValueRef::Message(self)
    }
}
