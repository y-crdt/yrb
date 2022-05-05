use std::cell::RefCell;
use std::rc::Rc;
use yrs::types::Delta;
use yrs::updates::decoder::Decode;
use yrs::{Doc, Update};

// fn main() {
//     let d1 = Doc::with_client_id(1);
//     let mut txt = {
//         let mut txn = d1.transact();
//         txn.get_text("text")
//     };
//
//     let diff: Rc<RefCell<Vec<Delta>>> =
//         Rc::new(RefCell::new(Vec::with_capacity(1)));
//     let d = diff.clone();
//
//     let _sub = txt.observe(move |txn, e| {
//         let mut delta = e.delta(txn).to_vec();
//         (*d).borrow_mut().append(&mut delta);
//     });
//
//     // {
//     //     let mut txn = d1.transact();
//     //     txt.insert(&mut txn, 0, "abcd");
//     // }
//     // {
//     //     let mut txn = d1.transact();
//     //     txt.insert(&mut txn, 0, "efgh");
//     // }
//     // {
//     //     let mut txn = d1.transact();
//     //     txt.remove_range(&mut txn, 2, 2);
//     // }
//
//     {
//         let mut txn = d1.transact();
//         txt.insert(&mut txn, 0, "abcd");
//         txt.insert(&mut txn, 4, "efgh");
//         txt.remove_range(&mut txn, 4, 1);
//     }
//
//     for change in (*diff).borrow().iter() {
//         match change {
//             Delta::Inserted(v, oa) => {
//                 println!("insert: {}", v.clone().to_string());
//             }
//             Delta::Retain(p, oa) => {
//                 println!("retain position: {}", p);
//             }
//             Delta::Deleted(p) => {
//                 println!("delete position: {}", p);
//             }
//         }
//     }
// }

fn main() {
    let local = Doc::new();
    let mut local_txn = local.transact();
    let mut local_txt = local_txn.get_text("text");

    let remote = Doc::new();
    let mut remote_txn = remote.transact();
    let remote_txt = remote_txn.get_text("text");
    remote_txt.push(&mut remote_txn, "hello, world");
    remote_txt.insert(&mut remote_txn, 0, "H");
    remote_txt.remove_range(&mut remote_txn, 1, 1);
    remote_txt.insert(&mut remote_txn, 7, "W");
    remote_txt.remove_range(&mut remote_txn, 8, 1);
    remote_txt.push(&mut remote_txn, "!");

    let local_sv = local_txn.state_vector();
    let remote_update = remote_txn.encode_diff_v1(&local_sv);
    // let remote_update = remote_txn.encode_update_v1();

    let diff: Rc<RefCell<Vec<Delta>>> =
        Rc::new(RefCell::new(Vec::with_capacity(1)));
    let d = diff.clone();

    let _sub = local_txt.observe(move |txn, e| {
        println!("update observed");
        let mut delta = e.delta(txn).to_vec();
        (*d).borrow_mut().append(&mut delta);
    });

    let local_update = Update::decode_v1(&*remote_update);
    let mut tx2 = local.transact();
    tx2.apply_update(local_update);
    tx2.commit();

    println!("num updates {}", (*diff).borrow().len());
    println!("{}", local_txt.to_string());
}
