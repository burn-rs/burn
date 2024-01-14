use std::collections::HashMap;

use super::{
    error::{self, Error},
    reader::NestedValue,
};

use serde::{
    ser::{self, SerializeSeq, SerializeStruct, Serializer as SerializerTrait},
    Serialize,
};

// Simple struct serializer that converts a struct into NestedValues
pub(crate) struct Serializer {
    // This will hold the state of the serialization process
    state: Option<NestedValue>,
}

impl Serializer {
    pub fn new() -> Self {
        Serializer { state: None }
    }
}

impl SerializerTrait for Serializer {
    type Ok = NestedValue;
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = ser::Impossible<NestedValue, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<NestedValue, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<NestedValue, Self::Error>;
    type SerializeMap = ser::Impossible<NestedValue, Self::Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = ser::Impossible<NestedValue, Self::Error>;

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(NestedValue::I32(v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(NestedValue::String(v.to_string()))
    }

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(NestedValue::I16(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(NestedValue::I64(v))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(NestedValue::U16(v))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(NestedValue::U64(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(NestedValue::F32(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(NestedValue::F64(v))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!("serialize_unit not implemented for Serializer")
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!("serialize_unit_struct not implemented for Serializer")
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unimplemented!("serialize_unit_variant not implemented for Serializer")
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("serialize_newtype_variant not implemented for Serializer")
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!("serialize_tuple not implemented for Serializer")
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!("serialize_tuple_struct not implemented for Serializer")
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!("serialize_tuple_variant not implemented for Serializer")
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!("serialize_map not implemented for Serializer")
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }
}

// Implementing the SerializeStruct trait for Serializer
impl SerializeStruct for Serializer {
    type Ok = NestedValue;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let serialized_value = value.serialize(Serializer::new())?;

        match self.state {
            Some(NestedValue::Map(ref mut map)) => {
                map.insert(key.to_string(), serialized_value); // Inserting into the state
            }
            Some(_) => {
                panic!("Invalid state encountered");
            }
            None => {
                let mut map = HashMap::new();
                map.insert(key.to_string(), serialized_value); // Inserting into the state
                self.state = Some(NestedValue::Map(map));
            }
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.state.is_none() {
            // If the state is empty, return an empty map
            Ok(NestedValue::Map(HashMap::new()))
        } else {
            self.state.ok_or(error::Error::InvalidState)
        }
    }
}

impl SerializeSeq for Serializer {
    type Ok = NestedValue;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let serialized_value = value.serialize(Serializer::new())?;

        match self.state {
            Some(NestedValue::Vec(ref mut vec)) => {
                vec.push(serialized_value); // Inserting into the state
            }
            Some(_) => {
                panic!("Invalid state encountered");
            }
            None => {
                self.state = Some(NestedValue::Vec(vec![serialized_value]));
            }
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.state.is_none() {
            // If the state is empty, return an empty vector
            Ok(NestedValue::Vec(Vec::new()))
        } else {
            self.state.ok_or(error::Error::InvalidState)
        }
    }
}

#[cfg(test)]
mod tests {
    use burn::{
        module::{Param, ParamId},
        record::{FullPrecisionSettings, Record},
        tensor::Tensor,
    };
    use serde::Deserialize;

    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct MyStruct1 {
        a: MyStruct3,
        b: MyStruct2,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct MyStruct2 {
        a: i32,
        b: Option<i32>,
        c: String,
        d: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct MyStruct3 {
        x: String,
        y: String,
    }

    #[test]
    fn test_serialize() {
        let my_struct = MyStruct1 {
            a: MyStruct3 {
                x: "Hello".to_owned(),
                y: "World".to_owned(),
            },
            b: MyStruct2 {
                a: 1,
                b: None,
                c: "Hello".to_owned(),
                d: Some("World".to_owned()),
            },
        };

        let serialized = my_struct.serialize(Serializer::new()).unwrap();

        let serialized_str = format!("{:?}", serialized);

        // Compare the lengths of expected and actual serialized strings because
        // the order of the fields is not guaranteed for HashMaps.
        assert_eq!(
            serialized_str.len(),
            concat!(
            r#"Map({"b": Map({"a": Default, "c": String("Hello"), "b": Default, "d": String("World")}),"#,
            r#" "a": Map({"x": String("Hello"), "y": String("World")})})"#
            ).len()
        );
    }

    #[test]
    fn test_param_serde() {
        type Backend = burn_ndarray::NdArray<f32>;

        let tensor: Tensor<Backend, 2> = Tensor::ones([2, 2]);

        let param = Param::new(ParamId::new(), tensor);

        let param_item = param.into_item::<FullPrecisionSettings>();

        let serialized = param_item.serialize(Serializer::new()).unwrap();

        println!("{:?}", serialized);
    }
}
