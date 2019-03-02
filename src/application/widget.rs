use gtk::{
    ApplicationWindow, Box, Button, ButtonExt, ContainerExt, GtkWindowExt, HeaderBar, HeaderBarExt,
    IconSize, Statusbar, StatusbarExt, WidgetExt,
};

use gtk::Orientation::Vertical;

pub struct MainWindow {
    context_id: u32,
    statusbar: Statusbar,
    window: ApplicationWindow,
}

impl MainWindow {
    pub fn build_ui(application: &gtk::Application) -> Self {
        let header = HeaderBar::new();
        header.set_show_close_button(true);

        let add_button = Button::new_from_icon_name("document-new", IconSize::LargeToolbar);
        header.add(&add_button);

        let main_box = Box::new(Vertical, 0);

        let statusbar = Statusbar::new();
        let id = statusbar.get_context_id("Status");
        main_box.add(&statusbar);

        let window = ApplicationWindow::new(application);
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
