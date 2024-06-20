#![allow(unused)]
use darling::FromMeta;

#[derive(FromMeta)]
pub struct Foo {
    qux: String,

    #[darling(skip)]
    bar: Option<u64>,
}
