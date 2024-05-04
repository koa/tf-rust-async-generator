use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt, ToTokens};
use serde::{Deserialize, Serialize};
use syn::parse_quote;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonContent {
    pub author: Box<str>,
    pub api_version: JsonApiVersion,
    pub category: JsonCategory,
    pub device_identifier: i16,
    pub name: Box<str>,
    pub display_name: JsonDisplayName,
    pub manufacturer: Box<str>,
    pub description: JsonLocalizedText,
    pub released: bool,
    pub documented: bool,
    pub doc: JsonLocalizedText,
    pub packets: Box<[JsonPacketDescription]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonApiVersion([u8; 3]);

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum JsonCategory {
    Brick,
    Bricklet,
    TNG,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonDisplayName {
    pub short: Box<str>,
    pub long: Box<str>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonLocalizedText(pub HashMap<JsonLocale, Box<str>>);

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum JsonLocale {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "de")]
    De,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonPacketDescription {
    pub level: JsonLevel,
    pub r#type: JsonPacketType,
    pub name: Box<str>,
    pub function_id: u8,
    pub since_firmware: JsonApiVersion,
    pub doc: JsonDoc,
    pub elements: Box<[JsonElement]>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JsonLevel {
    Normal,
    Low,
    High,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JsonPacketType {
    Function,
    Callback,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonDoc {
    pub r#type: JsonDocType,
    pub text: JsonLocalizedText,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum JsonDocType {
    BF,
    AF,
    CCF,
    C,
    IF,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonElement {
    pub level: JsonLevel,
    pub name: Box<str>,
    pub r#type: JsonElementType,
    pub cardinality: i32,
    pub direction: JsonDirection,
    pub role: Option<JsonRole>,
    pub extra: Box<[JsonElementExtra]>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JsonElementType {
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float,
    Bool,
    Char,
    String,
}

impl JsonElementType {
    pub fn convert_value(&self, value: &JsonAnyDefaultValue) -> TokenStream {
        match (self, value) {
            (JsonElementType::UInt8, JsonAnyDefaultValue::Integer(v)) => {
                let value: u8 = *v as u8;
                parse_quote!(#value)
            }
            (JsonElementType::Int8, JsonAnyDefaultValue::Integer(v)) => {
                let value: i8 = *v as i8;
                parse_quote!(#value)
            }
            (JsonElementType::UInt16, JsonAnyDefaultValue::Integer(v)) => {
                let value: u16 = *v as u16;
                parse_quote!(#value)
            }
            (JsonElementType::Int16, JsonAnyDefaultValue::Integer(v)) => {
                let value: i16 = *v as i16;
                parse_quote!(#value)
            }
            (JsonElementType::UInt32, JsonAnyDefaultValue::Integer(v)) => {
                let value: u32 = *v as u32;
                parse_quote!(#value)
            }
            (JsonElementType::Int32, JsonAnyDefaultValue::Integer(v)) => {
                let value: i32 = *v as i32;
                parse_quote!(#value)
            }
            (JsonElementType::UInt64, JsonAnyDefaultValue::Integer(v)) => {
                let value: u64 = *v as u64;
                parse_quote!(#value)
            }
            (JsonElementType::Int64, JsonAnyDefaultValue::Integer(v)) => {
                let value: i64 = *v;
                parse_quote!(#value)
            }
            (JsonElementType::Bool, JsonAnyDefaultValue::Bool(b)) => {
                parse_quote!(#b)
            }
            (JsonElementType::Char, JsonAnyDefaultValue::Character(c)) => {
                parse_quote!(#c)
            }
            (JsonElementType::String, JsonAnyDefaultValue::String(v)) => {
                let string = v.as_ref();
                parse_quote!(#string)
            }
            (JsonElementType::Float, JsonAnyDefaultValue::Float(v)) => {
                parse_quote!(#v)
            }

            _ => panic!("Invalid combination: Type {self:?}, value: {value:?}"),
        }
    }
    pub fn bytecount(&self, array_length: usize) -> usize {
        match self {
            JsonElementType::UInt8 => array_length,
            JsonElementType::Int8 => array_length,
            JsonElementType::UInt16 => array_length * 2,
            JsonElementType::Int16 => array_length * 2,
            JsonElementType::UInt32 => array_length * 4,
            JsonElementType::Int32 => array_length * 4,
            JsonElementType::Bool => (array_length + 7) / 8,
            JsonElementType::Char => array_length,
            JsonElementType::String => array_length,
            JsonElementType::Float => array_length * 4,
            JsonElementType::UInt64 => array_length * 8,
            JsonElementType::Int64 => array_length * 8,
        }
    }
}

impl ToTokens for JsonElementType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(
            match self {
                JsonElementType::UInt8 => {
                    quote!(u8)
                }
                JsonElementType::UInt32 => {
                    quote!(u32)
                }
                JsonElementType::UInt16 => {
                    quote!(u16)
                }
                JsonElementType::Bool => {
                    quote!(bool)
                }
                JsonElementType::Char => {
                    quote!(char)
                }
                JsonElementType::String => {
                    quote!(Box<str>)
                }
                JsonElementType::Int8 => {
                    quote!(i8)
                }
                JsonElementType::Int16 => {
                    quote!(i16)
                }
                JsonElementType::Int32 => {
                    quote!(i32)
                }
                JsonElementType::Float => {
                    quote!(f32)
                }
                JsonElementType::UInt64 => {
                    quote!(u64)
                }
                JsonElementType::Int64 => {
                    quote!(i64)
                }
            }
                .into_token_stream(),
        )
    }
}


#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JsonDirection {
    IN,
    OUT,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum JsonRole {
    StreamLength,
    StreamChunkData,
    StreamData,
    StreamChunkOffset,
    StreamChunkWritten,
    StreamWritten,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonElementExtra {
    pub index: Option<u8>,
    pub name: Box<str>,
    pub scale: JsonScale,
    pub unit: Option<JsonUnit>,
    pub range: Option<JsonRange>,
    pub default: Option<JsonAnyDefaultValue>,
    pub constant_group: Option<JsonConstantGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum JsonScale {
    Fraction { numerator: u32, denominator: u32 },
    Named(JsonScaleName),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum JsonScaleName {
    Dynamic,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonUnit {
    pub title: JsonLocalizedText,
    pub symbol: Box<str>,
    pub usage: JsonLocalizedText,
    pub sequence: JsonLocalizedText,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum JsonRange {
    Entries(Box<[JsonMinMaxEntry]>),
    Named(JsonRangeName),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum JsonMinMaxEntry {
    Integer { minimum: i32, maximum: i32 },
    Character { minimum: char, maximum: char },
    Float { minimum: f32, maximum: f32 },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum JsonAnyDefaultValue {
    Integer(i64),
    Character(char),
    Float(f32),
    Bool(bool),
    String(Box<str>),
    Array(Box<[JsonAnyDefaultValue]>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum JsonRangeName {
    Dynamic,
    Type,
    Constants,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonConstantGroup {
    pub name: Box<str>,
    pub constants: Box<[JsonConstantEntry]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonConstantEntry {
    pub name: Box<str>,
    pub value: JsonAnyDefaultValue,
}
