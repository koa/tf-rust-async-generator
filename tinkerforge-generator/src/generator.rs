use std::{
    collections::HashSet,
    default::Default,
    env::{self, current_dir},
    fs, path,
};
use std::path::PathBuf;

use convert_case::{Case, Casing};
use prettyplease::unparse;
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{
    Arm,
    Block,
    Expr,
    ExprMatch, Field, FieldMutability, FieldValue, File, Ident, ImplItem, ImplItemFn, Item, ItemImpl,
    ItemMod, Lit, parse_quote, Path, PathArguments, PathSegment, punctuated::Punctuated, Stmt, token::{Comma, PathSep, Pub}, Type,
    TypePath, Variant, Visibility,
};

use crate::json_model::{
    JsonAnyDefaultValue, JsonCategory, JsonConstantGroup, JsonContent, JsonDirection, JsonElement,
    JsonLevel, JsonLocale, JsonPacketDescription, JsonPacketType, JsonRole,
};

pub fn parse_json() {
    let file = process_directory(
        current_dir()
            .expect("Cannot access current directory")
            .join("bindings"),
    );

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = path::Path::new(&out_dir).join("bindings.rs");
    fs::write(dest_path, unparse(&file)).expect("Cannot write source file");
}

pub fn process_directory(bindings_dir: PathBuf) -> File {
    generate_code(
        bindings_dir
            .read_dir()
            .expect("Cannot read directory")
            .into_iter()
            .map(|entry| entry.expect("cannot access directory entry"))
            .filter(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .map(|name| name.ends_with(".json"))
                    .unwrap_or(false)
            })
            .map(|json_dir_entry| {
                fs::File::open(&json_dir_entry.path()).expect("Cannot open json file")
            })
            .map(|json_file| {
                serde_json::from_reader::<_, JsonContent>(json_file).expect("Cannot parse json")
            }),
    )
}

