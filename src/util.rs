use std::collections::HashMap;
use rutie::{AnyException, AnyObject, Array, Boolean, Exception, Fixnum, Float, Hash, Integer, NilClass, Object, RString, Symbol};
use std::convert::TryFrom;
use lib0::any::Any;

pub(crate) fn convert_vecu8_to_array(vec: Vec<u8>) -> Array {
    let mut array = Array::new();

    for i in vec {
        array.push(Fixnum::new(i64::from(i)));
    }

    array
}

pub(crate) fn convert_array_to_vecu8(arr: Array) -> Vec<u8> {
    arr.into_iter()
        .map(|val| val.try_convert_to::<Fixnum>().unwrap().to_u32())
        .map(|val| u8::try_from(val).unwrap())
        .collect()
}


pub(crate) fn map_ruby_type_to_rust(input: AnyObject) -> Result<Any, AnyException> {
    if let Ok(_v) = input.try_convert_to::<NilClass>() {
        return Ok(Any::Null);
    } else if let Ok(v) = input.try_convert_to::<Boolean>() {
        return Ok(Any::Bool(v.to_bool()));
    } else if let Ok(v) = input.try_convert_to::<Float>() {
        return Ok(Any::Number(v.to_f64()));
    } else if let Ok(v) = input.try_convert_to::<Fixnum>() {
        return Ok(Any::BigInt(v.to_i64()));
    } else if let Ok(v) = input.try_convert_to::<Integer>() {
        return Ok(Any::BigInt(v.to_i64()));
    } else if let Ok(v) = input.try_convert_to::<RString>() {
        return Ok(Any::String(Box::from(v.to_str())));
    } else if let Ok(v) = input.try_convert_to::<Array>() {
        let arr: Vec<Any> = v
            .into_iter()
            .map(|value| map_ruby_type_to_rust(value).unwrap())
            .collect();
        return Ok(Any::Array(Box::from(arr)));
    } else if let Ok(v) = input.try_convert_to::<Hash>() {
        let m = map_hash_to_rust(v);
        return Ok(Any::Map(Box::from(m)));
    }

    Err(AnyException::new(
        "TypeError",
        Some("cannot map input type"),
    ))
}

/// This function gets reported as unused.
#[allow(dead_code)]
pub(crate) fn map_hash_to_rust(input: Hash) -> HashMap<String, Any> {
    let mut m = HashMap::with_capacity(input.length());
    input.each(|key, value| {
        if let Ok(v) = map_ruby_type_to_rust(value) {
            if let Ok(k) = key.try_convert_to::<RString>() {
                m.insert(k.to_string(), v);
            } else if let Ok(k) = key.try_convert_to::<Symbol>() {
                m.insert(k.to_string(), v);
            }
        }
    });
    m
}

