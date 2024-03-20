
use std::io::Write;
use log::Level;
use chrono::Local;
use env_logger::{Builder, Target, fmt::style};

// 由于 env_logger 把这个类私有了，所以只能直接搬代码了。
struct MyStyledValue<T> {
    style: style::Style,
    value: T,
}

impl<T: std::fmt::Display> std::fmt::Display for MyStyledValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let style = self.style;

        // We need to make sure `f`s settings don't get passed onto the styling but do get passed
        // to the value
        write!(f, "{style}")?;
        self.value.fmt(f)?;
        write!(f, "{style:#}")?;
        Ok(())
    }
}

pub fn init_env_logger() {
    Builder::from_env(
        env_logger::Env::new()
        .default_filter_or("info")
    )
    .target(Target::Stdout)
    .format(|stream, record| {
        let now = Local::now().naive_local();
        let level = record.level();
        let level_style = match level {
            Level::Trace => style::AnsiColor::Cyan.on_default(),
            Level::Debug => style::AnsiColor::Blue.on_default(),
            Level::Info => style::AnsiColor::Green.on_default(),
            Level::Warn => style::AnsiColor::Yellow.on_default(),
            Level::Error => style::AnsiColor::Red
                .on_default()
                .effects(style::Effects::BOLD),
        };

        writeln!(
            stream,
            "[{now} {level} {module_path}:{line}] {args}",
            now = MyStyledValue {
                style: style::AnsiColor::Cyan.on_default(),
                value: now
            },
            level = MyStyledValue {
                style: level_style,
                value: level
            },
            module_path=if let Some(mp) = record.module_path() { mp } else {"-"},
            line = MyStyledValue {
                style: style::AnsiColor::Yellow.on_default(),
                value: if let Some(line) = record.line() { line } else { 0u32 }
            },
            args = record.args()
        )
    })
    .init();
}