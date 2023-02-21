// TODO: Adding green and red colors
// TODO: Connect cursor events and change the color of the lasttyped character
use gtk::prelude::{
	BoxExt, ButtonExt, GtkWindowExt, OrientableExt, TextBufferExt, TextViewExt,
};
use relm4::{
	gtk::{self, traits::WidgetExt},
	ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent,
};

const TEXT: &str = "Relm4 TextView Experiment";

#[derive(Debug, Default)]
struct AppModel {
	base_text: gtk::TextBuffer,
	tagtable: gtk::TextTagTable,
	info_label_text: String,
}

#[derive(Debug)]
enum AppMsg {
	CheckChar(u32),
	GoBack,
	LoadText,
}

#[relm4::component]
impl SimpleComponent for AppModel {
	type Init = ();
	type Input = AppMsg;
	type Output = ();

	view! {
		gtk::Window {
			set_title: Some("TextView Experiment"),
			set_default_width: 300,
			set_default_height: 300,

			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
				set_spacing: 5,
				set_margin_all: 5,

				gtk::TextView {
					#[watch]
					set_buffer: Some(&model.base_text),
					set_editable: true,
					set_wrap_mode: gtk::WrapMode::WordChar,
					set_margin_all: 5,
					set_focusable: true,
					set_cursor_visible: true,
					set_overwrite: true,
				},
				gtk::Label {
					#[watch]
					set_margin_all: 5,
					// BUG: The label isn't updated when it is changed
					set_label: &model.info_label_text,
				},
				gtk::Button {
					set_label: "Load Text",
					connect_clicked[sender] => move |_| {
					sender.input(AppMsg::LoadText);
				}
				}

			}
		}
	}

	fn init(
		_app_state: Self::Init,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		// let model = AppModel::default();
		// Create an instance of the AppModel App State
		let model = AppModel {
			base_text: gtk::TextBuffer::new(None),
			tagtable: gtk::TextTagTable::new(),
			info_label_text: String::new(),
		};

		// model.base_text.set_text(TEXT);
		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
		match msg {
			AppMsg::CheckChar(position) => {
				println!("The current curson_index: {}", position)
			},
			AppMsg::GoBack => {
				println!("Go one character back")
			},
			AppMsg::LoadText => {
				println!("Loading Text: {}", TEXT);
				self.info_label_text = String::from("Loaded");
				// BUG: I am trying to change the label but it isn't uploaded
				self.base_text.set_text(TEXT);
			},
		}
	}
}

fn main() {
	let app = RelmApp::new("relm4.textview.experiment");
	app.run::<AppModel>(())
}
