use std::collections::HashMap;
use lib0::any::Any;
use magnus::{RArray, RHash, Value};
use magnus::r_hash::ForEach;
use yrs::types::Attrs;

#[derive(Debug, Clone)]
pub(crate) struct TypeConversionError;

pub(crate) fn map_magnus_rhash_to_lib0_attrs(_hash: RHash) -> Result<Attrs, TypeConversionError> {
    todo!()
}
