// 官方提供 2种方式，以下是第一种。
// 不可以通过编译，要使用 *.slint 的方式
// 注：详见 slinttdemo 代码，使用第二种方式。

slint::slint!{
    export component HelloWorld {
        Text {
            text: "hello world";
            color: green;
        }
    }
}

fn main() {
    HelloWorld::new().run();
}