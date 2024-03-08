use crate::{YText, YXmlElement, YXmlText};
use magnus::r_hash::ForEach::Continue;
use magnus::value::{Qnil, ReprValue};
use magnus::{class, value, Float, Integer, IntoValue, RArray, RHash, RString, Symbol, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use yrs::types::Value as YrsValue;
use yrs::{
    Any, Array, Map, TextRef as YrsText, Transact, XmlElementRef as YrsXmlElement,
    XmlTextRef as YrsXmlText,
};

pub(crate) struct YValue(pub(crate) RefCell<Value>);

impl From<Value> for YValue {
    fn from(value: Value) -> Self {
        YValue(RefCell::from(value))
    }
}

impl From<Qnil> for YValue {
    fn from(value: Qnil) -> Self {
        YValue(RefCell::from(value.into_value()))
    }
}

impl From<bool> for YValue {
    fn from(value: bool) -> Self {
        YValue(RefCell::from(value.into_value()))
    }
}

impl From<f64> for YValue {
    fn from(value: f64) -> Self {
        YValue(RefCell::from(value.into_value()))
    }
}

impl From<i64> for YValue {
    fn from(value: i64) -> Self {
        YValue(RefCell::from(value.into_value()))
    }
}

impl From<u32> for YValue {
    fn from(value: u32) -> Self {
        YValue(RefCell::from(value.into_value()))
    }
}

impl From<String> for YValue {
    fn from(value: String) -> Self {
        YValue(RefCell::from(value.into_value()))
    }
}

impl From<RArray> for YValue {
    fn from(value: RArray) -> Self {
        YValue(RefCell::from(value.into_value()))
    }
}

impl From<RHash> for YValue {
    fn from(value: RHash) -> Self {
        YValue(RefCell::from(value.into_value()))
    }
}

impl From<Vec<u8>> for YValue {
    fn from(value: Vec<u8>) -> Self {
        YValue(RefCell::from(value.into_value()))
    }
}

impl From<YrsText> for YValue {
    fn from(value: YrsText) -> Self {
        YValue(RefCell::from(YText(RefCell::from(value)).into_value()))
    }
}

impl From<YrsXmlElement> for YValue {
    fn from(value: YrsXmlElement) -> Self {
        YValue(RefCell::from(
            YXmlElement(RefCell::from(value)).into_value(),
        ))
    }
}

impl From<YrsXmlText> for YValue {
    fn from(value: YrsXmlText) -> Self {
        YValue(RefCell::from(YXmlText(RefCell::from(value)).into_value()))
    }
}

impl From<YText> for YValue {
    fn from(value: YText) -> Self {
        YValue(RefCell::from(value.into_value()))
    }
}

impl From<YXmlElement> for YValue {
    fn from(value: YXmlElement) -> Self {
        YValue(RefCell::from(value.into_value()))
    }
}

impl From<YXmlText> for YValue {
    fn from(value: YXmlText) -> Self {
        YValue(RefCell::from(value.into_value()))
    }
}

impl From<Any> for YValue {
    fn from(value: Any) -> Self {
        match value {
            Any::Null => YValue::from(value::qnil()),
            Any::Undefined => YValue::from(value::qnil()),
            Any::Bool(v) => YValue::from(v),
            Any::Number(v) => YValue::from(v),
            Any::BigInt(v) => YValue::from(v),
            Any::String(v) => YValue::from(v.to_string()),
            Any::Buffer(v) => YValue::from(v.to_vec()),
            Any::Array(v) => {
                let arr = RArray::new();
                for item in v.iter() {
                    let val = YValue::from(item.clone());
                    let val = *val.0.borrow();
                    arr.push(val).expect("cannot push item event to array");
                }
                YValue::from(arr)
            }
            Any::Map(v) => {
                let map = v
                    .iter()
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
            YrsValue::YText(text) => YValue::from(text),
            YrsValue::YXmlElement(el) => YValue::from(el),
            YrsValue::YXmlText(text) => YValue::from(text),
            YrsValue::YArray(val) => {
                let tx = val.transact();
                let arr = RArray::new();
                for item in val.iter(&tx) {
                    let val = YValue::from(item.clone());
                    let val = *val.0.borrow();
                    arr.push(val).expect("cannot push item event to array");
                }
                YValue::from(arr)
            }
            YrsValue::YMap(val) => {
                let tx = val.transact();
                let iter = val.iter(&tx).map(|(key, val)| {
                    let val = YValue::from(val);
                    let val = val.0.into_inner();
                    (key, val)
                });
                YValue::from(RHash::from_iter(iter))
            }
            v => panic!("cannot map complex yrs values to yvalue: {:?}", v),
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
            Any::String(Arc::from(s.name().unwrap()))
        } else if value.is_kind_of(class::true_class()) {
            Any::Bool(true)
        } else if value.is_kind_of(class::false_class()) {
            Any::Bool(false)
        } else if value.is_kind_of(class::string()) {
            let s = RString::from_value(value).unwrap();
            unsafe { Any::String(Arc::from(s.as_str().unwrap().to_string())) }
        } else if value.is_kind_of(class::array()) {
            let arr = RArray::from_value(value).unwrap();
            let items = arr
                .each()
                .map(|item| {
                    let yvalue = YValue::from(item.unwrap());
                    Any::from(yvalue)
                })
                .collect::<Vec<Any>>();
            Any::Array(Arc::from(items))
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
                    let result = converted_key.to_string();
                    result.unwrap()
                };
                m.insert(k, Any::from(YValue::from(val)));
                Ok(Continue)
            })
            .expect("cannot map key/value pair");

            Any::Map(Arc::from(m))
        } else {
            Any::Undefined
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Value> for YValue {
    fn into(self) -> Value {
        self.0.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use crate::yvalue::YValue;
    use magnus::value::ReprValue;
    use yrs::Any;

    #[test]
    fn convert_any_to_yvalue() {
        let _cleanup = unsafe { magnus::embed::init() };
        let value = Any::Null;
        let yvalue: YValue = value.into();

        assert!(yvalue.0.into_inner().is_nil());
    }
}
