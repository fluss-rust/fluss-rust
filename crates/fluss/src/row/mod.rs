use crate::row::datum::Datum;

mod column;

mod datum;

pub use column::*;

pub trait InternalRow {
    /// Returns the number of fields in this row
    fn get_field_count(&self) -> usize;

    /// Returns true if the element is null at the given position
    fn is_null_at(&self, pos: usize) -> bool;

    /// Returns the boolean value at the given position
    fn get_boolean(&self, pos: usize) -> bool;

    /// Returns the byte value at the given position
    fn get_byte(&self, pos: usize) -> i8;

    /// Returns the short value at the given position
    fn get_short(&self, pos: usize) -> i16;

    /// Returns the integer value at the given position
    fn get_int(&self, pos: usize) -> i32;

    /// Returns the long value at the given position
    fn get_long(&self, pos: usize) -> i64;

    /// Returns the float value at the given position
    fn get_float(&self, pos: usize) -> f32;

    /// Returns the double value at the given position
    fn get_double(&self, pos: usize) -> f64;

    /// Returns the string value at the given position with fixed length
    fn get_char(&self, pos: usize, length: usize) -> String;

    /// Returns the string value at the given position
    fn get_string(&self, pos: usize) -> &str;

    // /// Returns the decimal value at the given position
    // fn get_decimal(&self, pos: usize, precision: usize, scale: usize) -> Decimal;

    // /// Returns the timestamp value at the given position
    // fn get_timestamp_ntz(&self, pos: usize, precision: usize) -> TimestampNtz;

    // /// Returns the timestamp value at the given position
    // fn get_timestamp_ltz(&self, pos: usize, precision: usize) -> TimestampLtz;

    /// Returns the binary value at the given position with fixed length
    fn get_binary(&self, pos: usize, length: usize) -> Vec<u8>;

    /// Returns the binary value at the given position
    fn get_bytes(&self, pos: usize) -> Vec<u8>;
}


pub struct GenericRow<'a> {
    pub values: Vec<Datum<'a>>,
}

impl<'a> InternalRow for GenericRow<'a> {
    fn get_field_count(&self) -> usize {
        self.values.len()
    }

    fn is_null_at(&self, pos: usize) -> bool {
        false
    }

    fn get_boolean(&self, pos: usize) -> bool {
        todo!()
    }

    fn get_byte(&self, pos: usize) -> i8 {
        todo!()
    }

    fn get_short(&self, pos: usize) -> i16 {
        todo!()
    }

    fn get_int(&self, pos: usize) -> i32 {
        self.values.get(pos).unwrap().try_into().unwrap()
    }

    fn get_long(&self, pos: usize) -> i64 {
        todo!()
    }

    fn get_float(&self, pos: usize) -> f32 {
        todo!()
    }

    fn get_double(&self, pos: usize) -> f64 {
        todo!()
    }

    fn get_char(&self, pos: usize, length: usize) -> String {
        todo!()
    }

    fn get_string(&self, pos: usize) -> &str {
        self.values.get(pos).unwrap().try_into().unwrap()
    }

    fn get_binary(&self, pos: usize, length: usize) -> Vec<u8> {
        todo!()
    }

    fn get_bytes(&self, pos: usize) -> Vec<u8> {
        todo!()
    }
}

impl<'a> Default for GenericRow<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> GenericRow<'a> {
    pub fn new() -> GenericRow<'a> {
        GenericRow { values: vec![] }
    }

    pub fn set_field(&mut self, pos: usize, value: impl Into<Datum<'a>>) {
        self.values.insert(pos, value.into());
    }
}