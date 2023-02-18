use gtk::prelude::{
	BoxExt, ButtonExt, GtkWindowExt, TextBufferExt, TextViewExt,
};
use gtk::{glib::clone, traits::WidgetExt};
use relm4::{
	gtk, tokio::runtime::TryCurrentError, ComponentParts, ComponentSender,
	RelmApp, RelmWidgetExt, SimpleComponent,
};

// Since I can't seem to get the overlay working, I will focus
// first on showing a text in one textview
// And in the other textview I will type the text
// And in a label i will show the correct or incorrect
// That way I can at least have the process of comparing
// each character and keypress
// By using Pango it is possible to change the color of
// characters with start_iter / end_iter and Pango tags

const TEXT: &str = "Trying to create an overlay for a TextView";
const TEXT2: &str = "Testing to see how to show two texts";

struct AppModel {
	base_text: gtk::TextBuffer,
	overlay_text: gtk::TextBuffer,
	cursor_index: u16,
	keypress_result: bool,
}

#[derive(Debug)]
enum AppInput {
	// Define the Messages
	CheckChar(u16),
}

struct AppWidgets {
	// We can leave the label for now
	base_textview: gtk::TextView,
	overlay_textview: gtk::TextView,
}

impl SimpleComponent for AppModel {
	type Input = AppInput;
	type Output = ();
	type Init = AppModel;
	type Root = gtk::Window;
	type Widgets = AppWidgets;

	fn init_root() -> Self::Root {
		gtk::Window::builder()
			.title("TextView Layered")
			.default_width(500)
			.default_height(500)
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
			overlay_text: model.overlay_text,
			cursor_index: model.cursor_index,
			keypress_result: model.keypress_result,
		};
		// Start the cursor at 0
		model.cursor_index = 0;
		model.keypress_result = true;

		// Push the initial TEXT to the base_text
		model.base_text.set_text(TEXT);
		model.overlay_text.set_text(TEXT2);

		// Build the UI
		let vbox = gtk::Box::builder()
			.orientation(gtk::Orientation::Vertical)
			.spacing(5)
			.build();

		let overlay_textview = gtk::TextView::builder()
			.editable(true)
			.height_request(50)
			.wrap_mode(gtk::WrapMode::Word)
			.buffer(&model.overlay_text)
			.opacity(0.3)
			.build();

		let base_textview = gtk::TextView::builder()
			.editable(true)
			.height_request(50)
			.wrap_mode(gtk::WrapMode::Word)
			.opacity(0.1)
			.buffer(&model.base_text)
			.build();
		base_textview.add_overlay(&overlay_textview, 100, 100);

		let label_index = gtk::Label::builder().label("The current index").build();

		// Overlay parent
		// Overlay child
		// Same position

		// let overlayer = gtk::Overlay::builder().build();
		// overlayer.set_parent(&base_textview);
		// overlayer.set_opacity(0.50);
		// // overlayer.add_overlay(&overlay_textview);
		// overlayer.set_clip_overlay(&overlay_textview, false);
		// // overlayer.set_child(Some(&overlay_textview));

		window.set_child(Some(&vbox));
		vbox.set_margin_all(5);
		vbox.append(&base_textview);
		vbox.append(&label_index);
		// vbox.append(&base_textview);
		// vbox.append(&overlay_textview);

		// inc_button.connect_clicked(clone!(@strong sender => move |_| {
		// sender.input(AppInput::Increment);
		// }));

		let widgets = AppWidgets {
			base_textview,
			overlay_textview,
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
		overlay_text: gtk::TextBuffer::new(None),
		cursor_index: 0,
		keypress_result: true,
	});
}
