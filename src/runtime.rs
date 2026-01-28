use std::sync::OnceLock;

use tokio::runtime::Runtime;

static RT: OnceLock<Runtime> = OnceLock::new();

pub(super) fn init_runtime() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("crate fail tokio runtime");
    let _ = RT.set(rt);
}

pub(crate) fn rt() -> &'static Runtime {
    RT.get().expect("runtime not initialized")
}
