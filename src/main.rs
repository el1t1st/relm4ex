//
// Create a form with a Question TextInput and an Answer with a multi-line
// textarea.
//
// After pressing the save button a new MenuItem is created and pushed to the Vector
// holding all the MenuItems.
//
// Create a FactorVecDeque (Relm4) to show the MenuItem List
//
use gtk::{
	prelude::{
		BoxExt, ButtonExt, EntryBufferExtManual, EntryExt, GtkWindowExt,
		OrientableExt, TextBufferExt, TextViewExt,
	},
	traits::WidgetExt,
	TextTagTable,
};

use relm4::{
	gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent,
};

#[derive(Debug)]
struct MemItem {
	question: String,
	answer: String,
}

#[derive(Debug, Default)]
struct AppModel {
	question: gtk::EntryBuffer,
	answer: gtk::TextBuffer,
	list: Vec<MemItem>,
}

#[derive(Debug)]
enum AppMsg {
	SaveMemItem,
}

#[relm4::component]
impl SimpleComponent for AppModel {
	type Widgets = AppWidgets;
	type Init = AppModel;

	type Input = AppMsg;
	type Output = ();

	view! {
	gtk::Window {
			set_title: Some("The Relm Experiment"),
			set_default_width: 700,
			set_default_height: 500,

			gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					set_spacing: 5,
					set_margin_all: 5,

					gtk::Entry {
						set_buffer: &model.question,
						set_placeholder_text: Some("What is the Question"),
					},
					gtk::Label {
						set_halign: gtk::Align::Start,
						set_label: "What is the the answer?:",
						set_margin_all: 5,
				},
					gtk::TextView {
						set_margin_all: 5,
						set_height_request: 100,
						set_buffer: Some(&model.answer),
						set_editable: true,
						set_wrap_mode: gtk::WrapMode::Word,
					},

					gtk::Button {
							set_label: "Save",
							connect_clicked[sender] => move |_| {
									sender.input(AppMsg::SaveMemItem);
							}
					}
			}
		}
	}

	// Initialize the UI
	fn init(
		_app_state: Self::Init,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = AppModel::default();

		// Insert the macro code generation here from view! macro
		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	// Update logics for saving
	fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
		match msg {
			AppMsg::SaveMemItem => {
				let question = self.question.text();
				println!("Loaded question: {:#?}", question);
				let answer = self
					.answer
					.text(&self.answer.start_iter(), &self.answer.end_iter(), true)
					.to_string();
				println!("Loaded TextBuffer: {:#?}", answer);
				// check if both fields are not empty before saving
				if !answer.is_empty() && !question.is_empty() {
					self.list.push(MemItem { question, answer });
					println!("The updated list: {:#?}", self.list);
				}
			},
		}
	}
}

fn main() {
	let app = RelmApp::new("relm4.experiment");
	// Todo: why do we need to pass it a second time although we have already defined it in fn init?
	let model = AppModel::default();
	app.run::<AppModel>(model);
}
