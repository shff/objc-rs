#![crate_name = "objc"]
#![crate_type = "lib"]

use std::os::raw::{c_char, c_void};

#[cfg(not(target_arch = "aarch64"))]
pub type BOOL = ::std::os::raw::c_schar;
#[cfg(not(target_arch = "aarch64"))]
pub const YES: BOOL = 1;
#[cfg(not(target_arch = "aarch64"))]
pub const NO: BOOL = 0;

#[cfg(target_arch = "aarch64")]
pub type BOOL = bool;
#[cfg(target_arch = "aarch64")]
pub const YES: BOOL = true;
#[cfg(target_arch = "aarch64")]
pub const NO: BOOL = false;

#[repr(C)]
pub struct Sel {
    ptr: *const c_void,
}

#[repr(C)]
pub struct Class {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct Object {
    _priv: [u8; 0],
}

pub type Imp = unsafe extern fn();

#[link(name = "objc", kind = "dylib")]
extern {
    pub fn sel_registerName(name: *const c_char) -> Sel;
    pub fn objc_getClass(name: *const c_char) -> *const Class;
}

extern {
    fn objc_msgSend();
}

pub fn msg_send_fn() -> Imp {
    objc_msgSend
}

#[macro_export]
macro_rules! send {
    (@$class:ident $sel:ident) => ({
        unsafe {
            let name = concat!(stringify!($class), '\0');
            let class = objc::objc_getClass(name.as_ptr() as *const _).as_ref().expect(stringify!($class));
            let selName = concat!(stringify!($sel), '\0');
            let sel = objc::sel_registerName(selName.as_ptr() as *const _);
            let imp: extern fn(*mut $crate::Class, $crate::Sel) -> *mut $crate::Object = std::mem::transmute($crate::msg_send_fn());
            let receiver = &*class as *const $crate::Class as *mut $crate::Class;
            imp(receiver, sel)
        }
    });
    ($obj:expr, $sel:ident) => ({
        unsafe {
            let selName = concat!(stringify!($sel), '\0');
            let sel = objc::sel_registerName(selName.as_ptr() as *const _);
            let imp: extern fn(*mut $crate::Object, $crate::Sel) -> *mut $crate::Object = std::mem::transmute($crate::msg_send_fn());
            let receiver = &*$obj as *const $crate::Object as *mut $crate::Object;
            imp(receiver, sel)
        }
    });
    ($obj:expr, $($sel:ident : $arg:expr)+) => ({
        unsafe {
            unsafe fn invoke<R>(obj: *mut $crate::Object, sel: $crate::Sel, $($sel: R,)*) -> *mut $crate::Object where R: std::any::Any {
                let imp: unsafe extern fn(*mut $crate::Object, $crate::Sel $(, $sel: R)*) -> *mut $crate::Object = std::mem::transmute($crate::msg_send_fn());
                imp(obj, sel $(, $sel)*)
            }
            let selName = concat!($(stringify!($sel), ':'),+, '\0');
            let sel = objc::sel_registerName(selName.as_ptr() as *const _);
            let receiver = &*$obj as *const $crate::Object as *mut $crate::Object;
            invoke(receiver, sel $(, $arg)*)
        }
    });
}
