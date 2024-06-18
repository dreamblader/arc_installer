use gtk4::{self as gtk, Orientation};
use gdk4::Display;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Box, CssProvider};

mod loader;

const APP_ID: &str = "org.gtk_rs.Css1";

// #[derive(Clone, Data)]
// struct UiData {
//     x:u8
// }

 fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|_button| {
        loader::plugin_loader::load_plugins();
    });

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&button);
    content.add_css_class("main-container");

    let window = ApplicationWindow::builder()
    .application(app)
    .default_width(1035)
    .default_height(584)
    .title("Guild Wars 2 - Arc Installer")
    .child(&content)
    .build();

    // Show the window.
    window.present();
 }


 fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_path("res/style.css");

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

        
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run()
}

// fn main() {
//     let main_window = WindowDesc::new(build_ui())
//         .window_size((600.0, 400.0))
//         .title("Guild Wars 2 - Arc DPS Manager");

//     let ui_data = UiData{x: 0};

//     AppLauncher::with_window(main_window)
//         .launch(ui_data)
//         .expect("Failed to launch application");
// }

//FIND THIS FILE: "gw2-64.exe" to add dll a the same folder of it