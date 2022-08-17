use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use lib0::any::Any;
use magnus::{class, Float, Integer, QNIL, RArray, RHash, RString, Symbol, Value};
use magnus::r_hash::ForEach;
use magnus::r_hash::ForEach::Continue;
use magnus::value::Qnil;
use yrs::types::Attrs;
use yrs::types::Value as YrsValue;
use crate::utils::{map_magnus_value_to_lib0_any, map_yrs_value_to_magnus_type};
use crate::{YArray, YText};
use crate::ymap::YMap;
use crate::yxml_element::YXmlElement;
use crate::yxml_text::YXmlText;

pub(crate) struct YValue(pub(crate) RefCell<Value>);

impl From<Value> for YValue {
    fn from(value: Value) -> Self {
        YValue(RefCell::from(value))
    }
}

impl From<Qnil> for YValue {
    fn from(value: Qnil) -> Self {
        YValue(RefCell::from(Value::from(value)))
    }
}

impl From<bool> for YValue {
    fn from(value: bool) -> Self {
        YValue(RefCell::from(Value::from(value)))
    }
}

impl From<f64> for YValue {
    fn from(value: f64) -> Self {
        YValue(RefCell::from(Value::from(value)))
    }
}

impl From<i64> for YValue {
    fn from(value: i64) -> Self {
        YValue(RefCell::from(Value::from(value)))
    }
}

impl From<String> for YValue {
    fn from(value: String) -> Self {
        YValue(RefCell::from(Value::from(value)))
    }
}

impl From<RArray> for YValue {
    fn from(value: RArray) -> Self {
        YValue(RefCell::from(Value::from(value)))
    }
}

impl From<RHash> for YValue {
    fn from(value: RHash) -> Self {
        YValue(RefCell::from(Value::from(value)))
    }
}

impl From<Any> for YValue {
    fn from(value: Any) -> Self {
        match value {
            Any::Null => YValue::from(QNIL),
            Any::Undefined => YValue::from(QNIL),
            Any::Bool(v) => YValue::from(v),
            Any::Number(v) => YValue::from(v),
            Any::BigInt(v) => YValue::from(v),
            Any::String(v) => YValue::from(v.into_string()),
            Any::Buffer(v) => YValue::from(Value::from(v.into_vec())),
            Any::Array(v) => {
                let arr = v.iter()
                    .map(|i| YValue::from(i.clone()))
                    .map(|value| *value.0.borrow())
                    .collect::<Vec<Value>>();
                YValue::from(RArray::from_vec(arr))
            }
            Any::Map(v) => {
                let map = v.iter()
                    .map(|(key, val)| {
                        let v = val.clone();
                        (key.to_string(), YValue::from(v).into())
                    })
                    .collect::<HashMap<String, Value>>();
                YValue::from(RHash::from_iter(map))
            }
        }
    }
}

impl From<YrsValue> for YValue {
    fn from(value: YrsValue) -> Self {
        match value {
            YrsValue::Any(val) => YValue::from(val),
            _ => panic!("cannot map complex yrs values to yvalue")
        }
    }
}

impl From<YValue> for Any {
    fn from(val: YValue) -> Self {
        let value = val.0.into_inner();
        if value.is_nil() {
            Any::Null
        } else if value.is_kind_of(class::float()) {
            let f = Float::from_value(value).unwrap();
            Any::Number(f.to_f64())
        } else if value.is_kind_of(class::integer()) {
            let i = Integer::from_value(value).unwrap();
            Any::BigInt(i.to_i64().unwrap())
        } else if value.is_kind_of(class::symbol()) {
            let s = Symbol::from_value(value).unwrap();
            Any::String(Box::from(s.name().unwrap()))
        } else if value.is_kind_of(class::true_class()) {
            Any::Bool(true)
        } else if value.is_kind_of(class::false_class()) {
            Any::Bool(false)
        } else if value.is_kind_of(class::string()) {
            let s = RString::from_value(value).unwrap();
            unsafe {
                Any::String(Box::from(s.as_str().unwrap().to_string()))
            }
        } else if value.is_kind_of(class::array()) {
            let arr = RArray::from_value(value).unwrap();
            let items = arr.each()
                .map(|item| {
                    let yvalue = YValue::from(item.unwrap());
                    Any::from(yvalue)
                })
                .collect::<Vec<Any>>();
            Any::Array(Box::from(items))
        } else if value.is_kind_of(class::hash()) {
            let map = RHash::from_value(value).unwrap();
            let mut m: HashMap<String, Any> = HashMap::new();

            // we need to map symbol keys to strings, because we can't store
            // symbols in any of the yrs data structures
            map.foreach(|key: Value, val: Value| {
                let k = if let Some(converted_key) = Symbol::from_value(key) {
                    converted_key.to_string()
                } else {
                    let converted_key = RString::from_value(key).unwrap();
                    let result = unsafe {
                        converted_key.to_string()
                    };
                    result.unwrap()
                };
                m.insert(k, Any::from(YValue::from(val)));
                Ok(Continue)
            }).expect("cannot map key/value pair");

            Any::Map(Box::from(m))
        } else {
            Any::Undefined
        }
    }
}

impl Into<Value> for YValue {
    fn into(self) -> Value {
        self.0.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use lib0::any::Any;
    use magnus::RArray;
    use crate::yvalue::YValue;

    #[test]
    fn convert_any_to_yvalue() {
        let value = Any::Null;
        let yvalue: YValue = value.into();
    }
}
