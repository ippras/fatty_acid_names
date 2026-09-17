pub mod names;
pub mod readable;
pub mod writable;

pub use self::names::Names;

use crate::egui::{
    readable::{Readable, ReadableBuilder},
    writable::{Writable, WritableBuilder},
};
use lipid::r#struct::fatty_acid::FattyAcid;
use typed_builder::TypedBuilder;

/// Name widget
#[derive(Debug, PartialEq, TypedBuilder)]
pub struct Name<'a> {
    fatty_acid: &'a FattyAcid,
}

impl<'a> NameBuilder<'a, ((&'a FattyAcid,),)> {
    pub fn readable(self) -> ReadableBuilder<'a, ((&'a FattyAcid,), (), (), ())> {
        Readable::builder().fatty_acid(self.fields.0.0)
    }

    pub fn writable(self) -> WritableBuilder<'a, ((&'a FattyAcid,), (), ())> {
        Writable::builder().fatty_acid(self.fields.0.0)
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
