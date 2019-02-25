extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{Button, Label, Separator, Window, WindowType};
use relm::{Relm, Update, Widget};

struct Model {}

#[derive(Msg)]
enum Msg {
    Quit,
}

#[derive(Clone)]
struct Widgets {
    import_button: Button,
    window: Window,
}

struct Win {
    // …
    model: Model,
    widgets: Widgets,
}

impl Update for Win {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    // Return the initial model.
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {}
    }

    // The model may be updated when a message is received.
    // Widgets may also be updated in this function.
    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    // Specify the type of the root widget.
    type Root = Window;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    // Create the widgets.
    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let hbox = gtk::Box::new(Horizontal, 10);
        let vbox_side_pane = gtk::Box::new(Vertical, 0);
        let vbox_content_pane = gtk::Box::new(Vertical, 0);

        let import_button = Button::new_with_label("Import URL");
        vbox_side_pane.add(&import_button);

        let main_label = Label::new("<b>Chanel videos</b>");
        main_label.set_use_markup(true);
        vbox_content_pane.add(&main_label);

        hbox.add(&vbox_side_pane);
        let sep = Separator::new(Vertical);
        hbox.add(&sep);
        hbox.add(&vbox_content_pane);

        // GTK+ widgets are used normally within a `Widget`.
        let window = Window::new(WindowType::Toplevel);
        window.set_title("YT Video Subscriber");
        window.set_default_size(800, 600);
        window.add(&hbox);

        // Connect the signal `delete_event` to send the `Quit` message.
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );
        // There is also a `connect!()` macro for GTK+ events that do not need a
        // value to be returned in the callback.

        window.show_all();

        Win {
            model,
            widgets: Widgets {
                import_button,
                window: window,
            },
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
