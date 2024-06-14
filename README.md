# Rust FXR Reloader
This is just a fork of [vswarte/fxr-reloader](https://github.com/vswarte/fxr-reloader) that changes it into to a static library that provides a function that can be run from C/C++.

It is used by [fxr-ws-reloader](https://github.com/EvenTorset/fxr-ws-reloader) to patch FXR definitions while Elden Ring is running.

## C++ function declaration
```cpp
extern "C" {
  void patch_fxr(const char* process_name, const unsigned char* fxr_bytes, size_t fxr_size);
}
```
