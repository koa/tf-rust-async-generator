//! Traits for (de)serialization of structs to byte vectors.
use std::{
    convert::TryInto,
    fmt::{Debug, Formatter},
};

use byteorder::{ByteOrder, LittleEndian};

/// A trait to serialize the implementing type to a byte vector.
pub trait ToBytes {
    fn write_to_slice(&self, target: &mut [u8]) -> usize;
}

/// A trait to deserialize the implemeting type from a byte slice.
pub trait FromByteSlice {
    /// Deserialize the implementing type from a byte slice.
    fn from_le_byte_slice(bytes: &[u8]) -> Self;
    /// Returns how many bytes are expected to deserialize a instance of the implementing type. Currently this method is only used for strings.
    fn bytes_expected() -> usize;
}

impl<const N: usize> FromByteSlice for Box<[u8; N]> {
    fn from_le_byte_slice(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), Self::bytes_expected());
        let mut ret = Box::new([0; N]);
        ret.clone_from_slice(bytes);
        ret
    }

    fn bytes_expected() -> usize {
        N
    }
}

impl<const N: usize, T: Default + Copy + FromByteSlice> FromByteSlice for [T; N] {
    fn from_le_byte_slice(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), Self::bytes_expected());
        let mut ret = [T::default(); N];
        let component_size = T::bytes_expected();
        for i in 0..N {
            ret[i] = T::from_le_byte_slice(&bytes[i * component_size..(i + 1) * component_size]);
        }
        ret
    }

    fn bytes_expected() -> usize {
        T::bytes_expected() * N
    }
}

impl<const N: usize> ToBytes for [u8; N] {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        write_bytes_to_target(self, target);
        N
    }
}

impl ToBytes for [u8] {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        target.copy_from_slice(self);
        self.len()
    }
}

impl<const N: usize> ToBytes for [i8; N] {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        write_bytes_to_target(self, target);
        N
    }
}

impl<const N: usize> ToBytes for [u16; N] {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        write_bytes_to_target(self, target);
        2 * N
    }
}

impl ToBytes for [u16] {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        for (idx, value) in self.iter().enumerate() {
            value.write_to_slice(&mut target[idx * 2..idx * 2 + 2]);
        }
        2 * self.len()
    }
}

impl<const N: usize> ToBytes for [i16; N] {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        write_bytes_to_target(self, target);
        2 * N
    }
}

impl<const N: usize> ToBytes for [u32; N] {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        write_bytes_to_target(self, target);
        4 * N
    }
}

impl<const N: usize> ToBytes for [i32; N] {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        write_bytes_to_target(self, target);
        4 * N
    }
}

impl<const N: usize> ToBytes for [u64; N] {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        write_bytes_to_target(self, target);
        8 * N
    }
}

impl<const N: usize> ToBytes for [i64; N] {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        write_bytes_to_target(self, target);
        8 * N
    }
}

impl<const N: usize> ToBytes for [char; N] {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        write_bytes_to_target(self, target);
        N
    }
}

impl ToBytes for [char] {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        for (idx, char) in self.iter().enumerate() {
            *(target.get_mut(idx).expect("slice too small")) = *char as u8;
        }
        self.len()
    }
}

impl<const N: usize> ToBytes for [f32; N] {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        write_bytes_to_target(self, target);
        4 * N
    }
}

#[inline]
fn write_bytes_to_target<T: Default + Copy + FromByteSlice + ToBytes, const N: usize>(
    value: &[T; N],
    target: &mut [u8],
) {
    let component_size = T::bytes_expected();
    for i in 0..N {
        value[i].write_to_slice(&mut target[i * component_size..(i + 1) * component_size]);
    }
}

impl<const N: usize> ToBytes for [bool; N] {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        let bytecount = (N + 7) / 8;
        for value in target.iter_mut().take(bytecount) {
            *value = 0;
        }
        for (i, b) in self.iter().enumerate() {
            target[i / 8] |= (*b as u8) << (i % 8);
        }
        bytecount
    }
}

