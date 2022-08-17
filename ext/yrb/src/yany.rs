use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use lib0::any::Any;
use magnus::{QNIL, RArray, RHash, RString, Value};

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
                let mut arr = RArray::new();
                Ok(Value::from(arr))
            }
            Any::Map(_v) => {
                let mut hash = RHash::new();
                Ok(Value::from(hash))
            }
            Any::Null => Ok(Value::from(QNIL)),
            Any::Undefined => Ok(Value::from(QNIL)),
            Any::Bool(v) => Ok(Value::from(v)),
            Any::Number(v) => Ok(Value::from(v)),
            Any::BigInt(v) => Ok(Value::from(v)),
            Any::String(v) => Ok(Value::from(RString::from(v.borrow()))),
            Any::Buffer(v) => Ok(Value::from(RString::from_slice(v.borrow()))),
        };
    }
}
