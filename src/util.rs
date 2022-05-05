use crate::ytext::TEXT_WRAPPER;
use lib0::any::Any;
use rutie::{
    AnyException, AnyObject, Array, Boolean, Exception, Fixnum, Float, Hash,
    Integer, Module, NilClass, Object, RString, Symbol,
};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::Rc;
use yrs::types::{Attrs, Value};

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

pub(crate) fn map_any_type_to_ruby(input: &Any) -> AnyObject {
    match input {
        Any::Null => NilClass::new().to_any_object(),
        Any::Undefined => NilClass::new().to_any_object(),
        Any::Bool(b) => Boolean::new(*b).to_any_object(),
        Any::Number(f) => Float::new(*f).to_any_object(),
        Any::BigInt(i) => Integer::new(*i).to_any_object(),
        Any::String(s) => RString::new_utf8(s.as_ref()).to_any_object(),
        // TODO convert buffer into an array of Fixnum
        Any::Buffer(_b) => Array::new().to_any_object(),
        Any::Array(a) => {
            let values = a.iter().map(|n| map_any_type_to_ruby(n));
            Array::from_iter(values).to_any_object()
        }
        Any::Map(m) => {
            let mut h = Hash::new();
            m.iter().for_each(|(k, v)| {
                let key = Symbol::new(k.as_ref());
                let val = map_any_type_to_ruby(v);
                h.store(key, val);
                ()
            });
            h.to_any_object()
        }
    }
}

// This function gets reported as unused.
pub(crate) fn map_yrs_value_to_ruby(value: Value) -> AnyObject {
    match value {
        Value::Any(v) => map_any_type_to_ruby(v.borrow()),
        Value::YArray(a) => {
            let values = a.iter().map(|n| map_yrs_value_to_ruby(n));
            Array::from_iter(values).to_any_object()
        }
        Value::YText(t) => Module::from_existing("Y")
            .get_nested_class("Text")
            .wrap_data(t, &*TEXT_WRAPPER),
        _ => panic!("not supported yet"),
    }
}

pub(crate) fn map_ruby_type_to_rust(
    input: AnyObject,
) -> Result<Any, AnyException> {
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

// This function gets reported as unused.
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

pub(crate) fn map_hash_to_attrs(input: Hash) -> Attrs {
    let attributes = map_hash_to_rust(input);
    let mut attrs = Attrs::with_capacity(attributes.len());
    for (k, v) in attributes {
        attrs.insert(Rc::from(k), v);
    }
    attrs
}

pub(crate) fn map_attrs_to_hash(attrs: Attrs) -> Hash {
    let mut h = Hash::new();

    for (key, val) in attrs {
        let key = Symbol::new(key.as_ref());
        let value = map_any_type_to_ruby(val.borrow());
        h.store(key, value);
    }

    h
}
