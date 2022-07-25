#![feature(const_trait_impl)]

use magnus::{eval, method, Error, Module, Object, require, RModule, define_module, function};
use crate::yarray::YArray;
use crate::ydoc::YDoc;
use crate::ytext::YText;
use crate::ytransaction::YTransaction;

mod yarray;
mod ydoc;
mod ytext;
mod ytransaction;

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Y")
        .expect("cannot define ::Y module");

    let yarray = module.define_class("Array", Default::default())
        .expect("cannot find class Y::Array");

    yarray.define_private_method("yarray_each", method!(YArray::yarray_each, 1)).expect("cannot define private method: yarray_each");

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
    ytransaction.define_private_method("ytransaction_apply_update", method!(YTransaction::ytransaction_get_array, 1)).expect("cannot define private method: ytransaction_get_array");
    ytransaction.define_private_method("ytransaction_get_text", method!(YTransaction::ytransaction_get_text, 1)).expect("cannot define private method: ytransaction_get_text");
    ytransaction.define_private_method("ytransaction_state_vector", method!(YTransaction::ytransaction_state_vector, 0)).expect("cannot define private method: ytransaction_state_vector");

    let ytext = module
        .define_class("Text", Default::default())
        .expect("cannot define class Y::Text");

    ytext.define_private_method("ytext_insert", method!(YText::ytext_insert, 3)).expect("cannot define private method: ytext_insert");
    ytext.define_private_method("ytext_push", method!(YText::ytext_push, 2)).expect("cannot define private method: ytext_push");
    ytext.define_private_method("ytext_to_s", method!(YText::ytext_to_s, 0)).expect("cannot define private method: ytext_to_s");

    Ok(())
}
