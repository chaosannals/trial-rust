#[cfg(feature = "async-std-runtime")]
extern crate async_std;

#[allow(unused_imports)]
#[cfg(feature = "async-std-runtime")]
pub use async_std::{main, test};

#[allow(unused)]
#[cfg(feature = "async-std-runtime")]
pub async fn sleep(duration: std::time::Duration) {
    async_std::task::sleep(duration).await
}

pub const REQ_PORT: u32  = 4444;