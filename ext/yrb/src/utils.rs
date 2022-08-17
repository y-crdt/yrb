use std::collections::HashMap;
use lib0::any::Any;
use magnus::{RArray, RHash, Value};
use magnus::r_hash::ForEach;
use yrs::types::Attrs;

#[derive(Debug, Clone)]
pub(crate) struct TypeConversionError;

pub(crate) fn map_magnus_value_to_lib0_any(value: Value) -> Result<Any, TypeConversionError> {
    return if value.is_nil() {
        Ok(Any::Null)
    } else if let Ok(v) = value.try_convert::<i8>() {
        Ok(Any::BigInt(v as i64))
    } else if let Ok(v) = value.try_convert::<i16>() {
        Ok(Any::BigInt(v as i64))
    } else if let Ok(v) = value.try_convert::<i32>() {
        Ok(Any::BigInt(v as i64))
    } else if let Ok(v) = value.try_convert::<i64>() {
        Ok(Any::BigInt(v))
    } else if let Ok(v) = value.try_convert::<isize>() {
        Ok(Any::BigInt(v as i64))
    } else if let Ok(v) = value.try_convert::<u8>() {
        Ok(Any::BigInt(v as i64))
    } else if let Ok(v) = value.try_convert::<u16>() {
        Ok(Any::BigInt(v as i64))
    } else if let Ok(v) = value.try_convert::<u32>() {
        Ok(Any::BigInt(v as i64))
    } else if let Ok(v) = value.try_convert::<u64>() {
        Ok(Any::BigInt(v as i64))
    } else if let Ok(v) = value.try_convert::<usize>() {
        Ok(Any::BigInt(v as i64))
    } else if let Ok(v) = value.try_convert::<f32>() {
        Ok(Any::Number(v as f64))
    } else if let Ok(v) = value.try_convert::<f64>() {
        Ok(Any::Number(v))
    } else if let Ok(v) = value.try_convert::<String>() {
        Ok(Any::String(Box::from(v)))
    } else if let Ok(v) = value.try_convert::<char>() {
        Ok(Any::String(Box::from(v.to_string())))
    } else if let Ok(v) = value.try_convert::<bool>() {
        Ok(Any::Bool(v))
    } else if let Some(v) = RArray::from_value(value) {
        let arr = v.each()
            .into_iter()
            .map(|val|
                map_magnus_value_to_lib0_any(val.unwrap()).unwrap()
            )
            .collect::<Vec<Any>>();
        Ok(Any::Array(Box::from(arr)))
    } else if let Some(v) = RHash::from_value(value) {
        let mut map: HashMap<String, Any> = HashMap::new();
        v.foreach(|key, val| {
            map.insert(key, map_magnus_value_to_lib0_any(val).unwrap());
            Ok(ForEach::Continue)
        }).unwrap();

        Ok(Any::Map(Box::from(map)))
    } else {
        Err(TypeConversionError)
    };
}

pub(crate) fn map_magnus_rhash_to_lib0_attrs(_hash: RHash) -> Result<Attrs, TypeConversionError> {
    todo!()
}
