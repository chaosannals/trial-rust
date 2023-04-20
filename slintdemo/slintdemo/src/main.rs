// 官方提供 2种方式，以下是 第一种 内联宏方式。
// 注：官方文档留有很多老代码，API 变动频繁，很多代码官方文档拿下来都是语法错误。
// 注：要执行需要查看对应的版本对应的示例。
// 注：详见 slinttdemo 代码，使用第二种方式。
// 内联宏感觉不错，直接通过宏实现语法分析，内置新语言。


// 注：因为要走 slint-build 所以每次编译要走2次，第一次是没有更新代码修改的。第二次才是新的。
slint::slint! {
    // 旧版本示例 export global Logic := {....}
    // 去掉 := 就好了。
    // 旧版 默认 property 是开放的，但是新版必须加上 in-out 来开放
    export global Logic {
        in-out property <int> the-value;
        callback magic-operation(int) -> int;
    }

    // 可以通过以下特征判别新旧版本。
    // 1. 早期版本 export HelloWorld := {....}
    // 后来去掉了 := 改用 export component HelloWorld
    export component HelloWorld {

        VerticalLayout { // VerticalLayout 这东西有 bug 属性无效，布局乱
            min-width: 400px;
            min-height: 400px;

            HorizontalLayout { // 官方示例使用这个，所以这个 bug 少。
                min-width: 400px;
                min-height: 400px;

                Text {
                    // 注：引用变量的方式和 *.slint 文件不一致。 \{..} 会引发编译错误。
                    text: "the: {Logic.the-value}";
                    // text: "the: \{Logic.the-value}";
                    color: green;
                }
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let app = HelloWorld::new()?;
    app.global::<Logic>().on_magic_operation(|value| {
        eprintln!("magic operation input: {}", value);
        value * 2
    });
    app.global::<Logic>().set_the_value(44);
    app.run()
}