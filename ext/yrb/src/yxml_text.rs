use std::cell::RefCell;
use magnus::block::Proc;
use magnus::{Error};
use magnus::value::Qnil;
use yrs::{XmlText};

#[magnus::wrap(class = "Y::XMLText")]
pub(crate) struct YXmlText(pub(crate) RefCell<XmlText>);

impl YXmlText {}
