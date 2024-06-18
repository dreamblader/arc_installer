use druid::widget::Button;
use druid::{AppLauncher, Data, Widget, WindowDesc};

mod loader;

#[derive(Clone, Data)]
struct UiData {
    x:u8
}

fn build_ui() -> impl Widget<UiData> {
    Button::new("LOAD").on_click(|_,_,_| { loader::plugin_loader::load_plugins()})
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .window_size((600.0, 400.0))
        .title("Guild Wars 2 - Arc DPS Manager");
    
    let ui_data = UiData{x: 0};

    AppLauncher::with_window(main_window)
        .launch(ui_data)
        .expect("Failed to launch application");
}

//FIND THIS FILE: "gw2-64.exe" to add dll a the same folder of it