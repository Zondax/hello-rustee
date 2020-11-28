use crate::wrapper::raw::{trace_get_level, trace_set_level, utee_log};
use core::fmt;
use heapless::consts::U64;

pub struct Trace;

impl Trace {
    pub fn level() -> i32 {
        unsafe { trace_get_level() }
    }

    pub fn msg(args: fmt::Arguments) {
        let mut s: heapless::String<U64> = heapless::String::new();
        fmt::write(&mut s, args).expect("Bad formatting");
        unsafe { utee_log(s.as_str().as_ptr() as *const _, s.as_str().len() as _) }
    }

    pub fn set_level(level: i32) {
        unsafe {
            trace_set_level(level);
        }
    }
}
