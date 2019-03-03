use gtk::{ButtonExt, ContainerExt, GtkWindowExt, HeaderBarExt, StatusbarExt, WidgetExt};

use gtk::Orientation::Vertical;

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

#[derive(Clone)]
pub struct MainWindow {
    context_id: u32,
    statusbar: gtk::Statusbar,
    window: gtk::ApplicationWindow,
}

impl MainWindow {
    pub fn build_ui(application: &gtk::Application) -> Self {
        let glade_src = include_str!("ui/main.glade");
        let builder = gtk::Builder::new_from_string(glade_src);

        // let header = gtk::HeaderBar::new();
        let header: gtk::HeaderBar = builder.get_object("header").expect("Couldn't get header!");
        //header.set_show_close_button(true);

        // let add_button =
        //     gtk::Button::new_from_icon_name("document-new", gtk::IconSize::LargeToolbar);
        // header.add(&add_button);

        let main_box = gtk::Box::new(Vertical, 0);

        let statusbar = gtk::Statusbar::new();
        let id = statusbar.get_context_id("Status");
        main_box.add(&statusbar);

        let window: gtk::ApplicationWindow = builder
            .get_object("main-window")
            .expect("Couldn't get main-window");
        window.set_application(application);
        window.set_titlebar(&header);
        // window.set_title("YT Video Subscriber");
        window.set_default_size(800, 600);

        window.add(&main_box);

        // add_button.connect_clicked(clone!(statusbar => move |_| {
        //     statusbar.push(id, "Import clicked");
        // }));

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