pub fn generate_code<IT: Iterator<Item=JsonContent>>(file_contents: IT) -> File {
    let mut bindings_content = Vec::new();

    let mut device_variants: Punctuated<Variant, Comma> = Default::default();
    let mut device_encode_arms = Vec::new();
    let mut device_parse_arms = Vec::new();
    let mut device_name_arms = Vec::new();

    for tf_device in file_contents {
        if tf_device.device_identifier < 1 {
            println!("Invalid device identifier: {tf_device:#?}");
            continue;
        }
        let device_id: u16 = tf_device.device_identifier as u16;
        let raw_package_name = tf_device.name;
        println!("Device: {raw_package_name}");

        if raw_package_name.as_ref() == "Unknown" {
            // probleme mit doppelten einträgen in der config
            continue;
        }

        let package_name = raw_package_name.to_string().to_case(Case::Snake);
        let package_ident = create_ident(&package_name);
        let package_path = parse_quote!(crate::bindings::#package_ident);

        let device_name_prefix = raw_package_name.to_string().to_case(Case::UpperCamel);
        let device_st = match tf_device.category {
            JsonCategory::Brick => {
                format!("{}Brick", device_name_prefix)
            }
            JsonCategory::Bricklet => {
                format!("{}Bricklet", device_name_prefix)
            }
            JsonCategory::TNG => {
                format!("{}TNG", device_name_prefix)
            }
        };
        let device_struct_name = Ident::new(&device_st, Span::call_site());

        device_variants.push(parse_quote!(#device_struct_name));
        device_encode_arms.push(parse_quote!(DeviceIdentifier::#device_struct_name =>#device_id));
        device_parse_arms
            .push(parse_quote!(#device_id => Ok(DeviceIdentifier::#device_struct_name)));
        device_name_arms
            .push(parse_quote!(DeviceIdentifier::#device_struct_name =>#raw_package_name));

        let mut items = Vec::new();
        items.push(parse_quote!(
            #[allow(unused_imports)]
            use tinkerforge_base::byte_converter::{FromByteSlice, ToBytes};
        ));
        items.push(parse_quote!(
            #[allow(unused_imports)]
            use tokio_stream::StreamExt;
        ));
        items.push(parse_quote!(
            #[allow(unused_imports)]
            use std::convert::TryInto;
        ));

        println!("Name: {raw_package_name}");
        println!("Package: {package_name}");
        //println!("Tf Device: {tf_device:#?}");
        items.push(parse_quote!(
            #[derive(Clone, Debug)]
            pub struct #device_struct_name {
                device: tinkerforge_base::device::Device,
            }
        ));
        let mut device_impl: ItemImpl = parse_quote!(
            impl #device_struct_name {
                pub fn new(uid: impl Into<tinkerforge_base::base58::Uid>, connection: tinkerforge_base::ip_connection::async_io::AsyncIpConnection) -> #device_struct_name {
                    Self{
                        device: tinkerforge_base::device::Device::new(uid.into(),connection,#raw_package_name)
                    }
                }
                pub fn uid(&self)->tinkerforge_base::base58::Uid{
                    self.device.uid()
                }
            }
        );
        let mut already_declared_constants = HashSet::new();
        for packet_description in tf_device.packets.iter() {
            if packet_description.level == JsonLevel::High {
                continue;
            }
            let function = generate_packet_element_item(
                &mut items,
                packet_description,
                &package_path,
                &mut already_declared_constants,
            );
            device_impl.items.push(ImplItem::Fn(function));
        }
        items.push(Item::Impl(device_impl));
        bindings_content.push(Item::Mod(ItemMod {
            attrs: vec![],
            vis: Visibility::Public(Default::default()),
            unsafety: None,
            mod_token: Default::default(),
            ident: package_ident,
            content: Some((Default::default(), items)),
            semi: None,
        }));
    }
    device_parse_arms.push(parse_quote!(_ => Err(())));

    bindings_content.push(Item::Enum(parse_quote!(
        #[derive(Copy,Clone,Eq,PartialEq,Debug,Ord, PartialOrd)]
        pub enum DeviceIdentifier{
            #device_variants
        }
    )));
    let name_match = match_self(device_name_arms);
    bindings_content.push(Item::Impl(parse_quote!(
        impl DeviceIdentifier {
            pub fn name(self) -> &'static str {
                #name_match
            }
        }
    )));
    let encode_match = match_self(device_encode_arms);
    bindings_content.push(Item::Impl(parse_quote!(
        impl Into<u16> for DeviceIdentifier {
            fn into(self) -> u16 {
                #encode_match
            }
        }
    )));
    let parse_match = match_self(device_parse_arms);
    bindings_content.push(Item::Impl(parse_quote!(
        impl std::convert::TryInto<DeviceIdentifier> for u16 {
            type Error = ();
            fn try_into(self) -> Result<DeviceIdentifier, Self::Error> {
                #parse_match
            }
        }
    )));

    let file = File {
        shebang: None,
        attrs: vec![],
        items: bindings_content,
    };
    file
}

#[derive(Debug, Eq, PartialEq)]
pub enum TfValueType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    Bool,
    Char,
    String,
    Float,
}

impl ToTokens for TfValueType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(
            match self {
                TfValueType::U8 => {
                    quote!(u8)
                }
                TfValueType::U32 => {
                    quote!(u32)
                }
                TfValueType::U16 => {
                    quote!(u16)
                }
                TfValueType::Bool => {
                    quote!(bool)
                }
                TfValueType::Char => {
                    quote!(char)
                }
                TfValueType::String => {
                    quote!(Box<str>)
                }
                TfValueType::I8 => {
                    quote!(i8)
                }
                TfValueType::I16 => {
                    quote!(i16)
                }
                TfValueType::I32 => {
                    quote!(i32)
                }
                TfValueType::Float => {
                    quote!(f32)
                }
                TfValueType::U64 => {
                    quote!(u64)
                }
                TfValueType::I64 => {
                    quote!(i64)
                }
            }
                .into_token_stream(),
        )
    }
}

impl TfValueType {
    fn convert_value(&self, value: &JsonAnyDefaultValue) -> TokenStream {
        match (self, value) {
            (TfValueType::U8, JsonAnyDefaultValue::Integer(v)) => {
                let value: u8 = *v as u8;
                parse_quote!(#value)
            }
            (TfValueType::I8, JsonAnyDefaultValue::Integer(v)) => {
                let value: i8 = *v as i8;
                parse_quote!(#value)
            }
            (TfValueType::U16, JsonAnyDefaultValue::Integer(v)) => {
                let value: u16 = *v as u16;
                parse_quote!(#value)
            }
            (TfValueType::I16, JsonAnyDefaultValue::Integer(v)) => {
                let value: i16 = *v as i16;
                parse_quote!(#value)
            }
            (TfValueType::U32, JsonAnyDefaultValue::Integer(v)) => {
                let value: u32 = *v as u32;
                parse_quote!(#value)
            }
            (TfValueType::I32, JsonAnyDefaultValue::Integer(v)) => {
                let value: i32 = *v as i32;
                parse_quote!(#value)
            }
            (TfValueType::U64, JsonAnyDefaultValue::Integer(v)) => {
                let value: u64 = *v as u64;
                parse_quote!(#value)
            }
            (TfValueType::I64, JsonAnyDefaultValue::Integer(v)) => {
                let value: i64 = *v as i64;
                parse_quote!(#value)
            }
            (TfValueType::Bool, JsonAnyDefaultValue::Bool(b)) => {
                parse_quote!(#b)
            }
            (TfValueType::Char, JsonAnyDefaultValue::Character(c)) => {
                parse_quote!(#c)
            }
            (TfValueType::String, JsonAnyDefaultValue::String(v)) => {
                let string = v.as_ref();
                parse_quote!(#string)
            }
            (TfValueType::Float, JsonAnyDefaultValue::Float(v)) => {
                parse_quote!(#v)
            }

            _ => panic!("Invalid combination: Type {self:?}, value: {value:?}"),
        }
    }
    fn bytecount(&self, array_length: usize) -> usize {
        match self {
            TfValueType::U8 => array_length,
            TfValueType::I8 => array_length,
            TfValueType::U16 => array_length * 2,
            TfValueType::I16 => array_length * 2,
            TfValueType::U32 => array_length * 4,
            TfValueType::I32 => array_length * 4,
            TfValueType::Bool => (array_length + 7) / 8,
            TfValueType::Char => array_length,
            TfValueType::String => array_length,
            TfValueType::Float => array_length * 4,
            TfValueType::U64 => array_length * 8,
            TfValueType::I64 => array_length * 8,
        }
    }
}

fn generate_packet_element_item(
    items: &mut Vec<Item>,
    packet_description: &JsonPacketDescription,
    base_path: &Path,
    already_declared_constants: &mut HashSet<Box<str>>,
) -> ImplItemFn {
    let packet_name = packet_description.name.as_ref().to_case(Case::UpperCamel);
    let packet_type = &packet_description.r#type;
    let function_id = packet_description.function_id;
    let doc = &packet_description.doc;

    let doc_de = doc
        .text
        .0
        .get(&JsonLocale::De)
        .map(|v| v.as_ref())
        .unwrap_or_default();
    println!("Packet: {packet_name}");

    let mut fields = parse_packet_elements(
        packet_description,
        base_path,
        items,
        already_declared_constants,
    );
    match packet_type {
        JsonPacketType::Function => generate_element_function(
            items,
            packet_description,
            base_path,
            &packet_name,
            &mut fields,
        ),
        JsonPacketType::Callback => {
            let out_fields = &mut fields.out_fields;
            let function_name = create_ident(&format!(
                "{}_stream",
                packet_description.name.as_ref().to_case(Case::Snake)
            ));
            if out_fields.is_empty() {
                let function_block: Block = parse_quote!({self.device
                    .get_callback_receiver(#function_id)
                    .await
                    .map(|_| ())});
                parse_quote!(
                    #[doc = #doc_de]
                    pub async fn #function_name(&mut self) -> impl futures_core::Stream<Item = ()>
                        #function_block
                )
            } else if out_fields.len() == 1 {
                let first_field = out_fields.remove(0);
                let length = first_field.size();
                let first_field = first_field.field();
                let length_literal: Lit = parse_quote!(#length);
                let method_ident = parse_quote!(from_le_byte_slice);
                let args = parse_quote!((&p.body()[0..#length_literal]));
                let read_method_call = static_method_call(&first_field.ty, method_ident, args);
                let struct_name = first_field.ty.clone();
                let function_block: Block = parse_quote!(
                    {self.device
                            .get_callback_receiver(#function_id)
                            .await
                            .map(|p| #read_method_call)
                        }
                );
                parse_quote!(
                    #[doc = #doc_de]
                    pub async fn #function_name(&mut self) -> impl futures_core::Stream<Item = #struct_name>
                        #function_block
                )
            } else {
                let struct_name: Ident = create_ident(&format!("{packet_name}Callback"));
                append_data_object(items, out_fields, &struct_name);
                let function_block: Block = parse_quote!({
                           self.device
                            .get_callback_receiver(#function_id)
                            .await
                            .map(|p| #struct_name::from_le_byte_slice(p.body()))}
                );
                parse_quote!(
                    #[doc = #doc_de]
                    pub async fn #function_name(&mut self) -> impl futures_core::Stream<Item = #base_path::#struct_name>
                        #function_block
                )
            }
        }
    }
}

const LOW_LEVEL_SUFFIX: &str = " Low Level";

fn generate_element_function(
    items: &mut Vec<Item>,
    packet_description: &JsonPacketDescription,
    base_path: &Path,
    packet_name: &str,
    fields: &mut ParsedPacketFields,
) -> ImplItemFn {
    let function_id = packet_description.function_id;
    let doc = &packet_description.doc;
    let doc_de = doc
        .text
        .0
        .get(&JsonLocale::De)
        .map(|v| v.as_ref())
        .unwrap_or_default();

    let in_fields = &mut fields.in_fields;
    let out_fields = &mut fields.out_fields;

    let (request_type, request_size): (Option<Type>, usize) = if in_fields.is_empty() {
        (None, 0)
    } else if in_fields.len() == 1 {
        let first_field = in_fields.remove(0);
        let length = first_field.size();
        (Some(first_field.0.ty), length)
    } else {
        let struct_name: Ident = create_ident(&format!("{packet_name}Request"));
        let size = append_data_object(items, in_fields, &struct_name);
        if packet_description.level == JsonLevel::Low {
            if let Some(stripped_raw_name) = packet_description.name.strip_suffix(LOW_LEVEL_SUFFIX) {
                let stripped_struct_name = stripped_raw_name.to_case(Case::UpperCamel);
                let stripped_function_name = stripped_raw_name.to_case(Case::Snake);
                let mut struct_fields = Punctuated::<Field, Comma>::new();
                let mut chunk_offset_field = None;
                let mut stream_length_field = None;
                let mut data_field = None;
                let mut writer_statements = Vec::<Stmt>::new();
                writer_statements.push(parse_quote!(let mut i=0;));

                for field in in_fields.iter() {
                    match field.1.role {
                        None => {
                            let field_name = field.0.ident.clone().unwrap();
                            let increment = field.size();
                            writer_statements.push(parse_quote!(i+=(&self.request.#field_name).write_to_slice(&mut target[i..i+#increment]);));
                            struct_fields.push(field.0.clone());
                        }
                        Some(JsonRole::StreamChunkData) => {
                            writer_statements.push(
                                parse_quote!(i+=(&self.data).write_to_slice(&mut target[i..]);),
                            );
                            data_field = Some(field.1);
                        }
                        Some(JsonRole::StreamLength) => {
                            let increment = field.size();
                            writer_statements.push(parse_quote!(i+=(&self.length).write_to_slice(&mut target[i..i+#increment]);));
                            stream_length_field = Some(field.1);
                        }
                        Some(JsonRole::StreamData) => {}
                        Some(JsonRole::StreamChunkOffset) => {
                            let increment = field.size();
                            writer_statements.push(parse_quote!(i+=(&self.offset).write_to_slice(&mut target[i..i+#increment]);));
                            chunk_offset_field = Some(field.1);
                        }
                        Some(JsonRole::StreamChunkWritten) => {}
                        Some(JsonRole::StreamWritten) => {}
                    }
                }
                writer_statements.push(parse_quote!(return i;));
                if let (Some(data_element), Some(offset_element), Some(length_element)) =
                    (data_field, chunk_offset_field, stream_length_field)
                {
                    let max_chunk_size = data_element.cardinality as usize;
                    let element_type: TfValueType = data_element.r#type.into();
                    let offset_type: TfValueType = offset_element.r#type.into();
                    let length_type: TfValueType = length_element.r#type.into();
                    let high_level_struct_name: Ident =
                        create_ident(&format!("{stripped_struct_name}Request"));
                    let high_level_iterator_name: Ident =
                        create_ident(&format!("{stripped_struct_name}Iterator"));
                    let high_level_slice_name: Ident =
                        create_ident(&format!("{stripped_struct_name}Slice"));
                    //println!("Struct: {}", struct_fields.clone().into_token_stream());
                    if struct_fields.is_empty() {
                        items.push(parse_quote!(
                            #[derive(Copy, Clone, PartialEq, Debug)]
                            pub struct #high_level_struct_name<'d> {
                                pub data: &'d [#element_type],
                            }
                        ));
                    } else {
                        items.push(parse_quote!(
                            #[derive(Copy, Clone, PartialEq, Debug)]
                            pub struct #high_level_struct_name<'d> {
                                #struct_fields,
                                pub data: &'d [#element_type],
                            }
                        ));
                    }
                    items.push(parse_quote!(
                        pub struct #high_level_iterator_name<'r> {
                            request: &'r #high_level_struct_name<'r>,
                            offset: #offset_type,
                        }
                    ));
                    items.push(parse_quote!(
                        impl<'d> #high_level_struct_name<'d> {
                            pub fn write_to_slices(&'d self) -> #high_level_iterator_name<'d> {
                                #high_level_iterator_name {
                                    request: self,
                                    offset: 0,
                                }
                            }
                        }
                    ));
                    items.push(parse_quote!(
                        impl<'r> Iterator for #high_level_iterator_name<'r> {
                            type Item = #high_level_slice_name<'r>;

                            fn next(&mut self) -> Option<Self::Item> {
                                if self.offset as usize >= self.request.data.len() {
                                    None
                                } else {
                                    let slice_offset = self.offset;
                                    let length = self.request.data.len() as #length_type;
                                    let packet_length = #length_type::min(#max_chunk_size as #length_type, length - slice_offset);
                                    self.offset += packet_length;
                                    let data = &self.request.data[slice_offset as usize..slice_offset as usize + packet_length as usize];
                                    Some(#high_level_slice_name {
                                        request: self.request,
                                        offset: slice_offset,
                                        length: length,
                                        data,
                                    })
                                }
                            }
                        }
                    ));
                    items.push(parse_quote!(
                        pub struct #high_level_slice_name<'r> {
                            request: &'r #high_level_struct_name<'r>,
                            offset: #offset_type,
                            length: #length_type,
                            data: &'r [#element_type],
                        }
                    ));
                    let write_fields = Block {
                        brace_token: Default::default(),
                        stmts: writer_statements,
                    };
                    items.push(parse_quote!(
                         impl <'r> tinkerforge_base::byte_converter::ToBytes for #high_level_slice_name<'r> {
                            fn write_to_slice(&self, target: &mut [u8])->usize
                                #write_fields
                        }
                    ));
                    let function_name = create_ident(&stripped_function_name);
                    return parse_quote!(
                        #[doc = #doc_de]
                        pub async fn #function_name(&mut self, request:#high_level_struct_name<'_>) -> Result<(), tinkerforge_base::error::TinkerforgeError>{
                            let mut buffer = [0; 64];
                            for slice in request.write_to_slices() {
                                let length = slice.write_to_slice(&mut buffer);
                                let payload = &buffer[0..length];
                                self.device.set(#function_id, &payload,Some(std::time::Duration::from_secs(20))).await?;
                            }
                            Ok(())
                        }
                    );
                }
            }
            (Some(parse_quote!(#base_path::#struct_name)), size)
        } else {
            let struct_name: Ident = create_ident(&format!("{packet_name}Request"));
            (Some(parse_quote!(#base_path::#struct_name)), size)
        }
    };
    let (response_type, response_line): (Type, Option<Stmt>) = if out_fields.is_empty() {
        (parse_quote!(()), None)
    } else if out_fields.len() == 1 {
        let first_field = out_fields.remove(0);
        let length = first_field.size();
        let first_field = first_field.field();
        let length_literal: Lit = parse_quote!(#length);
        let method_ident = parse_quote!(from_le_byte_slice);
        let args = parse_quote!((&result.body()[0..#length_literal]));
        let read_method_call = static_method_call(&first_field.ty, method_ident, args);
        (
            first_field.ty.clone(),
            Some(Stmt::Expr(parse_quote!(Ok(#read_method_call)), None)),
        )
    } else {
        let name = format!("{packet_name}Response");
        let struct_name: Ident = create_ident(&name);
        append_data_object(items, &out_fields, &struct_name);
        (
            parse_quote!(#base_path::#struct_name),
            Some(Stmt::Expr(
                parse_quote!(Ok(#base_path::#struct_name::from_le_byte_slice(result.body()))),
                None,
            )),
        )
    };
    let function_name = create_ident(&packet_description.name.as_ref().to_case(Case::Snake));
    let mut function_statements = Vec::new();
    if request_type.is_some() {
        function_statements.push(parse_quote!(let mut payload = [0; #request_size];));
        function_statements.push(parse_quote!(tinkerforge_base::byte_converter::ToBytes::write_to_slice(&request,&mut payload);))
    } else {
        function_statements.push(parse_quote!(let payload = [0; #request_size];));
    }

    if let Some(response_line) = response_line {
        function_statements
            .push(parse_quote!(let result = self.device.get(#function_id, &payload).await?;));
        function_statements.push(response_line);
    } else {
        function_statements
            .push(parse_quote!(self.device.set(#function_id, &payload,Some(std::time::Duration::from_secs(20))).await?;));
        function_statements.push(Stmt::Expr(parse_quote!(Ok(())), None));
    }
    let function_block = Block {
        brace_token: Default::default(),
        stmts: function_statements,
    };
    if let Some(request_type) = request_type {
        parse_quote!(
            #[doc = #doc_de]
            pub async fn #function_name(&mut self, request: #request_type) -> Result<#response_type, tinkerforge_base::error::TinkerforgeError>
                #function_block
        )
    } else {
        parse_quote!(
            #[doc = #doc_de]
            pub async fn #function_name(&mut self) -> Result<#response_type, tinkerforge_base::error::TinkerforgeError>
                #function_block
        )
    }
}

struct ParsedPacketFields<'a> {
    in_fields: Vec<(Field, &'a JsonElement)>,
    out_fields: Vec<(Field, &'a JsonElement)>,
}

trait FieldWithSize {
    fn field(&self) -> &Field;
    fn size(&self) -> usize;
}

impl FieldWithSize for (Field, &JsonElement) {
    fn field(&self) -> &Field {
        &self.0
    }

    fn size(&self) -> usize {
        let element_entry = self.1;
        let transfer_type: TfValueType = element_entry.r#type.into();
        if element_entry.cardinality > 1
            && (transfer_type == TfValueType::String || element_entry.extra.len() == 1)
        {
            transfer_type.bytecount(element_entry.cardinality as usize)
        } else if element_entry.extra.len() as i32 == element_entry.cardinality {
            transfer_type.bytecount(1)
        } else {
            panic!(
                "Count mismatch {} != {}",
                element_entry.extra.len(),
                element_entry.cardinality
            );
        }
    }
}

fn parse_packet_elements<'a>(
    packet_entry: &'a JsonPacketDescription,
    base_path: &Path,
    constant_items: &mut Vec<Item>,
    already_declared_constants: &mut HashSet<Box<str>>,
) -> ParsedPacketFields<'a> {
    let mut in_fields = Vec::new();
    let mut out_fields = Vec::new();
    for element_entry in packet_entry.elements.iter() {
        let element_name = element_entry.name.as_ref();
        let element_name_rust = element_name.to_case(Case::Camel);
        let transfer_type: TfValueType = element_entry.r#type.into();
        if element_entry.cardinality < 1 {
            println!(
                "Skip negative cardinality on {}; {element_name}",
                base_path.into_token_stream().to_string()
            );
            continue;
        }
        let repeat_count = element_entry.cardinality as usize;
        let direction = element_entry.direction;
        let (fields, wrap_enum) = match direction {
            JsonDirection::IN => (&mut in_fields, true),
            JsonDirection::OUT => (&mut out_fields, false),
        };

        let ident = create_ident(&element_name_rust.to_case(Case::Snake));
        let create_fields: Box<[(Type, Ident)]> = if element_entry.cardinality > 1
            && transfer_type == TfValueType::String
        {
            vec![(parse_quote!([char;#repeat_count]), parse_quote!(#ident))].into()
        } else {
            let base_type = transfer_type.to_token_stream();
            let mut found_types: Vec<(Type, Ident)> = Vec::with_capacity(element_entry.extra.len());
            for extra_entry in element_entry.extra.iter() {
                let extra_entry_name = extra_entry.name.as_ref();
                let extra_ident = if extra_entry_name == element_name {
                    ident.clone()
                } else {
                    create_ident(
                        &format!("{element_name_rust} {extra_entry_name}").to_case(Case::Snake),
                    )
                };
                found_types.push(if let Some(constant_group) = &extra_entry.constant_group {
                    if already_declared_constants.insert(constant_group.name.clone()) {
                        process_constant_group(constant_items, element_entry, constant_group);
                    }
                    let constant_type_name = Some(create_ident(&constant_group.name.as_ref().to_case(Case::UpperCamel)));
                    (
                        if wrap_enum {
                            parse_quote!(#base_path::#constant_type_name)
                        } else {
                            parse_quote!(tinkerforge_base::byte_converter::ParsedOrRaw<#base_path::#constant_type_name,#transfer_type>)
                        },
                        parse_quote!(#extra_ident)
                    )
                } else {
                    (parse_quote!(#base_type), parse_quote!( #extra_ident))
                });
            }
            if found_types.len() == 1 && element_entry.cardinality > 1 {
                if let [(base_type, ident)] = &found_types[..] {
                    vec![(parse_quote!([#base_type;#repeat_count]), ident.clone())].into()
                } else {
                    panic!("Invalid");
                }
            } else if found_types.len() == repeat_count {
                found_types.into_boxed_slice()
            } else {
                panic!(
                    "Count mismatch {} != {}",
                    found_types.len(),
                    element_entry.cardinality
                );
            }
        };
        for (ty, ident) in create_fields.iter().cloned() {
            fields.push((
                Field {
                    attrs: vec![],
                    vis: Visibility::Public(Pub::default()),
                    mutability: FieldMutability::None,
                    ident: Some(ident),
                    colon_token: None,
                    ty,
                },
                element_entry,
            ));
        }
    }
    ParsedPacketFields {
        in_fields,
        out_fields,
    }
}

fn process_constant_group(items: &mut Vec<Item>, element: &JsonElement, group: &JsonConstantGroup) {
    let camel_name = group.name.as_ref().to_case(Case::UpperCamel);
    println!("Constant group: {}", group.name);

    let ty: TfValueType = element.r#type.into();
    let enum_name_ident = create_ident(&camel_name);
    let mut variants: Punctuated<Variant, Comma> = Default::default();
    let mut encode_arms = vec![];
    let mut parse_arms = vec![];
    for constant_entry in group.constants.iter() {
        let name = constant_entry.name.as_ref();
        let value = &constant_entry.value;
        let variant_ident = create_ident(&name.to_case(Case::UpperCamel));
        variants.push(parse_quote!(#variant_ident));
        let value = ty.convert_value(value);
        encode_arms.push(parse_quote!(#enum_name_ident::#variant_ident =>#value));
        parse_arms.push(parse_quote!(#value => Ok(#enum_name_ident::#variant_ident)))
    }
    items.push(parse_quote!(
        #[derive(Copy,Clone,Eq,PartialEq,Debug)]
        pub enum #enum_name_ident{
            #variants
        }
    ));
    let encode_match = ExprMatch {
        attrs: vec![],
        match_token: Default::default(),
        expr: Box::new(parse_quote!(self)),
        brace_token: Default::default(),
        arms: encode_arms,
    };

    parse_arms.push(parse_quote!(_ => Err(())));
    let parse_match = ExprMatch {
        attrs: vec![],
        match_token: Default::default(),
        expr: Box::new(parse_quote!(self)),
        brace_token: Default::default(),
        arms: parse_arms,
    };

    items.push(Item::Impl(parse_quote!(
        impl Into<#ty> for #enum_name_ident {
            fn into(self) -> #ty {
                #encode_match
            }
        }
    )));
    items.push(parse_quote!(
        impl tinkerforge_base::byte_converter::ToBytes for #enum_name_ident {
            fn write_to_slice(&self,target: &mut [u8])->usize{
                <#enum_name_ident as Into<#ty>>::into(*self).write_to_slice(target)
            }
        }
    ));
    let type_size = ty.bytecount(1);
    items.push(parse_quote!(
        impl tinkerforge_base::byte_converter::FromByteSlice for #enum_name_ident {
            fn from_le_byte_slice(bytes: &[u8])->Self{
                #ty::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
            }
            fn bytes_expected() -> usize{
                #type_size
            }
        }
    ));

    items.push(Item::Impl(parse_quote!(
        impl std::convert::TryInto<#enum_name_ident> for #ty {
            type Error = ();
            fn try_into(self) -> Result<#enum_name_ident, Self::Error> {
                #parse_match
            }
        }
    )));
}

fn create_ident(string: &str) -> Ident {
    if if string == "type" {
        true
    } else if let Some(first_char) = string.chars().next() {
        !first_char.is_alphabetic()
    } else {
        false
    } {
        Ident::new(&format!("_{string}"), Span::call_site())
    } else {
        Ident::new(string, Span::call_site())
    }
}

fn match_self(arms: Vec<Arm>) -> ExprMatch {
    ExprMatch {
        attrs: vec![],
        match_token: Default::default(),
        expr: Box::new(parse_quote!(self)),
        brace_token: Default::default(),
        arms,
    }
}

fn append_data_object<F: FieldWithSize>(
    items: &mut Vec<Item>,
    fields: &[F],
    struct_name: &Ident,
) -> usize {
    let mut reader_statements = Vec::<Stmt>::new();
    let mut writer_statements = Vec::<Stmt>::new();
    let mut initialization_fields = Punctuated::<FieldValue, Comma>::new();
    let mut offset = 0;
    let mut struct_fields = Punctuated::<Field, Comma>::new();
    for field in fields.iter() {
        let size = field.size();
        let field = field.field();
        if let Some(field_name) = &field.ident {
            let offset_before: Lit = parse_quote!(#offset);
            offset += size;
            let offset_after: Lit = parse_quote!(#offset);
            let read_method_call = static_method_call(
                &field.ty,
                parse_quote!(from_le_byte_slice),
                parse_quote!((&bytes[#offset_before..#offset_after])),
            );
            reader_statements.push(parse_quote!(let #field_name = #read_method_call;));
            initialization_fields.push(parse_quote!(#field_name));
            writer_statements.push(parse_quote!((&self.#field_name).write_to_slice(&mut target[#offset_before..#offset_after]);));
            struct_fields.push(field.clone());
        }
    }
    let total_size: Lit = parse_quote!(#offset);
    items.push(parse_quote!(
        #[derive(Copy, Clone, PartialEq, Debug)]
        pub struct #struct_name {
            #struct_fields
        }

    ));

    reader_statements.push(Stmt::Expr(parse_quote!(Self{#initialization_fields}), None));
    let read_fields = Block {
        brace_token: Default::default(),
        stmts: reader_statements,
    };
    items.push(parse_quote!(
       impl tinkerforge_base::byte_converter::FromByteSlice for #struct_name {
       fn from_le_byte_slice(bytes: &[u8]) -> Self
               #read_fields
       fn bytes_expected() -> usize {
         #total_size
       }
    }));
    writer_statements.push(parse_quote!(return #total_size;));
    let write_fields = Block {
        brace_token: Default::default(),
        stmts: writer_statements,
    };
    items.push(parse_quote!(
         impl tinkerforge_base::byte_converter::ToBytes for #struct_name {
            fn write_to_slice(&self, target: &mut [u8])->usize
                #write_fields
        }
    ));
    offset
}

fn static_method_call(ty: &Type, method: Ident, args: Punctuated<Expr, Comma>) -> Expr {
    if let Type::Path(TypePath {
                          qself: None,
                          path: Path {
                              leading_colon: _,
                              segments,
                          },
                      }) = &ty
    {
        let seg = segments.last();
        if let Some(PathSegment {
                        ident,
                        arguments: PathArguments::AngleBracketed(bracketed),
                    }) = seg
        {
            return if segments.len() > 1 {
                let mut type_path: Punctuated<PathSegment, PathSep> = Punctuated::new();
                for segment in segments.iter().take(segments.len() - 1) {
                    type_path.push(segment.clone());
                }
                type_path.push(parse_quote!(#ident));
                parse_quote!(#type_path::#bracketed::#method #args)
            } else {
                parse_quote!(#ident::#bracketed::#method #args)
            };
        }
    }
    if let Type::Array(_) = ty {
        parse_quote!(<#ty>::#method #args)
    } else {
        parse_quote!(#ty::#method #args)
    }
}
