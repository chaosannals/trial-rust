pub struct ScopeCall<F: FnMut()>(pub F);
impl <F: FnMut()> Drop for ScopeCall<F> {
    fn drop(&mut self) {
        (self.0)();
    }
}

#[macro_export]
macro_rules! defer {
    ($e: block) => {
        let _scope_call = ScopeCall(||$e);
    }
}

fn main() {
    println!("defer demo.");
    defer!({
        println!("do on end.");
    });
    println!("do 1");
}
