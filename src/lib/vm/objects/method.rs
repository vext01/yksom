#![allow(clippy::new_ret_no_self)]

use std::cell::UnsafeCell;

use abgc_derive::GcLayout;

use crate::{
    compiler::instrs::Primitive,
    vm::{
        core::VM,
        objects::{Obj, ObjType, StaticObjType},
        val::{NotUnboxable, Val},
    },
};

#[derive(Debug, GcLayout)]
pub struct Method {
    pub name: String,
    pub body: MethodBody,
    class: UnsafeCell<Val>,
}

#[derive(Debug)]
pub enum MethodBody {
    /// A built-in primitive.
    Primitive(Primitive),
    /// User bytecode.
    User {
        /// How many variables does this method define?
        num_vars: usize,
        /// The offset of this method's bytecode in its parent class.
        bytecode_off: usize,
        max_stack: usize,
    },
}

impl Obj for Method {
    fn dyn_objtype(&self) -> ObjType {
        ObjType::Method
    }

    fn get_class(&self, _: &mut VM) -> Val {
        unimplemented!();
    }
}

impl NotUnboxable for Method {}

impl StaticObjType for Method {
    fn static_objtype() -> ObjType {
        ObjType::Method
    }
}

impl Method {
    pub fn new(vm: &VM, name: String, body: MethodBody) -> Method {
        Method {
            name,
            body,
            class: UnsafeCell::new(vm.nil.clone()),
        }
    }

    pub fn class(&self) -> Val {
        unsafe { &*self.class.get() }.clone()
    }

    pub fn set_class(&self, _: &VM, class: Val) {
        *unsafe { &mut *self.class.get() } = class;
    }
}
