//! Default build script generated by `cpp_to_rust`.
//! See the template at `cpp_to_rust/cpp_to_rust_generator/templates/crate/build.rs`.
//!
//! You can replace this with a custom build script by placing `build.rs` file in
//! the crate template and using `Config::set_crate_template_path` to specify the template.
//! However, make sure to call `cpp_to_rust_build_tools::run()` or
//! use the other `cpp_to_rust_build_tools` API in the custom build script to
//! perform the necessary build steps.

fn main() {
    ritual_build::run()
}