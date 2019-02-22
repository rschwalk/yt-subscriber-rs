extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use gtk::prelude::*;
use gtk::Orientation::Vertical;
use gtk::{Button, Inhibit, Label, Window, WindowType};
use relm::{Relm, Update, Widget};

struct Model {
    // …
}

#[derive(Msg)]
enum Msg {
    // …
    Quit,
}

#[derive(Clone)]
struct Widgets {
    counter_label: Label,
    minus_buttton: Button,
    plus_button: Button,
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
        let vbox = gtk::Box::new(Vertical, 0);

        let plus_button = Button::new_with_label("+");
        vbox.add(&plus_button);

        let counter_label = Label::new("0");
        vbox.add(&counter_label);

        let minus_buttton = Button::new_with_label("-");
        vbox.add(&minus_buttton);

        // GTK+ widgets are used normally within a `Widget`.
        let window = Window::new(WindowType::Toplevel);
        window.add(&vbox);

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
                counter_label,
                minus_buttton,
                plus_button,
                window: window,
            },
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
