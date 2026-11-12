//! Multiple generated bridges must link without overlapping native exports.
#![cfg(not(target_family = "wasm"))]

macro_rules! bridge {
    ($module:ident, $prefix:literal, $hash:expr) => {
        mod $module {
            use flutter_rust_bridge::for_generated::{Dart2RustMessageSse, WireSyncRust2DartSse};
            const FLUTTER_RUST_BRIDGE_CODEGEN_CONTENT_HASH: i32 = $hash;
            flutter_rust_bridge::frb_generated_boilerplate_io!();
            static FLUTTER_RUST_BRIDGE_HANDLER: Handler = Handler;
            struct Handler;
            impl Handler {
                fn dart_fn_handle_output(&self, _: i32, _: Dart2RustMessageSse) {}
            }
            pub static LAST_CALL: std::sync::atomic::AtomicI32 =
                std::sync::atomic::AtomicI32::new(0);
            fn pde_ffi_dispatcher_primary_impl(id: i32, _: i64, _: *mut u8, _: i32, _: i32) {
                LAST_CALL.store(id, std::sync::atomic::Ordering::SeqCst);
            }
            fn pde_ffi_dispatcher_sync_impl(
                _: i32,
                _: *mut u8,
                _: i32,
                _: i32,
            ) -> WireSyncRust2DartSse {
                WireSyncRust2DartSse {
                    ptr: std::ptr::null_mut(),
                    len: $hash,
                }
            }
        }
    };
}

bridge!(package_a, "package_a_", 11);
bridge!(package_b, "package_b_", 22);
bridge!(legacy, "", 33);


#[test]
fn two_generated_bridges_link() {}
