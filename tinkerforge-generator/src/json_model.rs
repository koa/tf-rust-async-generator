use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::generator::TfValueType;

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

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
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

impl Into<TfValueType> for JsonElementType {
    fn into(self) -> TfValueType {
        match self {
            JsonElementType::Int8 => TfValueType::I8,
            JsonElementType::Int16 => TfValueType::I16,
            JsonElementType::Int32 => TfValueType::I32,
            JsonElementType::Int64 => TfValueType::I64,
            JsonElementType::UInt8 => TfValueType::U8,
            JsonElementType::UInt16 => TfValueType::U16,
            JsonElementType::UInt32 => TfValueType::U32,
            JsonElementType::UInt64 => TfValueType::U64,
            JsonElementType::Float => TfValueType::Float,
            JsonElementType::Bool => TfValueType::Bool,
            JsonElementType::Char => TfValueType::Char,
            JsonElementType::String => TfValueType::String,
        }
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
