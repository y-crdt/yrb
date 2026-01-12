use magnus::r_string::IntoRString;
use magnus::value::ReprValue;
use magnus::{IntoValue, Ruby, Value};
use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use yrs::Any;

#[allow(dead_code)]
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
        let ruby = Ruby::get().unwrap();
        match self.0 {
            Any::Array(_v) => {
                let arr = ruby.ary_new();
                Ok(arr.as_value())
            }
            Any::Map(_v) => {
                let hash = ruby.hash_new();
                Ok(hash.as_value())
            }
            Any::Null => Ok(ruby.qnil().as_value()),
            Any::Undefined => Ok(ruby.qnil().as_value()),
            Any::Bool(v) => Ok(v.into_value_with(&ruby)),
            Any::Number(v) => Ok(v.into_value_with(&ruby)),
            Any::BigInt(v) => Ok(v.into_value_with(&ruby)),
            Any::String(v) => Ok(v.into_r_string_with(&ruby).as_value()),
            Any::Buffer(v) => Ok(ruby.str_from_slice(v.borrow()).as_value()),
        }
    }
}
