use gtk::{ApplicationWindow, GtkWindowExt, HeaderBar, HeaderBarExt, WidgetExt};

pub struct MainWindow {
    window: ApplicationWindow,
}

impl MainWindow {
    pub fn build_ui(application: &gtk::Application) -> Self {
        let header = HeaderBar::new();
        header.set_show_close_button(true);

        let window = ApplicationWindow::new(application);
        window.set_titlebar(&header);
        window.set_title("YT Video Subscriber");
        window.set_default_size(800, 600);

        // window.show_all();

        MainWindow { window: window }
    }

    pub fn start(&mut self) {
        self.window.show_all();
    }
}
