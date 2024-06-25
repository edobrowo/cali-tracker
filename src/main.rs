slint::include_modules!();
use std::rc::Rc;

fn main() -> Result<(), slint::PlatformError> {
    use slint::Model;

    let ui = AppWindow::new()?;

    ui.on_add_cali({
        let ui_handle = ui.as_weak();
        move |cali_data| {
            let ui = ui_handle.unwrap();

            let mut cali_counters: Vec<CaliData> = ui.get_cali_counters().iter().collect();
            cali_counters.push(cali_data);

            let cali_counters_model = Rc::new(slint::VecModel::from(cali_counters));
            ui.set_cali_counters(cali_counters_model.into());
        }
    });

    // let mut cali_counters: Vec<CaliData> = ui.get_cali_counters().iter().collect();

    // for cali_counter in &cali_counters {
    //     //
    // }

    ui.run()
}
