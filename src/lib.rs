extern crate libc;
#[macro_use]
extern crate log;

use std::thread;
use std::string;

mod discovery;

use discovery::MulticastDiscovery;
use std::ffi::CString;
use std::ffi::CStr;
use self::libc::c_char;
use self::libc::size_t;
use std::iter::Iterator;
use std::mem;

#[repr(C)]
pub struct InstanceDescriptor {
  pub id: *const c_char,
  pub identifier: *const c_char,
  pub tags: *const *const c_char,
  pub codecs: *const *const c_char,
  pub connection_urls: *const *const c_char,
  pub tags_length: size_t,
  pub codecs_length: size_t,
  pub connection_urls_length: size_t
}

#[repr(C)]
pub struct OnReady {
  callback: extern fn() -> bool
}

//TODO, explicitly destroy the Service Instances


#[no_mangle]
pub extern fn get_service_names(target: *mut MulticastDiscovery) {
  let services = unsafe {
    (*target).get_known_services();
  };

  //TODO, transform to a list of cstr
}

#[no_mangle]
pub extern fn get_service_named(target: *mut MulticastDiscovery) -> InstanceDescriptor {
  //  let instances = unsafe {
  //    (*target).get_known_services();
  //  };
  println!("Getting service named");
  //TODO, find the right one
  unsafe {
    InstanceDescriptor {
      id: to_ptr("hello".to_string()),
      identifier: to_ptr("MY IDENTIFIER!".to_string()),
      tags: vec![].as_ptr(),
      codecs: vec![].as_ptr(),
      connection_urls: vec![].as_ptr(),
      tags_length: 0,
      codecs_length: 0,
      connection_urls_length: 0
    }
  }
}

#[no_mangle]
pub extern fn get_service_with_tags(target: *mut MulticastDiscovery) -> InstanceDescriptor {
  let instances = unsafe {
    (*target).get_known_services();
  };

  //TODO, find the right one

  unsafe {
//    Box::into_raw(Box::new(
      InstanceDescriptor {
        id: to_ptr("hello".to_string()),
        identifier: to_ptr("MY IDENT IS COOL".to_string()),
        tags: vec![].as_ptr(),
        codecs: vec![].as_ptr(),
        connection_urls: vec![].as_ptr(),
        tags_length: 0,
        codecs_length: 0,
        connection_urls_length: 0
      }
//    ))
  }
}

#[no_mangle]
pub extern fn destroy_descriptor(descriptor: *mut InstanceDescriptor) {
  unsafe {
    drop(Box::from_raw(descriptor))
  }
}

#[no_mangle]
pub extern fn advertise_local_service_full(target: *mut MulticastDiscovery,
id: *const c_char,
identifier: *const c_char,
tags: *const *const c_char,
//codecs: *const *const c_char,
//connection_urls: *const *const c_char,
tags_length: size_t,
//codecs_length: size_t,
//connection_urls_length: size_t
) {

  unsafe {
    println!("WOOT? {}", from_cstr(id));
    println!("WOOT? {}", from_cstr(identifier));
    println!("WOOT? {}", tags_length);
  }
}



#[no_mangle]
pub extern fn advertise_local_service(target: *mut MulticastDiscovery, descriptor: InstanceDescriptor) {
  unsafe {
    (*target).advertise_local_service(discovery::InstanceDescriptor {
      id: from_cstr(descriptor.id),
      identifier: from_cstr(descriptor.identifier),
      tags: array_to_vec(descriptor.tags, descriptor.tags_length),
      codecs: array_to_vec(descriptor.codecs, descriptor.codecs_length),
      connection_urls: array_to_vec(descriptor.connection_urls, descriptor.connection_urls_length),
    });
  }
}

#[no_mangle]
pub extern fn on_ready(target: *mut MulticastDiscovery, call: OnReady) {
  println!("I'm called from C");
  unsafe {
    (*target).on_ready(move || {
      println!("EXTERNAL ON READY, calling exec.....");
      (call.callback)();
    })
  }
}

#[no_mangle]
pub extern fn create(name: *const u8) -> *mut MulticastDiscovery {
  let ret = Box::into_raw(Box::new(discovery::run()));
  mem::forget(ret);
  ret
}

#[no_mangle]
pub extern fn shutdown(target: *mut MulticastDiscovery) {
  unsafe {
    (*target).shutdown();
    drop(target);
  }
}

///  Internal reworking

fn array_to_vec<'a>(vals: *const *const c_char, len: size_t) -> Vec<String>
{
  let arr = unsafe {
    std::slice::from_raw_parts(vals, len as usize)
      .iter().map(|tag| {
      let val = CStr::from_ptr((*tag)).to_str();
      return val.unwrap();
    })
  };

  let mut vector = Vec::new();
  for i in arr {
    println!("{}", i);
    vector.push(i.to_string());
  }
  vector
}

fn to_ptr(string: String) -> *const c_char {
  let cs = CString::new(string.as_bytes()).unwrap();
  let ptr = cs.as_ptr();
  // Tell Rust not to clean up the string while we still have a pointer to it.
  // Otherwise, we'll get a segfault.
  mem::forget(cs);
  ptr
}

fn from_cstr(str: *const c_char ) -> String {
  unsafe { CStr::from_ptr(str) }.to_str().unwrap().to_string().to_owned()
}
