#![cfg(target_os = "macos")]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unexpected_cfgs)]
#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

// use libc::*;
use core_foundation_sys::array::*;
use core_foundation_sys::attributed_string::*;
use core_foundation_sys::base::*;
use core_foundation_sys::bundle::*;
use core_foundation_sys::characterset::*;
use core_foundation_sys::data::*;
use core_foundation_sys::date::*;
use core_foundation_sys::dictionary::*;
use core_foundation_sys::error::*;
use core_foundation_sys::filedescriptor::*;
use core_foundation_sys::mach_port::*;
use core_foundation_sys::messageport::*;
use core_foundation_sys::number::*;
use core_foundation_sys::propertylist::*;
use core_foundation_sys::runloop::*;
use core_foundation_sys::set::*;
use core_foundation_sys::string::*;
use core_foundation_sys::url::*;
use core_foundation_sys::uuid::*;
use libc::{natural_t, uuid_t};
use mach::{clock_types::*, kern_return::kern_return_t, mach_types::*, message::*, vm_types::*};
use objc::runtime::Object as NSObject;

type CFMutableSetRef = *mut __CFSet;
type CFMutableArrayRef = *mut __CFArray;

include!(concat!(env!("OUT_DIR"), "/iokit.rs"));
// include!(concat!(env!("OUT_DIR"), "/iokit-extra.rs"));
