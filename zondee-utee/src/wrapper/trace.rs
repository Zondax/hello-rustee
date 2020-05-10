use crate::wrapper::raw::{trace_get_level, trace_set_level, utee_log};
use core::fmt;
use zondee::StackStr;

pub struct Trace;

impl Trace {
    pub fn level() -> i32 {
        unsafe { trace_get_level() }
    }

    pub fn msg(args: fmt::Arguments) {
        let ss: StackStr<[u8; 64]> = StackStr::from_arguments(args);
        unsafe { utee_log(ss.as_str().as_ptr() as *const _, ss.as_str().len() as _) }
    }

    pub fn set_level(level: i32) {
        unsafe {
            trace_set_level(level);
        }
    }
}
