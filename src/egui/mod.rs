pub use crate::egui::names::Names;

use crate::egui::{
    readable::{Readable, ReadableBuilder},
    writable::{Writable, WritableBuilder},
};
use typed_builder::TypedBuilder;

/// Name widget
#[derive(Debug, PartialEq, TypedBuilder)]
pub struct Name<'a> {
    id: &'a str,
}

impl<'a> NameBuilder<'a, ((&'a str,),)> {
    pub fn readable(self) -> ReadableBuilder<'a, ((&'a str,), (), (), ())> {
        Readable::builder().id(self.fields.0.0)
    }

    pub fn writable(self) -> WritableBuilder<'a, ((&'a str,), (), ())> {
        Writable::builder().id(self.fields.0.0)
    }
}

impl<'a> Name<'a> {
    pub fn readable() -> ReadableBuilder<'a> {
        Readable::builder()
    }

    pub fn writable() -> WritableBuilder<'a> {
        Writable::builder()
    }
}

pub mod names;
pub mod readable;
pub mod writable;
