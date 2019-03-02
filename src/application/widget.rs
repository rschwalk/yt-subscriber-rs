use gtk::{ApplicationWindow, GtkWindowExt, HeaderBar, WidgetExt};

pub struct MainWindow {
    // window: ApplicationWindow,
}

impl MainWindow {
    pub fn build_ui(application: &gtk::Application) {
        let window = ApplicationWindow::new(application);
        window.set_title("YT Video Subscriber");
        window.set_default_size(800, 600);

        window.show_all();

        // MainWindow { window: window }
    }
}
