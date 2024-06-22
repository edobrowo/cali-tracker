slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // ui.on_add_reps({
    //     let ui_handle = ui.as_weak();
    //     move |reps| {
    //         let ui = ui_handle.unwrap();
    //         ui.set_rep_count(ui.get_rep_count() + reps);
    //     }
    // });

    ui.run()
}
