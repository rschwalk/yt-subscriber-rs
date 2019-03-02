use gtk::{ButtonExt, ContainerExt, GtkWindowExt, HeaderBarExt, StatusbarExt, WidgetExt};

use gtk::Orientation::Vertical;

pub struct MainWindow {
    context_id: u32,
    statusbar: gtk::Statusbar,
    window: gtk::ApplicationWindow,
}

impl MainWindow {
    pub fn build_ui(application: &gtk::Application) -> Self {
        let header = gtk::HeaderBar::new();
        header.set_show_close_button(true);

        let add_button =
            gtk::Button::new_from_icon_name("document-new", gtk::IconSize::LargeToolbar);
        header.add(&add_button);

        let main_box = gtk::Box::new(Vertical, 0);

        let statusbar = gtk::Statusbar::new();
        let id = statusbar.get_context_id("Status");
        main_box.add(&statusbar);

        let window = gtk::ApplicationWindow::new(application);
        window.set_titlebar(&header);
        window.set_title("YT Video Subscriber");
        window.set_default_size(800, 600);

        window.add(&main_box);

        let statusbar_clone = statusbar.clone();
        add_button.connect_clicked(move |_| {
            statusbar_clone.push(id, "Import clicked");
        });

        // window.show_all();

        MainWindow {
            context_id: id,
            statusbar: statusbar,
            window: window,
        }
    }

    pub fn start(&mut self) {
        self.window.show_all();
    }
}
