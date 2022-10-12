use crate::awareness::{Awareness, AwarenessUpdate, Event};
use magnus::block::Proc;
use magnus::{Error, Value};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use yrs::block::ClientID;
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::Doc;

#[magnus::wrap(class = "Y::Awareness")]
pub(crate) struct YAwareness(pub(crate) RefCell<Awareness>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for YAwareness {}

impl YAwareness {
    pub(crate) fn yawareness_new() -> Self {
        let doc = Doc::new();
        let awareness = Awareness::new(doc);

        Self(RefCell::new(awareness))
    }

    pub(crate) fn yawareness_apply_update(
        &self,
        update: &YAwarenessUpdate
    ) -> Result<(), Error> {
        update.decode().and_then(|value| {
            self.0.borrow_mut().apply_update(value).map_err(|_error| {
                Error::runtime_error("cannot decode awareness update")
            })
        })
    }

    pub(crate) fn yawareness_clean_local_state(&self) {
        self.0.borrow_mut().clean_local_state();
    }

    pub(crate) fn yawareness_client_id(&self) -> ClientID {
        self.0.borrow().client_id()
    }

    pub(crate) fn yawareness_clients(&self) -> HashMap<ClientID, String> {
        self.0.borrow().clients().to_owned()
    }

    pub(crate) fn yawareness_local_state(&self) -> Option<String> {
        self.0.borrow().local_state().map(|value| value.to_string())
    }

    pub(crate) fn yawareness_on_update(
        &self,
        block: Proc
    ) -> Result<u32, Error> {
        let subscription_id = self
            .0
            .borrow_mut()
            .on_update(move |_awareness, event| {
                let awareness_event = YAwarenessEvent::from(event);
                let args = (awareness_event,);
                block
                    .call::<(YAwarenessEvent,), Value>(args)
                    .expect("cannot call block: on_update");
            })
            .into();

        Ok(subscription_id)
    }

    pub(crate) fn yawareness_remove_on_update(&self, subscription_id: u32) {
        self.0.borrow_mut().remove_on_update(subscription_id)
    }

    pub(crate) fn yawareness_remove_state(&self, client_id: ClientID) {
        self.0.borrow_mut().remove_state(client_id)
    }

    pub(crate) fn yawareness_set_local_state(&self, json: String) {
        self.0.borrow_mut().set_local_state(json)
    }

    pub(crate) fn yawareness_update(&self) -> Result<YAwarenessUpdate, Error> {
        self.0
            .borrow_mut()
            .update()
            .map(YAwarenessUpdate::from)
            .map_err(|_error| Error::runtime_error("cannot update awareness"))
    }

    pub(crate) fn yawareness_update_with_clients(
        &self,
        clients: Vec<ClientID>
    ) -> Result<YAwarenessUpdate, Error> {
        self.0
            .borrow_mut()
            .update_with_clients(clients)
            .map(YAwarenessUpdate::from)
            .map_err(|_error| {
                Error::runtime_error("cannot update awareness with clients")
            })
    }
}

impl From<Awareness> for YAwareness {
    fn from(value: Awareness) -> Self {
        Self(RefCell::from(value))
    }
}

#[magnus::wrap(class = "Y::AwarenessUpdate")]
pub(crate) struct YAwarenessUpdate(pub(crate) Vec<u8>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for YAwarenessUpdate {}

impl YAwarenessUpdate {
    pub(crate) fn decode(&self) -> Result<AwarenessUpdate, Error> {
        AwarenessUpdate::decode_v1(self.0.borrow()).map_err(|_error| {
            Error::runtime_error("cannot decode awareness update")
        })
    }
    pub(crate) fn yawareness_update_encode(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl From<AwarenessUpdate> for YAwarenessUpdate {
    fn from(value: AwarenessUpdate) -> Self {
        YAwarenessUpdate(value.encode_v1())
    }
}

impl From<Vec<u8>> for YAwarenessUpdate {
    fn from(value: Vec<u8>) -> Self {
        YAwarenessUpdate(value)
    }
}

#[magnus::wrap(class = "Y::AwarenessEvent")]
pub(crate) struct YAwarenessEvent(Event);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for YAwarenessEvent {}

impl YAwarenessEvent {
    pub(crate) fn added(&self) -> Vec<ClientID> {
        self.0.borrow().added().to_vec()
    }
    pub(crate) fn updated(&self) -> Vec<ClientID> {
        self.0.borrow().updated().to_vec()
    }
    pub(crate) fn removed(&self) -> Vec<ClientID> {
        self.0.borrow().removed().to_vec()
    }
}

impl From<&Event> for YAwarenessEvent {
    fn from(value: &Event) -> Self {
        Self(value.clone())
    }
}
