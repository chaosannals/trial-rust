// 官方提供 2种方式，以下是 第一种 内联宏方式。
// 不可以通过编译，要使用 *.slint 的方式
// 注：详见 slinttdemo 代码，使用第二种方式。
// 内联宏感觉不错，直接通过宏实现语法分析，内置新语言。

slint::slint!{
    export component HelloWorld {
        Text {
            text: "hello world";
            color: green;
        }
    }
}

fn main() {
    HelloWorld::new().unwrap().run().unwrap();
}