mod util;
mod yarray;
mod ydoc;
mod ytext;
mod ytransaction;

#[macro_use]
extern crate rutie;

use rutie::{Module, Object};

module!(Y);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_yrb() {
    Module::new("Y").define(|module| {
        module.define_nested_class("Array", None).define(|klass| {
            klass.def("length", yarray::yarray_length);
            klass.def("insert", yarray::yarray_insert);
            klass.def("remove", yarray::yarray_remove);
            klass.def("remove_range", yarray::yarray_remove_range);
            klass.def("to_arr", yarray::yarray_to_arr);
        });

        module.define_nested_class("Text", None).define(|klass| {
            klass.def("insert", ytext::ytext_insert);
            klass.def("insert_embed", ytext::ytext_insert_embed);
            klass.def(
                "insert_embed_with_attrs",
                ytext::ytext_insert_embed_with_attributes,
            );
            klass.def("insert_with_attrs", ytext::ytext_insert_with_attributes);
            klass.def("remove_range", ytext::ytext_remove_range);
            klass.def("format", ytext::format);
            klass.def("length", ytext::ytext_length);
            klass.def("push", ytext::ytext_push);
            klass.def("to_s", ytext::ytext_to_string);
        });

        module
            .define_nested_class("Transaction", None)
            .define(|klass| {
                klass.def("commit", ytransaction::ytransaction_commit);
                klass.def("get_array", ytransaction::ytransaction_get_array);
                klass.def("get_text", ytransaction::ytransaction_get_text);
                klass.def(
                    "apply_update",
                    ytransaction::ytransaction_apply_update,
                );
            });

        module.define_nested_class("Doc", None).define(|klass| {
            klass.def_self("new", ydoc::ydoc_new);
            klass.def("transact", ydoc::ydoc_transact);
            klass.def("state_vector", ydoc::ydoc_state_vector);
            klass.def("encode_diff_v1", ydoc::ydoc_encode_diff_v1);
        });
    });
}