impl ToBytes for &[bool] {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        let bytecount = (self.len() + 7) / 8;
        for value in target.iter_mut().take(bytecount) {
            *value = 0;
        }
        for (i, b) in self.iter().enumerate() {
            target[i / 8] |= (*b as u8) << (i % 8);
        }
        bytecount
    }
}

impl ToBytes for () {
    fn write_to_slice(&self, _target: &mut [u8]) -> usize {
        0
    }
}

impl FromByteSlice for () {
    fn from_le_byte_slice(_: &[u8]) {}

    fn bytes_expected() -> usize {
        0
    }
}

impl ToBytes for bool {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        *(target.get_mut(0).expect("slice too small")) = *self as u8;
        1
    }
}

impl FromByteSlice for bool {
    fn from_le_byte_slice(bytes: &[u8]) -> bool {
        bytes[0] != 0
    }

    fn bytes_expected() -> usize {
        1
    }
}

impl ToBytes for u8 {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        *(target.get_mut(0).expect("slice too small")) = *self;
        1
    }
}

impl FromByteSlice for u8 {
    fn from_le_byte_slice(bytes: &[u8]) -> u8 {
        bytes[0]
    }

    fn bytes_expected() -> usize {
        1
    }
}

impl ToBytes for i8 {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        *(target.get_mut(0).expect("slice too small")) = *self as u8;
        1
    }
}

impl FromByteSlice for i8 {
    fn from_le_byte_slice(bytes: &[u8]) -> i8 {
        bytes[0] as i8
    }

    fn bytes_expected() -> usize {
        1
    }
}

impl ToBytes for u16 {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        LittleEndian::write_u16(target, *self);
        2
    }
}

impl FromByteSlice for u16 {
    fn from_le_byte_slice(bytes: &[u8]) -> u16 {
        LittleEndian::read_u16(bytes)
    }

    fn bytes_expected() -> usize {
        2
    }
}

impl ToBytes for i16 {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        LittleEndian::write_i16(target, *self);
        2
    }
}

impl FromByteSlice for i16 {
    fn from_le_byte_slice(bytes: &[u8]) -> i16 {
        LittleEndian::read_i16(bytes)
    }

    fn bytes_expected() -> usize {
        2
    }
}

impl ToBytes for u32 {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        LittleEndian::write_u32(target, *self);
        4
    }
}

impl FromByteSlice for u32 {
    fn from_le_byte_slice(bytes: &[u8]) -> u32 {
        LittleEndian::read_u32(bytes)
    }

    fn bytes_expected() -> usize {
        4
    }
}

impl ToBytes for i32 {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        LittleEndian::write_i32(target, *self);
        4
    }
}

impl FromByteSlice for i32 {
    fn from_le_byte_slice(bytes: &[u8]) -> i32 {
        LittleEndian::read_i32(bytes)
    }

    fn bytes_expected() -> usize {
        4
    }
}

impl ToBytes for u64 {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        LittleEndian::write_u64(target, *self);
        8
    }
}

impl FromByteSlice for u64 {
    fn from_le_byte_slice(bytes: &[u8]) -> u64 {
        LittleEndian::read_u64(bytes)
    }

    fn bytes_expected() -> usize {
        8
    }
}

impl ToBytes for i64 {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        LittleEndian::write_i64(target, *self);
        8
    }
}

impl FromByteSlice for i64 {
    fn from_le_byte_slice(bytes: &[u8]) -> i64 {
        LittleEndian::read_i64(bytes)
    }

    fn bytes_expected() -> usize {
        8
    }
}

impl ToBytes for char {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        *(target.get_mut(0).expect("slice too small")) = *self as u8;
        1
    }
}

impl FromByteSlice for char {
    fn from_le_byte_slice(bytes: &[u8]) -> char {
        bytes[0] as char
    }

    fn bytes_expected() -> usize {
        1
    }
}

