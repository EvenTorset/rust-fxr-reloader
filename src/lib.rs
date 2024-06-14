mod fxr;
mod util;

use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn patch_fxr(process_name: *const c_char, fxr_bytes: *const u8, fxr_len: usize) {
  let process_name_cstr = CString::from_raw(process_name as *mut c_char);
  let process_name_str = process_name_cstr.to_str().unwrap().to_string();
  let fxr_vec = std::slice::from_raw_parts(fxr_bytes, fxr_len).to_vec();
  fxr::patch_fxr_definition(process_name_str, fxr_vec);
  std::mem::forget(process_name_cstr);
}
