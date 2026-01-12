use magnus::{IntoValue, RHash, Ruby, Value};

unsafe impl Send for YDiff {}

#[magnus::wrap(class = "Y::Diff")]
pub(crate) struct YDiff {
    pub(crate) ydiff_insert: Value,
    pub(crate) ydiff_attrs: Option<RHash>,
}

impl YDiff {
    pub(crate) fn ydiff_insert(&self) -> Value {
        self.ydiff_insert
    }

    pub(crate) fn ydiff_attrs(&self) -> Option<Value> {
        let ruby = unsafe { Ruby::get_unchecked() };
        self.ydiff_attrs
            .as_ref()
            .map(|value| value.into_value_with(&ruby))
    }
}