impl ToBytes for String {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        let bytes = self.clone().into_bytes();
        target.copy_from_slice(&bytes);
        bytes.len()
    }
}

impl ToBytes for f32 {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        LittleEndian::write_f32(target, *self);
        4
    }
}

impl FromByteSlice for f32 {
    fn from_le_byte_slice(bytes: &[u8]) -> f32 {
        LittleEndian::read_f32(bytes)
    }

    fn bytes_expected() -> usize {
        4
    }
}

impl ToBytes for f64 {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        LittleEndian::write_f64(target, *self);
        8
    }
}

impl FromByteSlice for f64 {
    fn from_le_byte_slice(bytes: &[u8]) -> f64 {
        LittleEndian::read_f64(bytes)
    }

    fn bytes_expected() -> usize {
        8
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum ParsedOrRaw<P, R>
    where
        P: Into<R> + Debug + Clone + Copy,
        R: TryInto<P> + FromByteSlice + ToBytes + Debug + Clone + Copy,
{
    Parsed(P),
    Raw(R),
}

impl<P, R> ParsedOrRaw<P, R>
    where
        P: Into<R> + Debug + Clone + Copy,
        R: TryInto<P> + FromByteSlice + ToBytes + Debug + Clone + Copy,
{
    pub fn parsed(&self) -> Option<P> {
        match self {
            ParsedOrRaw::Parsed(v) => Some(*v),
            ParsedOrRaw::Raw(_) => None,
        }
    }
    pub fn raw(&self) -> R {
        match self {
            ParsedOrRaw::Parsed(p) => (*p).into(),
            ParsedOrRaw::Raw(v) => *v,
        }
    }
}

impl<P, R> Debug for ParsedOrRaw<P, R>
    where
        P: Into<R> + Debug + Clone + Copy,
        R: TryInto<P> + FromByteSlice + ToBytes + Debug + Clone + Copy,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsedOrRaw::Parsed(value) => {
                write!(f, "{value:?}({:?})", (*value).into())
            }
            ParsedOrRaw::Raw(raw_value) => {
                write!(f, "<unknown>({:?})", raw_value)
            }
        }
    }
}

impl<P, R> From<R> for ParsedOrRaw<P, R>
    where
        P: Into<R> + Debug + Clone + Copy,
        R: TryInto<P> + FromByteSlice + ToBytes + Debug + Clone + Copy,
{
    fn from(value: R) -> Self {
        if let Ok(parsed) = value.try_into() {
            ParsedOrRaw::Parsed(parsed)
        } else {
            ParsedOrRaw::Raw(value)
        }
    }
}

impl<P, R> ToBytes for ParsedOrRaw<P, R>
    where
        P: Into<R> + Debug + Clone + Copy,
        R: TryInto<P> + FromByteSlice + ToBytes + Debug + Clone + Copy,
{
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        let raw_value = match self {
            ParsedOrRaw::Parsed(v) => (*v).into(),
            ParsedOrRaw::Raw(v) => *v,
        };
        raw_value.write_to_slice(target)
    }
}

impl<P, R> FromByteSlice for ParsedOrRaw<P, R>
    where
        P: Into<R> + Debug + Clone + Copy,
        R: TryInto<P> + FromByteSlice + ToBytes + Debug + Clone + Copy,
{
    fn from_le_byte_slice(bytes: &[u8]) -> Self {
        let raw_value = R::from_le_byte_slice(bytes);
        if let Ok(value) = raw_value.try_into() {
            Self::Parsed(value)
        } else {
            Self::Raw(raw_value)
        }
    }

    fn bytes_expected() -> usize {
        R::bytes_expected()
    }
}

impl<P, R> Default for ParsedOrRaw<P, R>
    where
        P: Into<R> + Debug + Clone + Copy,
        R: TryInto<P> + FromByteSlice + ToBytes + Debug + Clone + Copy + Default,
{
    fn default() -> Self {
        Self::from(R::default())
    }
}
