// TODO: Adding green and red colors
// TODO: Connect cursor events and change the color of the lasttyped character
use gtk::prelude::{
	BoxExt, ButtonExt, GtkWindowExt, OrientableExt, TextBufferExt, TextViewExt,
};
use gtk::TextTagTable;
use relm4::{
	gtk::{
		self,
		traits::{TextTagExt, WidgetExt},
	},
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
	ClearText,
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
					set_label: &format!("{}", model.info_label_text),
				},
				gtk::Button {
					set_label: "Load Text",
					connect_clicked[sender] => move |_| {
						sender.input(AppMsg::LoadText);
				}},
				gtk::Button {
					set_label: "Clear Text",
					connect_clicked[sender] => move |_| {
					sender.input(AppMsg::ClearText);
					}
				},
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

		// Create the color tags and add them to the tagtable.
		// 1) Create a tagtable
		// 2) Create the tags
		// 3) Add the tags to the tagtable
		//
		let green_tag = gtk::TextTag::new(Some("green"));
		green_tag.set_background(Some("green"));

		let red_tag = gtk::TextTag::new(Some("red"));
		red_tag.set_background(Some("red"));

		model.tagtable.add(&green_tag);
		model.tagtable.add(&red_tag);

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
				self.info_label_text = String::from("Loaded!");
				println!("Running the LoadText: {}", &self.info_label_text);
				self.base_text.set_text(TEXT);
			},
			AppMsg::ClearText => {
				self.info_label_text = String::from("Cleared!");
				self.base_text.set_text("");
				println!("Running ClearText!")
			},
		}
	}
}

fn main() {
	let app = RelmApp::new("relm4.textview.experiment");
	app.run::<AppModel>(())
}
