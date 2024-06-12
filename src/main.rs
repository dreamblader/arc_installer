use druid::widget::Button;
use druid::{AppLauncher, Widget, WindowDesc};

mod loader;

fn build_ui() -> impl Widget<()> {
    Button::new("LOAD").on_click(|_,_,_| { loader::plugin_loader::load_plugins()})
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .window_size((600.0, 400.0))
        .title("Guild Wars 2 Plugin Manager");
    let initial_data = ();

    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}
