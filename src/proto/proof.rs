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

#[derive(Clone,Default)]
pub struct ProofProto {
    // message fields
    root_hash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    lemma: ::protobuf::SingularPtrField<LemmaProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProofProto {}

impl ProofProto {
    pub fn new() -> ProofProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProofProto {
        static mut instance: ::protobuf::lazy::Lazy<ProofProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProofProto,
        };
        unsafe {
            instance.get(|| {
                ProofProto {
                    root_hash: ::protobuf::SingularField::none(),
                    lemma: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes root_hash = 1;

    pub fn clear_root_hash(&mut self) {
        self.root_hash.clear();
    }

    pub fn has_root_hash(&self) -> bool {
        self.root_hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_root_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.root_hash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_root_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.root_hash.is_none() {
            self.root_hash.set_default();
        };
        self.root_hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_root_hash(&mut self) -> ::std::vec::Vec<u8> {
        self.root_hash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_root_hash(&self) -> &[u8] {
        match self.root_hash.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .LemmaProto lemma = 2;

    pub fn clear_lemma(&mut self) {
        self.lemma.clear();
    }

    pub fn has_lemma(&self) -> bool {
        self.lemma.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lemma(&mut self, v: LemmaProto) {
        self.lemma = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lemma(&mut self) -> &mut LemmaProto {
        if self.lemma.is_none() {
            self.lemma.set_default();
        };
        self.lemma.as_mut().unwrap()
    }

    // Take field
    pub fn take_lemma(&mut self) -> LemmaProto {
        self.lemma.take().unwrap_or_else(|| LemmaProto::new())
    }

    pub fn get_lemma(&self) -> &LemmaProto {
        self.lemma.as_ref().unwrap_or_else(|| LemmaProto::default_instance())
    }
}

impl ::protobuf::Message for ProofProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.root_hash));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lemma));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.root_hash {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.lemma {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.root_hash.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.lemma.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ProofProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ProofProto {
    fn new() -> ProofProto {
        ProofProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProofProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "root_hash",
                    ProofProto::has_root_hash,
                    ProofProto::get_root_hash,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "lemma",
                    ProofProto::has_lemma,
                    ProofProto::get_lemma,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProofProto>(
                    "ProofProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProofProto {
    fn clear(&mut self) {
        self.clear_root_hash();
        self.clear_lemma();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ProofProto {
    fn eq(&self, other: &ProofProto) -> bool {
        self.root_hash == other.root_hash &&
        self.lemma == other.lemma &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ProofProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct LemmaProto {
    // message fields
    node_hash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    sub_lemma: ::protobuf::SingularPtrField<LemmaProto>,
    // message oneof groups
    sibling_hash: ::std::option::Option<LemmaProto_oneof_sibling_hash>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LemmaProto {}

#[derive(Clone,PartialEq)]
pub enum LemmaProto_oneof_sibling_hash {
    left_sibling_hash(::std::vec::Vec<u8>),
    right_sibling_hash(::std::vec::Vec<u8>),
}

impl LemmaProto {
    pub fn new() -> LemmaProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LemmaProto {
        static mut instance: ::protobuf::lazy::Lazy<LemmaProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LemmaProto,
        };
        unsafe {
            instance.get(|| {
                LemmaProto {
                    node_hash: ::protobuf::SingularField::none(),
                    sub_lemma: ::protobuf::SingularPtrField::none(),
                    sibling_hash: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes node_hash = 1;

    pub fn clear_node_hash(&mut self) {
        self.node_hash.clear();
    }

    pub fn has_node_hash(&self) -> bool {
        self.node_hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.node_hash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.node_hash.is_none() {
            self.node_hash.set_default();
        };
        self.node_hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_node_hash(&mut self) -> ::std::vec::Vec<u8> {
        self.node_hash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_node_hash(&self) -> &[u8] {
        match self.node_hash.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .LemmaProto sub_lemma = 2;

    pub fn clear_sub_lemma(&mut self) {
        self.sub_lemma.clear();
    }

    pub fn has_sub_lemma(&self) -> bool {
        self.sub_lemma.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sub_lemma(&mut self, v: LemmaProto) {
        self.sub_lemma = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sub_lemma(&mut self) -> &mut LemmaProto {
        if self.sub_lemma.is_none() {
            self.sub_lemma.set_default();
        };
        self.sub_lemma.as_mut().unwrap()
    }

    // Take field
    pub fn take_sub_lemma(&mut self) -> LemmaProto {
        self.sub_lemma.take().unwrap_or_else(|| LemmaProto::new())
    }

    pub fn get_sub_lemma(&self) -> &LemmaProto {
        self.sub_lemma.as_ref().unwrap_or_else(|| LemmaProto::default_instance())
    }

    // optional bytes left_sibling_hash = 3;

    pub fn clear_left_sibling_hash(&mut self) {
        self.sibling_hash = ::std::option::Option::None;
    }

    pub fn has_left_sibling_hash(&self) -> bool {
        match self.sibling_hash {
            ::std::option::Option::Some(LemmaProto_oneof_sibling_hash::left_sibling_hash(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_left_sibling_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.sibling_hash = ::std::option::Option::Some(LemmaProto_oneof_sibling_hash::left_sibling_hash(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_left_sibling_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(LemmaProto_oneof_sibling_hash::left_sibling_hash(_)) = self.sibling_hash {
        } else {
            self.sibling_hash = ::std::option::Option::Some(LemmaProto_oneof_sibling_hash::left_sibling_hash(::std::vec::Vec::new()));
        }
        match self.sibling_hash {
            ::std::option::Option::Some(LemmaProto_oneof_sibling_hash::left_sibling_hash(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_left_sibling_hash(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_left_sibling_hash() {
            match self.sibling_hash.take() {
                ::std::option::Option::Some(LemmaProto_oneof_sibling_hash::left_sibling_hash(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_left_sibling_hash(&self) -> &[u8] {
        match self.sibling_hash {
            ::std::option::Option::Some(LemmaProto_oneof_sibling_hash::left_sibling_hash(ref v)) => v,
            _ => &[],
        }
    }

    // optional bytes right_sibling_hash = 4;

    pub fn clear_right_sibling_hash(&mut self) {
        self.sibling_hash = ::std::option::Option::None;
    }

    pub fn has_right_sibling_hash(&self) -> bool {
        match self.sibling_hash {
            ::std::option::Option::Some(LemmaProto_oneof_sibling_hash::right_sibling_hash(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_right_sibling_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.sibling_hash = ::std::option::Option::Some(LemmaProto_oneof_sibling_hash::right_sibling_hash(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_right_sibling_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(LemmaProto_oneof_sibling_hash::right_sibling_hash(_)) = self.sibling_hash {
        } else {
            self.sibling_hash = ::std::option::Option::Some(LemmaProto_oneof_sibling_hash::right_sibling_hash(::std::vec::Vec::new()));
        }
        match self.sibling_hash {
            ::std::option::Option::Some(LemmaProto_oneof_sibling_hash::right_sibling_hash(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_right_sibling_hash(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_right_sibling_hash() {
            match self.sibling_hash.take() {
                ::std::option::Option::Some(LemmaProto_oneof_sibling_hash::right_sibling_hash(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_right_sibling_hash(&self) -> &[u8] {
        match self.sibling_hash {
            ::std::option::Option::Some(LemmaProto_oneof_sibling_hash::right_sibling_hash(ref v)) => v,
            _ => &[],
        }
    }
}

impl ::protobuf::Message for LemmaProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.node_hash));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.sub_lemma));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.sibling_hash = ::std::option::Option::Some(LemmaProto_oneof_sibling_hash::left_sibling_hash(try!(is.read_bytes())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.sibling_hash = ::std::option::Option::Some(LemmaProto_oneof_sibling_hash::right_sibling_hash(try!(is.read_bytes())));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.node_hash {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.sub_lemma {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let ::std::option::Option::Some(ref v) = self.sibling_hash {
            match v {
                &LemmaProto_oneof_sibling_hash::left_sibling_hash(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(3, &v);
                },
                &LemmaProto_oneof_sibling_hash::right_sibling_hash(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(4, &v);
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.node_hash.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.sub_lemma.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let ::std::option::Option::Some(ref v) = self.sibling_hash {
            match v {
                &LemmaProto_oneof_sibling_hash::left_sibling_hash(ref v) => {
                    try!(os.write_bytes(3, v));
                },
                &LemmaProto_oneof_sibling_hash::right_sibling_hash(ref v) => {
                    try!(os.write_bytes(4, v));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<LemmaProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LemmaProto {
    fn new() -> LemmaProto {
        LemmaProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<LemmaProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "node_hash",
                    LemmaProto::has_node_hash,
                    LemmaProto::get_node_hash,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "sub_lemma",
                    LemmaProto::has_sub_lemma,
                    LemmaProto::get_sub_lemma,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "left_sibling_hash",
                    LemmaProto::has_left_sibling_hash,
                    LemmaProto::get_left_sibling_hash,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "right_sibling_hash",
                    LemmaProto::has_right_sibling_hash,
                    LemmaProto::get_right_sibling_hash,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LemmaProto>(
                    "LemmaProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LemmaProto {
    fn clear(&mut self) {
        self.clear_node_hash();
        self.clear_sub_lemma();
        self.clear_left_sibling_hash();
        self.clear_right_sibling_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LemmaProto {
    fn eq(&self, other: &LemmaProto) -> bool {
        self.node_hash == other.node_hash &&
        self.sub_lemma == other.sub_lemma &&
        self.sibling_hash == other.sibling_hash &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LemmaProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x14, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x70, 0x72, 0x6f, 0x6f, 0x66,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x4c, 0x0a, 0x0a, 0x50, 0x72, 0x6f, 0x6f, 0x66, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1b, 0x0a, 0x09, 0x72, 0x6f, 0x6f, 0x74, 0x5f, 0x68, 0x61, 0x73,
    0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x08, 0x72, 0x6f, 0x6f, 0x74, 0x48, 0x61, 0x73,
    0x68, 0x12, 0x21, 0x0a, 0x05, 0x6c, 0x65, 0x6d, 0x6d, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0b, 0x2e, 0x4c, 0x65, 0x6d, 0x6d, 0x61, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x52, 0x05, 0x6c,
    0x65, 0x6d, 0x6d, 0x61, 0x22, 0xc1, 0x01, 0x0a, 0x0a, 0x4c, 0x65, 0x6d, 0x6d, 0x61, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x1b, 0x0a, 0x09, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x68, 0x61, 0x73, 0x68,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x08, 0x6e, 0x6f, 0x64, 0x65, 0x48, 0x61, 0x73, 0x68,
    0x12, 0x28, 0x0a, 0x09, 0x73, 0x75, 0x62, 0x5f, 0x6c, 0x65, 0x6d, 0x6d, 0x61, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x4c, 0x65, 0x6d, 0x6d, 0x61, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x52, 0x08, 0x73, 0x75, 0x62, 0x4c, 0x65, 0x6d, 0x6d, 0x61, 0x12, 0x2c, 0x0a, 0x11, 0x6c, 0x65,
    0x66, 0x74, 0x5f, 0x73, 0x69, 0x62, 0x6c, 0x69, 0x6e, 0x67, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0c, 0x48, 0x00, 0x52, 0x0f, 0x6c, 0x65, 0x66, 0x74, 0x53, 0x69, 0x62,
    0x6c, 0x69, 0x6e, 0x67, 0x48, 0x61, 0x73, 0x68, 0x12, 0x2e, 0x0a, 0x12, 0x72, 0x69, 0x67, 0x68,
    0x74, 0x5f, 0x73, 0x69, 0x62, 0x6c, 0x69, 0x6e, 0x67, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0c, 0x48, 0x00, 0x52, 0x10, 0x72, 0x69, 0x67, 0x68, 0x74, 0x53, 0x69, 0x62,
    0x6c, 0x69, 0x6e, 0x67, 0x48, 0x61, 0x73, 0x68, 0x42, 0x0e, 0x0a, 0x0c, 0x73, 0x69, 0x62, 0x6c,
    0x69, 0x6e, 0x67, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x4a, 0xe4, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x01,
    0x00, 0x11, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x01, 0x00, 0x12, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x03, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x03, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04,
    0x02, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x04, 0x02, 0x03,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x04, 0x02, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x04, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x05, 0x02, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x05, 0x02, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x05, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x05, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x05, 0x15,
    0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x08, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x08, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x09, 0x02, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x09, 0x02, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x09, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x14, 0x15, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0a, 0x02, 0x09, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0a, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0a, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x0a, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x08, 0x00, 0x12, 0x04, 0x0c,
    0x02, 0x0f, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x08, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x08,
    0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x04, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0d, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x0a, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03,
    0x12, 0x03, 0x0e, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x0e, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x0a,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0e, 0x1f, 0x20, 0x62,
    0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
