#![allow(unused)]

use std::collections::HashMap;

macro_rules! __func__ {
  () => {{
    fn f() {}
    fn type_name_of<T>(_: T) -> &'static str {
      std::any::type_name::<T>()
    }
    let name = type_name_of(f);
    &name[..name.len() - 3].split("::").last().unwrap_or_default()
  }}
}

#[cfg(test)]
mod tests {
  use serde::Serialize;

  use crate::ser::Json5Serializer;
  use crate::ser::Json5Error;

  fn test<T: Serialize, S: ToString>(data: T, expect: S) {
    let mut serializer = Json5Serializer::default();
    let mut serialized = data.serialize(&mut serializer);

    // TODO: Json5Error only has Custom
    if let Err(Json5Error::Custom(err)) = serialized {
      eprintln!("{}", err);
      return;
    }

    println!("{:?}", serializer.output);
    assert_eq!(serializer.output, expect.to_string());
  }

  #[test]
  fn test_primitives() {
    test(true, "true");
    test(false, "false");
    test(-127i8, "-127");
    test(255u8, "255");
    test(-255i16, "-255");
    test(255u16, "255");
    test(-255i32, "-255");
    test(255u32, "255");
    test(-255i64, "-255");
    test(255u64, "255");
    test(-255i128, "-255");
    test(255u128, "255");
    test(255.255f32, "255.255");
    test(255.255f64, "255.255");
  }

  #[test]
  fn test_vec() {
    test(vec![1, 2, 3], "[1, 2, 3]");
  }
}

pub struct Json5;

pub mod ser {
  use std::fmt::{Display, Formatter};

  use serde::ser::*;
  use std::ops::AddAssign;

  #[derive(Debug, Default)]
  pub struct Json5Serializer {
    pub(crate) output: String,
  }

  impl Json5Serializer {
    #[inline]
    fn append(&mut self, value: impl ToString) -> Json5Result<()> {
      self.output += &value.to_string();

      Ok(())
    }

    #[inline]
    fn append_str(&mut self, str: &str) -> Json5Result<()> {
      self.output += str;
      Ok(())
    }
  }

  //region Error
  type Json5Result<T> = std::result::Result<T, Json5Error>;

  #[derive(Debug)]
  pub enum Json5Error {
    Custom(String)
  }

  impl StdError for Json5Error {}

  impl Display for Json5Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f, "Better Error handling wip")
    }
  }

  impl Error for Json5Error {
    fn custom<T>(msg: T) -> Self where T: Display {
      Json5Error::Custom(msg.to_string())
    }
  }
  //endregion

  impl<'a> Serializer for &'a mut Json5Serializer {
    //region Types
    type Ok = ();
    type Error = Json5Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;
    //endregion

    //region Primitives
    fn serialize_bool(self, v: bool) -> Json5Result<()> {
      self.append(v)
    }

    fn serialize_i8(self, v: i8) -> Json5Result<()> {
      self.append(v)
    }

    fn serialize_i16(self, v: i16) -> Json5Result<()> {
      self.append(v)
    }

    fn serialize_i32(self, v: i32) -> Json5Result<()> {
      self.append(v)
    }

    fn serialize_i64(self, v: i64) -> Json5Result<()> {
      self.append(v)
    }

    serde::serde_if_integer128! {
      fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.append(v)
      }
    }

    fn serialize_u8(self, v: u8) -> Json5Result<()> {
      self.append(v)
    }

    fn serialize_u16(self, v: u16) -> Json5Result<()> {
      self.append(v)
    }

    fn serialize_u32(self, v: u32) -> Json5Result<()> {
      self.append(v)
    }

    fn serialize_u64(self, v: u64) -> Json5Result<()> {
      self.append(v)
    }

    serde::serde_if_integer128! {
      fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.append(v)
      }
    }

    fn serialize_f32(self, v: f32) -> Json5Result<()> {
      self.append(v)
    }

    fn serialize_f64(self, v: f64) -> Json5Result<()> {
      self.append(v)
    }

    fn serialize_char(self, v: char) -> Json5Result<()> {
      self.append(v)
    }

    fn serialize_str(self, v: &str) -> Json5Result<()> {
      self.append_str("\"")?;
      self.append_str(v)?;
      self.append_str("\"")
    }
    //endregion

    //region Non-Primitives
    fn serialize_bytes(self, v: &[u8]) -> Json5Result<()> {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn serialize_none(self) -> Json5Result<()> {
      self.append_str("null")
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Json5Result<()> where T: Serialize {
      value.serialize(self)
    }

    fn serialize_unit(self) -> Json5Result<()> {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Json5Result<()> {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Json5Result<()> {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Json5Result<()> where T: Serialize {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Json5Result<()> where T: Serialize {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn serialize_seq(self, len: Option<usize>) -> Json5Result<Self::SerializeSeq> {
      self.append_str("[")?;
      Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Json5Result<Self::SerializeTuple> {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Json5Result<Self::SerializeTupleStruct> {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Json5Result<Self::SerializeTupleVariant> {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn serialize_map(self, len: Option<usize>) -> Json5Result<Self::SerializeMap> {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Json5Result<Self::SerializeStruct> {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Json5Result<Self::SerializeStructVariant> {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }
    //endregion
  }

  //region Serialize Serializer
  impl<'a> SerializeSeq for &'a mut Json5Serializer {
    type Ok = ();
    type Error = Json5Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Json5Result<()> where T: Serialize {
      if !self.output.ends_with("[") {
        self.append_str(", ")?;
      }

      value.serialize(&mut **self)
    }

    fn end(self) -> Json5Result<()> {
      self.append_str("]")
    }
  }

  impl<'a> SerializeTuple for &'a mut Json5Serializer {
    type Ok = ();
    type Error = Json5Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Json5Result<()> where T: Serialize {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn end(self) -> Json5Result<()> {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }
  }

  impl<'a> SerializeTupleStruct for &'a mut Json5Serializer {
    type Ok = ();
    type Error = Json5Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Json5Result<()> where T: Serialize {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn end(self) -> Json5Result<()> {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }
  }

  impl<'a> SerializeTupleVariant for &'a mut Json5Serializer {
    type Ok = ();
    type Error = Json5Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Json5Result<()> where T: Serialize {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn end(self) -> Json5Result<()> {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }
  }

  impl<'a> SerializeMap for &'a mut Json5Serializer {
    type Ok = ();
    type Error = Json5Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Json5Result<()> where T: Serialize {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Json5Result<()> where T: Serialize {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn end(self) -> Json5Result<()> {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }
  }

  impl<'a> SerializeStruct for &'a mut Json5Serializer {
    type Ok = ();
    type Error = Json5Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Json5Result<()> where T: Serialize {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn end(self) -> Json5Result<()> {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }
  }

  impl<'a> SerializeStructVariant for &'a mut Json5Serializer {
    type Ok = ();
    type Error = Json5Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Json5Result<()> where T: Serialize {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }

    fn end(self) -> Json5Result<()> {
      Err(Error::custom(format!("{f}:{l} - {name} is not implemented", name = __func__!(), f = file!(), l = line!())))
    }
  }
  //endregion
}
