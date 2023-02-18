use gtk::{
	prelude::{
		BoxExt, GtkApplicationExt, GtkWindowExt, TextBufferExt,
		TextBufferExtManual, TextTagExt, TextViewExt,
	},
	traits::WidgetExt,
};
use pango::*;
use relm4::{
	gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent,
};

// Since I can't seem to get the overlay working, I will focus
// first on showing a text in one textview
// And in the other textview I will type the text
// And in a label i will show the correct or incorrect
// That way I can at least have the process of comparing
// each character and keypress
// By using Pango it is possible to change the color of
// characters with start_iter / end_iter and Pango tags

const TEXT: &str = "Trying to solve the issue in a 
	\n different why by using \n 
	TextView with Pango parsing and a keypress listener!";

struct AppModel {
	base_text: gtk::TextBuffer,
	cursor_index: u16,
	keypress_result: bool,
	// pango_attrlist: pango::AttrList,
}

#[derive(Debug)]
enum AppInput {
	// Define the Messages
	CheckChar,
}

struct AppWidgets {
	// We can leave the label for now
	base_textview: gtk::TextView,
	label_index: gtk::Label,
	label_keystroke: gtk::Label,
}

impl SimpleComponent for AppModel {
	type Input = AppInput;
	type Output = ();
	type Init = AppModel;
	// ApplicationWindow supports connect for listening to keypresses
	type Root = gtk::Window;
	type Widgets = AppWidgets;

	fn init_root() -> Self::Root {
		gtk::Window::builder()
			.title("TextView Layered")
			.default_width(500)
			.default_height(500)
			// Adding a listener for key-presses
			.build()
	}

	fn init(
		model: Self::Init,
		window: &Self::Root,
		_sender: ComponentSender<Self>,
	) -> relm4::ComponentParts<Self> {
		// Initialize the AppModel / state
		let mut model = AppModel {
			base_text: model.base_text,
			cursor_index: model.cursor_index,
			keypress_result: model.keypress_result,
		};
		// Start the cursor at 0
		model.cursor_index = 0;
		model.keypress_result = true;

		// Push the initial TEXT to the base_text
		model.base_text.set_text(TEXT);
		// todo: fix this
		let tag = model
			.base_text
			.tag_table()
			.create_tag(Some("orange_bg"), bgcolor = "orange");
		model.base_text.apply_tag(
			tag,
			&model.base_text.start_iter(),
			&model.base_text.end_iter(),
		);

		// Build the UI
		let vbox = gtk::Box::builder()
			.orientation(gtk::Orientation::Vertical)
			.spacing(5)
			.build();

		let base_textview = gtk::TextView::builder()
			// To connect the keypresses to the base_textview widget we need to set it focusable
			.focusable(true)
			.editable(true)
			.height_request(10)
			.wrap_mode(gtk::WrapMode::Word)
			.buffer(&model.base_text)
			// .opacity(0.3)
			.build();

		let label_keystroke = gtk::Label::builder()
			.label("The current keystroke:")
			.build();
		let label_index = gtk::Label::builder().label("The current index").build();

		window.set_child(Some(&vbox));
		vbox.set_margin_all(5);
		vbox.append(&base_textview);
		vbox.append(&label_index);
		vbox.append(&base_textview);

		// listen to keyevents
		// GTK keyboard events listener

		let widgets = AppWidgets {
			base_textview,
			label_keystroke,
			label_index,
		};
		ComponentParts { model, widgets }
	}

	fn update(&mut self, _message: Self::Input, _sender: ComponentSender<Self>) {
		// match message {
		// 	AppInput::Increment => {
		// 		self.counter = self.counter.wrapping_add(1);
		// 	},
		// 	AppInput::Decrement => {
		// 		self.counter = self.counter.wrapping_sub(1);
		// 	},
		// }
	}

	fn update_view(
		&self,
		_widgets: &mut Self::Widgets,
		_sender: ComponentSender<Self>,
	) {
		// Update the view
	}
}

fn main() {
	let app = RelmApp::new("relm4.test");
	app.run::<AppModel>(AppModel {
		base_text: gtk::TextBuffer::new(None),
		cursor_index: 0,
		keypress_result: true,
	});
}
