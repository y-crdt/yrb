use magnus::r_string::IntoRString;
use magnus::value::ReprValue;
use magnus::{value, IntoValue, RArray, RHash, RString, Value};
use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use yrs::Any;

pub(crate) struct YAny(pub(crate) Any);

impl Deref for YAny {
    type Target = Any;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for YAny {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryInto<Value> for YAny {
    type Error = ();

    fn try_into(self) -> Result<Value, Self::Error> {
        return match self.0 {
            Any::Array(_v) => {
                let arr = RArray::new();
                Ok(arr.as_value())
            }
            Any::Map(_v) => {
                let hash = RHash::new();
                Ok(hash.as_value())
            }
            Any::Null => Ok(value::qnil().as_value()),
            Any::Undefined => Ok(Value::from(value::qnil().as_value())),
            Any::Bool(v) => Ok(v.into_value()),
            Any::Number(v) => Ok(Value::from(v.into_value())),
            Any::BigInt(v) => Ok(Value::from(v.into_value())),
            Any::String(v) => Ok(RString::from(v.into_r_string()).as_value()),
            Any::Buffer(v) => Ok(RString::from_slice(v.borrow()).as_value()),
        };
    }
}
