use std::cell::RefCell;
use magnus::block::Proc;
use magnus::{Error};
use magnus::value::Qnil;
use yrs::{XmlElement};

#[magnus::wrap(class = "Y::XMLElement")]
pub(crate) struct YXmlElement(pub(crate) RefCell<XmlElement>);

impl YXmlElement {}
