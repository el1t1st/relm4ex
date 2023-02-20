use gtk::{
	prelude::{BoxExt, GtkWindowExt, TextBufferExt, TextTagExt, TextViewExt},
	traits::WidgetExt,
};
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

const TEXT: &str = "Trying to solve the issue in a different why by using TextView with Pango parsing and a keypress listener!";

struct AppModel {
	base_text: gtk::TextBuffer,
	cursor_index: u16,
	// css_provider: gtk::CssProvider,
	// display: gtk::gdk::Display,
	text_tag_table: gtk::TextTagTable,
}

#[derive(Debug)]
enum AppInput {
	// Define the Messages
	CheckChar,
	BackOneChar,
}

struct AppWidgets {
	// We can leave the label for now
	base_textview: gtk::TextView,
}

impl SimpleComponent for AppModel {
	type Input = AppInput;
	type Output = ();
	type Init = AppModel;
	type Root = gtk::Window;
	type Widgets = AppWidgets;

	fn init_root() -> Self::Root {
		gtk::Window::builder()
			.title("TextView With Pango Colors TextTagTable")
			.default_width(500)
			.default_height(500)
			.build()
	}

	fn init(
		_state: Self::Init,
		window: &Self::Root,
		_sender: ComponentSender<Self>,
	) -> relm4::ComponentParts<Self> {
		// Initialize the AppModel / state
		let mut model = AppModel {
			base_text: gtk::TextBuffer::new(None),
			cursor_index: 0,
			// css_provider: gtk::CssProvider::new(),
			// display: gtk::gdk::Display::default().unwrap(),
			text_tag_table: gtk::TextTagTable::new(),
		};
		// Activate CSS Provider
		// What does this do? And shouldn't we do this before the load_from_path?
		// model.css_provider.connect_parsing_error(|_, _, error| {
		// 	println!("{:?}", error);
		// });
		//

		// model.css_provider.load_from_path("test.css");

		// match ret {
		// 	Ok(ret) => println!("Worked? {:?}", ret),
		// 	Err(error) => println!("Worked? {:?}", error),
		// }

		// gtk::StyleContext::add_provider_for_display(
		// 	&model.display,
		// 	&model.css_provider,
		// 	gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
		// );

		model.base_text.set_text(TEXT);

		// create tag table
		model.text_tag_table = model.base_text.tag_table();
		// create tags
		let green_tag = gtk::TextTag::new(Some("green"));
		green_tag.set_background(Some("green"));

		let red_tag = gtk::TextTag::new(Some("red"));
		red_tag.set_background(Some("red"));

		// Add tags to tag_table
		model.text_tag_table.add(&green_tag);
		model.text_tag_table.add(&red_tag);

		// Once this works we can add more tags for font, fontsize, ...
		model.base_text.apply_tag(
			&red_tag,
			&model.base_text.start_iter(),
			&model.base_text.end_iter(),
		);

		model.base_text.place_cursor(&model.base_text.start_iter());

		// Build the UI
		let vbox = gtk::Box::builder()
			.orientation(gtk::Orientation::Vertical)
			.spacing(5)
			.build();

		let base_textview = gtk::TextView::builder()
			.focusable(true)
			.editable(true)
			.overwrite(true) // The cursor needs to overwrite
			.cursor_visible(true)
			.wrap_mode(gtk::WrapMode::WordChar)
			.buffer(&model.base_text)
			.build();

		window.set_child(Some(&vbox));
		vbox.set_margin_all(10);
		vbox.append(&base_textview);

		// get the bounds
		// set the character to the first char
		// listen to keyevents
		// GTK keyboard events listener

		let widgets = AppWidgets { base_textview };
		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		match message {
			AppInput::CheckChar => {
				// get the &model.current_index
				// take the string[current_index]
				// get current_keystroke
				// if == add_tag green else add_tag_red
				// model.current_index.wrapping_add(1);
				println!("Checking character");
			},
			AppInput::BackOneChar => {
				// remove tag from the character with current_index -1
				// &model.current_index.wrapping_sub(1);
				println!("One character back");
			},
		}
	}

	fn update_view(
		&self,
		_widgets: &mut Self::Widgets,
		_sender: ComponentSender<Self>,
	) {
		// Update the view
		// the model.text_base needs to update but ...?
	}
}

fn main() {
	let app = RelmApp::new("relm4.experiment");
	app.run::<AppModel>(AppModel {
		base_text: gtk::TextBuffer::new(None),
		cursor_index: 0,
		// css_provider: gtk::CssProvider::new(),
		text_tag_table: gtk::TextTagTable::new(),
		// display: gtk::gdk::Display::default().unwrap(),
	});
}
