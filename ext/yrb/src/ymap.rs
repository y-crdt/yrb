use std::cell::RefCell;
use magnus::block::Proc;
use magnus::{Error};
use magnus::value::Qnil;
use yrs::{Map};

#[magnus::wrap(class = "Y::Map")]
pub(crate) struct YMap(pub(crate) RefCell<Map>);

impl YMap {}
