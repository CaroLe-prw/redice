//! Global tokio runtime for async operations

use std::sync::OnceLock;
use tokio::runtime::Runtime;

static RT: OnceLock<Runtime> = OnceLock::new();

/// Initialize the global tokio runtime. Call once at app startup.
pub fn init() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to create tokio runtime");
    let _ = RT.set(rt);
}

/// Get a reference to the global runtime.
pub fn get() -> &'static Runtime {
    RT.get()
        .expect("runtime not initialized, call runtime::init() first")
}
