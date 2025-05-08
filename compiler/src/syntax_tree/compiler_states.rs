use super::{CompilerState, Object, Type};
use crate::types::Function;
use asm::assembler::register::Register;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
use util::{Offset, SResult};

/// グローバルなCompilerState
#[derive(Debug)]
pub struct GlobalState {
    function_list: RefCell<Vec<Function>>,
    type_list: RefCell<Vec<Type>>,
    assembly: RefCell<String>,
    error: RefCell<Vec<(Offset, String)>>,
}

/// プロシージャ用のCompilerState
pub struct ProcState {
    parent: Weak<dyn CompilerState>,
    object_list: RefCell<Vec<Object>>,
}

impl GlobalState {
    pub fn new() -> Self {
        GlobalState {
            function_list: RefCell::new(Vec::new()),
            type_list: RefCell::new(Type::primitive_types()),
            assembly: RefCell::new(String::new()),
            error: RefCell::new(Vec::new()),
        }
    }
}

impl CompilerState for GlobalState {
    fn child_for_proc(self: Rc<Self>) -> SResult<Rc<dyn CompilerState>> {
        let proc_state = ProcState {
            parent: Rc::downgrade(&self) as Weak<dyn CompilerState>,
            object_list: RefCell::new(Vec::new()),
        };
        Ok(Rc::new(proc_state))
    }

    fn add_function(self: Rc<Self>, function: Function) -> SResult<()> {
        if self.clone().get_function(&function.name).is_none() {
            self.function_list.borrow_mut().push(function);
            Ok(())
        } else {
            Err(format!(
                "function \"{}\" is defined multiple times",
                &function.name
            ))
        }
    }

    fn get_function(self: Rc<Self>, name: &str) -> Option<Function> {
        for i in &*self.function_list.borrow() {
            if &i.name == name {
                return Some(i.clone());
            }
        }
        None
    }

    fn add_type(self: Rc<Self>, r#type: Type) -> SResult<()> {
        if self.clone().get_type(&r#type.name).is_none() {
            self.type_list.borrow_mut().push(r#type);
            Ok(())
        } else {
            Err(format!(
                "type \"{}\" is defined multiple times",
                &r#type.name
            ))
        }
    }

    fn get_type(self: Rc<Self>, name: &str) -> Option<Type> {
        for i in &*self.type_list.borrow() {
            if &i.name == name {
                return Some(i.clone());
            }
        }
        None
    }

    fn clean_object(self: Rc<Self>) {
        // do nothing
    }

    fn add_object(self: Rc<Self>, object: Object) -> SResult<()> {
        Err("any object can't exist here".to_string())
    }

    fn get_object_by_name(self: Rc<Self>, name: &str) -> Option<Object> {
        None
    }

    fn get_object_by_register(self: Rc<Self>, register: Register) -> Option<Object> {
        None
    }

    fn copy_object(self: Rc<Self>, from: Register, to: Object) -> SResult<()> {
        Err("object can't be copied here".to_string())
    }

    fn move_object(self: Rc<Self>, from: Register, to: Object) -> SResult<()> {
        Err("object can't be moved here".to_string())
    }

    fn drop_object_by_name(self: Rc<Self>, name: &str) {
        // do nothing
    }

    fn drop_object_by_register(self: Rc<Self>, register: Register) {
        // do nothing
    }

    fn drop_object_without(self: Rc<Self>, register: Register) {
        // do nothiing
    }

    fn drop_object_all(self: Rc<Self>) {
        // do nothing
    }

    fn consume_object(self: Rc<Self>, register: Register) {
        // do nothing
    }

    fn add_asm(self: Rc<Self>, code: &str) {
        *self.assembly.borrow_mut() += code;
    }

    fn add_error(self: Rc<Self>, offset: Offset, msg: String) {
        self.error.borrow_mut().push((offset, msg))
    }

    fn status(self: Rc<Self>) -> Result<String, Vec<(Offset, String)>> {
        let error = self.error.borrow();
        if error.len() == 0 {
            Ok(self.assembly.borrow().clone())
        } else {
            Err(error.clone())
        }
    }
}

impl CompilerState for ProcState {
    fn child_for_proc(self: Rc<Self>) -> SResult<Rc<dyn CompilerState>> {
        let proc_state = ProcState {
            parent: Rc::downgrade(&self) as Weak<dyn CompilerState>,
            object_list: RefCell::new(Vec::new()),
        };
        Ok(Rc::new(proc_state))
    }

    fn add_function(self: Rc<Self>, function: Function) -> SResult<()> {
        Err("functions cannot defined here".to_string())
    }
    fn get_function(self: Rc<Self>, name: &str) -> Option<Function> {
        self.parent
            .upgrade()
            .expect("internal error")
            .get_function(name)
    }

    fn add_type(self: Rc<Self>, r#type: Type) -> SResult<()> {
        Err("types cannot defined here".to_string())
    }
    fn get_type(self: Rc<Self>, name: &str) -> Option<Type> {
        self.parent
            .upgrade()
            .expect("internal error")
            .get_type(name)
    }

