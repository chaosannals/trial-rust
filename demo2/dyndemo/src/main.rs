mod mylog;


use std::{
    io,
    sync::Arc
};
use mylog::init_env_logger;
use async_trait::async_trait;

#[async_trait]
trait MyTrait {
    async fn do_some(&self);
}

struct MyStruct {

}

#[async_trait]
impl MyTrait for MyStruct {
    async fn do_some(&self) {
        log::info!("do_some 1");
    }
}

struct MyStruct2 {

}

#[async_trait]
impl MyTrait for MyStruct2 {
    async fn do_some(&self) {
        log::info!("do_some 2");
    }
}

#[derive()]
struct DataHolder {
    pub my_t: Arc<dyn MyTrait + Send + Sync>,
}

impl Default for DataHolder {
    fn default() -> DataHolder {
        DataHolder {
            my_t: Arc::new(MyStruct {}),
        }
    }
}

fn init() {
    // init_env_logger();
}

#[tokio::main]
async fn main()  -> io::Result<()> {
    init_env_logger();

    let mut data = DataHolder::default();

    let handle = tokio::spawn(async move {
        data.my_t.do_some().await;
        data.my_t = Arc::new(MyStruct2 {});
        data.my_t.do_some().await;
    });

    let _ = handle.await.unwrap();

    Ok(())
}
