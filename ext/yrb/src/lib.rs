use magnus::{method, Error, Module, Object, define_module, function};
use crate::yarray::YArray;
use crate::ydoc::YDoc;
use crate::ytext::YText;
use crate::ytransaction::YTransaction;

mod yarray;
mod ydoc;
mod ytext;
mod ytransaction;
mod utils;
mod yvalue;
mod yattrs;
mod yany;
mod ymap;
mod yxml_element;
mod yxml_text;

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Y")
        .expect("cannot define ::Y module");

    let yarray = module.define_class("Array", Default::default())
        .expect("cannot find class Y::Array");

    yarray.define_private_method("yarray_each", method!(YArray::yarray_each, 1)).expect("cannot define private method: yarray_each");
    yarray.define_private_method("yarray_get", method!(YArray::yarray_get, 1)).expect("cannot define private method: yarray_get");
    yarray.define_private_method("yarray_insert", method!(YArray::yarray_insert, 3)).expect("cannot define private method: yarray_insert");
    yarray.define_private_method("yarray_insert_range", method!(YArray::yarray_insert_range, 3)).expect("cannot define private method: yarray_insert_range");
    yarray.define_private_method("yarray_length", method!(YArray::yarray_length, 0)).expect("cannot define private method: yarray_length");
    yarray.define_private_method("yarray_observe", method!(YArray::yarray_observe, 1)).expect("cannot define private method: yarray_observe");
    yarray.define_private_method("yarray_push_back", method!(YArray::yarray_push_back, 2)).expect("cannot define private method: yarray_push_back");
    yarray.define_private_method("yarray_push_front", method!(YArray::yarray_push_front, 2)).expect("cannot define private method: yarray_push_front");
    yarray.define_private_method("yarray_remove", method!(YArray::yarray_remove, 2)).expect("cannot define private method: yarray_remove");
    yarray.define_private_method("yarray_remove_range", method!(YArray::yarray_remove_range, 3)).expect("cannot define private method: yarray_remove_range");
    yarray.define_private_method("yarray_to_a", method!(YArray::yarray_to_a, 0)).expect("cannot define private method: yarray_to_a");
    yarray.define_private_method("yarray_unobserve", method!(YArray::yarray_unobserve, 1)).expect("cannot define private method: yarray_unobserve");

    let ydoc = module
        .define_class("Doc", Default::default())
        .expect("cannot define class Y::Doc");
    ydoc.define_singleton_method("new", function!(YDoc::ydoc_new, 0)).expect("cannot define singelton method: ydoc_new");
    ydoc.define_private_method("ydoc_transact", method!(YDoc::ydoc_transact, 0)).expect("cannot define private method: ydoc_transact");
    ydoc.define_private_method("ydoc_encode_diff_v1", method!(YDoc::ydoc_encode_diff_v1, 1)).expect("cannot define private method: ydoc_encode_diff_v1");

    let ytransaction = module
        .define_class("Transaction", Default::default())
        .expect("cannot define class Y::Transaction");

    ytransaction.define_private_method("ytransaction_apply_update", method!(YTransaction::ytransaction_apply_update, 1)).expect("cannot define private method: ytransaction_apply_update");
    ytransaction.define_private_method("ytransaction_commit", method!(YTransaction::ytransaction_commit, 0)).expect("cannot define private method: ytransaction_commit");
    ytransaction.define_private_method("ytransaction_get_array", method!(YTransaction::ytransaction_get_array, 1)).expect("cannot define private method: ytransaction_get_array");
    ytransaction.define_private_method("ytransaction_get_map", method!(YTransaction::ytransaction_get_map, 1)).expect("cannot define private method: ytransaction_get_mao");
    ytransaction.define_private_method("ytransaction_get_text", method!(YTransaction::ytransaction_get_text, 1)).expect("cannot define private method: ytransaction_get_text");
    ytransaction.define_private_method("ytransaction_get_xml_element", method!(YTransaction::ytransaction_get_xml_element, 1)).expect("cannot define private method: ytransaction_get_xml_element");
    ytransaction.define_private_method("ytransaction_get_xml_text", method!(YTransaction::ytransaction_get_xml_text, 1)).expect("cannot define private method: ytransaction_get_xml_text");
    ytransaction.define_private_method("ytransaction_state_vector", method!(YTransaction::ytransaction_state_vector, 0)).expect("cannot define private method: ytransaction_state_vector");

    let ytext = module
        .define_class("Text", Default::default())
        .expect("cannot define class Y::Text");

    ytext.define_private_method("ytext_insert", method!(YText::ytext_insert, 3)).expect("cannot define private method: ytext_insert");
    ytext.define_private_method("ytext_insert_embed", method!(YText::ytext_insert_embed, 3)).expect("cannot define private method: ytext_insert_embed");
    ytext.define_private_method("ytext_insert_embed_with_attributes", method!(YText::ytext_insert_embed_with_attributes, 4)).expect("cannot define private method: ytext_insert_embed_with_attributes");
    ytext.define_private_method("ytext_length", method!(YText::ytext_length, 0)).expect("cannot define private method: ytext_length");
    ytext.define_private_method("ytext_push", method!(YText::ytext_push, 2)).expect("cannot define private method: ytext_push");
    ytext.define_private_method("ytext_to_s", method!(YText::ytext_to_s, 0)).expect("cannot define private method: ytext_to_s");

    let yxml_element = module
        .define_class("XMLElement", Default::default())
        .expect("cannot define class Y::XmlElement");

    let yxml_text = module
        .define_class("XMLText", Default::default())
        .expect("cannot define class Y::XmlText");

    Ok(())
}
