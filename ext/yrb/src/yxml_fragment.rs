use std::cell::RefCell;
use yrs::XmlFragmentRef;

#[magnus::wrap(class = "Y::XMLFragment")]
pub(crate) struct YXmlFragment(pub(crate) RefCell<XmlFragmentRef>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for YXmlFragment {}

impl YXmlFragment {}

impl From<XmlFragmentRef> for YXmlFragment {
    fn from(v: XmlFragmentRef) -> Self {
        YXmlFragment(RefCell::from(v))
    }
}
