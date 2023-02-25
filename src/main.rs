use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, SimpleComponent};
use std::cell::Cell;
use std::rc::Rc;

struct AppModel {
	counter: Rc<Cell<i32>>,
}

#[derive(Debug)]
enum AppMsg {
	PlusOne,
	MinusOne,
}

struct AppWidgets {
	label: gtk::Label,
}

impl SimpleComponent for AppModel {
	type Init = i32;
	type Input = AppMsg;
	type Output = ();
	type Root = gtk::Window;
	type Widgets = AppWidgets;

	fn init_root() -> Self::Root {
		gtk::Window::builder()
			.title("Counter")
			.default_width(500)
			.default_height(400)
			.build()
	}

	fn init(
		_appstate: Self::Init,
		window: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = AppModel {
			counter: Rc::new(Cell::new(0)),
		};

		let vbox = gtk::Box::builder()
			.orientation(gtk::Orientation::Vertical)
			.build();

		let label = gtk::Label::builder()
			.label(&model.counter.get().to_string())
			.build();
		let button_plusone = gtk::Button::builder().label("Plus 1").build();
		let button_minusone = gtk::Button::builder().label("Minus 1").build();

		let sender_plusone = sender.clone();
		button_plusone.connect_clicked(move |_| {
			sender_plusone.input(AppMsg::PlusOne);
		});
		let sender_minusone = sender.clone();
		button_minusone.connect_clicked(move |_| {
			sender_minusone.input(AppMsg::MinusOne);
		});

		vbox.append(&label);
		vbox.append(&button_plusone);
		vbox.append(&button_minusone);

		window.set_child(Some(&vbox));

		let widgets = AppWidgets { label };

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		match message {
			AppMsg::PlusOne => {
				self.counter.set(self.counter.get() + 1);
			},
			AppMsg::MinusOne => {
				self.counter.set(self.counter.get() - 1);
			},
		}
	}

	fn update_view(
		&self,
		widgets: &mut Self::Widgets,
		_sender: ComponentSender<Self>,
	) {
		widgets.label.set_label(&self.counter.get().to_string());
	}
}

fn main() {
	let app = RelmApp::new("relm4.counter.vanilla");
	app.run::<AppModel>(0);
}
