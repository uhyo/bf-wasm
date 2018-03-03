#![feature(universal_impl_trait)]
#![feature(conservative_impl_trait)]

extern {
  // imported from js
  #[allow(dead_code)]
  fn js_debug(arg: usize);
}
use std::cell::{Cell, RefCell};
use std::mem;
use std::slice;
use std::os::raw::c_uchar;

mod error;
mod program;
mod run;

use error::*;

const MEMORY_INIT_SIZE: usize = 30000;

thread_local! {
  static PROGRAM: RefCell<Vec<program::Op>> = RefCell::new(vec![]);
  static MEMORY: RefCell<Vec<u8>> = RefCell::new(Vec::with_capacity(MEMORY_INIT_SIZE));
  // temporal buffer of user input.
  static INPUT: RefCell<(Vec<u8>, usize)> = RefCell::new((vec![], 0));
  static PC: Cell<usize> = Cell::new(0);
  static PTR: Cell<usize> = Cell::new(0);
}

/// Initialize the interpreter.
#[no_mangle]
pub fn parse(code_ptr: *const c_uchar, len: usize) -> i8 {
  unsafe {
    let prog = slice::from_raw_parts(code_ptr, len);
    match run::parse(prog) {
      Ok(code) => {
        PROGRAM.with(|p| p.replace(code));
      },
      Err(code) => {
        return code;
      },
    };
  }
  // Initialize memory
  MEMORY.with(|m| {
    let mut v = m.borrow_mut();
    for _ in v.len() .. MEMORY_INIT_SIZE {
      v.push(0);
    }
  });
  return ERROR_NONE;
}
/// Request an allocation of memory.
#[no_mangle]
pub fn alloc(size: usize) -> *mut c_uchar {
  let mut buf = Vec::with_capacity(size);
  buf.resize(size, 0);
  let ptr = buf.as_mut_ptr();
  // Do not drop it!
  mem::forget(buf);
  ptr
}
/// Drop it back.
#[no_mangle]
pub fn free(ptr: *mut c_uchar, size: usize) {
  unsafe {
    let _ = Vec::from_raw_parts(ptr, 0, size);
  }
}

/// Provide a new input.
#[no_mangle]
pub fn get_input_buf(size: usize) -> *mut c_uchar {
  let mut result = 0 as *mut c_uchar;
  INPUT.with(|rc| {
    let mut newbuf = Vec::with_capacity(size);
    newbuf.resize(size, 0);
    rc.replace((newbuf, 0));
    result = rc.borrow_mut().0.as_mut_ptr();
  });
  result
}
/// Run interpreter.
#[no_mangle]
pub fn run(
  // pointer to output buffer.
  buf: *mut c_uchar,
  // capacity of buffer.
  capacity: usize,
) -> isize {
  let outbuf = unsafe {
    slice::from_raw_parts_mut(buf, capacity)
  };

  let mut result = 0isize;
  PROGRAM.with(|p| {
    let code = p.borrow();

    MEMORY.with(|m| {
      let mut memory = m.borrow_mut();

      INPUT.with(|i| {
        let mut input = i.borrow_mut();

        PC.with(|pc_cell| {
          let pc = pc_cell.get();

          PTR.with(|ptr_cell| {
            let ptr = ptr_cell.get();

            let res = run::run(
              input.0.split_at(input.1).1,
              outbuf,
              &code,
              &mut memory,
              pc,
              ptr,
              );

            input.1 = res.input_read;
            pc_cell.set(res.pc);
            ptr_cell.set(res.ptr);
            if let Some(er) = res.error {
              if (er == ERROR_ENDED || er == ERROR_WAITING_INPUT) && res.output_size > 0 {
                result = res.output_size as isize;
              } else {
                result = er;
              }
            } else {
              result = res.output_size as isize;
            }
          });
        });
      });
    });
  });
  mem::forget(outbuf);

  result
}
