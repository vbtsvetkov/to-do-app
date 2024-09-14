use crate::to_do::structs::base::Base;
use crate::to_do::structs::traits::delete::Delete;
use crate::to_do::structs::traits::edit::Edit;
use crate::to_do::structs::traits::get::Get;

pub struct Done {
    pub super_struct: Base
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let base = Base::new(input_title, "done");
        Done {super_struct: base}
    }
}

impl Get for Done {}
impl Edit for Done {}
impl Delete for Done {}