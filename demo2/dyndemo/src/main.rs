use std::{
    io,
    sync::Arc
};

trait MyTrait {
    fn do_some(&self);
}

struct MyStruct {

}

impl MyTrait for MyStruct {
    fn do_some(&self) {

    }
}

struct MyStruct2 {

}

impl MyTrait for MyStruct2 {
    fn do_some(&self) {

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

#[tokio::main]
async fn main()  -> io::Result<()> {
    println!("dyn demo");

    let mut data = DataHolder::default();

    tokio::spawn(async move {
        data.my_t = Arc::new(MyStruct2 {});
    });

    Ok(())
}