    fn clean_object(self: Rc<Self>) {
        *self.object_list.borrow_mut() = Vec::new();
        self.parent
            .upgrade()
            .expect("internal error")
            .clean_object();
    }
    fn add_object(self: Rc<Self>, object: Object) -> SResult<()> {
        if let Some(ref name) = object.name {
            self.clone().drop_object_by_name(name);
        }
        self.clone().drop_object_by_register(object.data.register);

        let Some(r#type) = self.clone().get_type(&object.data.r#type) else {
            return Err(format!("type \"{}\" is undefined", object.data.r#type));
        };
        if !r#type.avaiable_registers.contains(&object.data.register) {
            return Err(format!(
                "register \"{}\" is unavaiable for type \"{}\"",
                object.data.register, &object.data.r#type
            ));
        }
        self.object_list.borrow_mut().push(object);
        Ok(())
    }
    fn get_object_by_name(self: Rc<Self>, name: &str) -> Option<Object> {
        for i in &*self.object_list.borrow() {
            if let Some(ref i_name) = i.name {
                if i_name == name {
                    return Some(i.clone());
                }
            }
        }
        self.parent
            .upgrade()
            .expect("internal error")
            .get_object_by_name(name)
    }
    fn get_object_by_register(self: Rc<Self>, register: Register) -> Option<Object> {
        for i in &*self.object_list.borrow() {
            if i.data.register == register {
                return Some(i.clone());
            }
        }
        self.parent
            .upgrade()
            .expect("internal error")
            .get_object_by_register(register)
    }
    fn copy_object(self: Rc<Self>, from: Register, to: Object) -> SResult<()> {
        let Some(r#type) = self.clone().get_type(&to.data.r#type) else {
            return Err(format!("type \"{}\" is undefined", &to.data.r#type));
        };
        if !r#type.copy {
            return Err("non-copyable data cannot be copied".to_string());
        }

        let Some(from_object) = self.clone().get_object_by_register(from) else {
            return Err("object doesn't exist".to_string());
        };

        if &from_object.data.r#type != &to.data.r#type {
            return Err("mismatching data type".to_string());
        }

        if from_object.data.register == to.data.register {
            let mut object_list = self.object_list.borrow_mut();
            for i in &mut *object_list {
                if i.data.register == to.data.register {
                    *i = to;
                    return Ok(());
                }
            }
            self.parent
                .upgrade()
                .expect("failed to upgrade parent compiler-state")
                .copy_object(from, to)
        } else {
            let to_data_register = to.data.register;
            self.clone().add_object(to)?;
            self.clone()
                .add_asm(&format!("mov {} {}\n", to_data_register, from));
            Ok(())
        }
    }
    fn move_object(self: Rc<Self>, from: Register, to: Object) -> SResult<()> {
        let Some(r#type) = self.clone().get_type(&to.data.r#type) else {
            return Err(format!("type \"{}\" is undefined", &to.data.r#type));
        };
        let Some(from_object) = self.clone().get_object_by_register(from) else {
            return Err("object doesn't exist".to_string());
        };

        if &from_object.data.r#type != &to.data.r#type {
            return Err("mismatching types".to_string());
        }
        if from != to.data.register {
            let mut removed_flag = false;
            self.clone()
                .object_list
                .borrow_mut()
                .retain(|object: &Object| {
                    object.data.register != from || {
                        removed_flag = true;
                        false
                    }
                });
            if removed_flag {
                self.clone()
                    .add_asm(&format!("mov {} {}\n", to.data.register, from));
                self.clone().add_object(to);
                return Ok(());
            }
        } else {
            for i in &mut *self.clone().object_list.borrow_mut() {
                if i.data.register == from {
                    *i = to;
                    return Ok(());
                }
            }
        }

        self.clone()
            .parent
            .upgrade()
            .expect("failed to upgrade parent compiler-state")
            .move_object(from, to)
    }
    fn drop_object_by_name(self: Rc<Self>, name: &str) {
        let mut object_list = self.object_list.borrow_mut();
        for i in 0..object_list.len() {
            if let Some(ref object_name) = object_list[i].name {
                if object_name == name {
                    object_list.remove(i);
                    break;
                }
            }
        }
        self.parent
            .upgrade()
            .expect("internal error")
            .drop_object_by_name(name);
    }
    fn drop_object_by_register(self: Rc<Self>, register: Register) {
        let mut object_list = self.object_list.borrow_mut();
        let mut i = 0;
        while i < object_list.len() {
            if object_list[i].data.register == register {
                object_list.remove(i);
            } else {
                i += 1;
            }
        }

        self.parent
            .upgrade()
            .expect("internal error")
            .drop_object_by_register(register);
    }

    fn drop_object_without(self: Rc<Self>, register: Register) {
        let mut object_list = self.object_list.borrow_mut();
        object_list.retain(|object: &Object| object.data.register == register);
    }

    fn drop_object_all(self: Rc<Self>) {
        let mut object_list = self.object_list.borrow_mut();
        object_list.clear();
    }

    fn consume_object(self: Rc<Self>, register: Register) {
        let mut object_list = self.object_list.borrow_mut();

        for i in 0..object_list.len() {
            if object_list[i].data.register == register {
                object_list.remove(i);
                return;
            }
        }
        self.parent
            .upgrade()
            .expect("failed to upgrade parent compiler-state")
            .consume_object(register);
    }

    fn add_asm(self: Rc<Self>, code: &str) {
        self.parent.upgrade().expect("internal error").add_asm(code);
    }

    fn add_error(self: Rc<Self>, offset: Offset, msg: String) {
        self.parent
            .upgrade()
            .expect("internal error")
            .add_error(offset, msg);
    }

    fn status(self: Rc<Self>) -> Result<String, Vec<(Offset, String)>> {
        self.parent.upgrade().expect("internal error").status()
    }
}
