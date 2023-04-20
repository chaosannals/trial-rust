use slint::SharedString;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_to_page(move | e | {
        let ui = ui_handle.unwrap();
        ui.set_tip(e);
    });
    ui.invoke_to_page(SharedString::from("CCC"));
    // ui.on_request_increase_value(move || {
    //     let ui = ui_handle.unwrap();
    //     ui.set_counter(ui.get_counter() + 1);
    // });

    ui.global::<Logic>().set_the_value(44);

    ui.run()
}
