use crate::wrapper::raw::{_utee_log as utee_log, trace_get_level, trace_set_level};
use arrayvec::ArrayString;
use core::fmt;

pub struct Trace;

impl Trace {
    pub fn level() -> i32 {
        unsafe { trace_get_level() }
    }

    pub fn msg(args: fmt::Arguments) {
        let mut s = ArrayString::<[_; 256]>::new();
        fmt::write(&mut s, args).expect("Bad formatting");
        unsafe { utee_log(s.as_str().as_ptr() as *const _, s.as_str().len() as _) }
    }

    pub fn set_level(level: i32) {
        unsafe {
            trace_set_level(level);
        }
    }
}
