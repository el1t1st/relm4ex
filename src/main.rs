use gtk::prelude::{
	BoxExt, ButtonExt, EntryBufferExt, EntryBufferExtManual, EntryExt,
	GtkWindowExt, OrientableExt, TextViewExt,
};

use relm4::{
	gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent,
};

#[derive(Debug)]
struct MemItem {
	question: String,
	answer: String,
}

struct AppModel {
	question_to_save: String,
	answer_to_save: String,
	list: Vec<MemItem>,
}

#[derive(Debug)]
enum AppMsg {
	SaveQuestion(String),
	SaveAnswer(String),
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
				set_default_width: 400,
				set_default_height: 200,

				gtk::Box {
						set_orientation: gtk::Orientation::Vertical,
						set_spacing: 5,
						set_margin_all: 5,

						#[name ="new_question"]
						gtk::Entry {
								set_placeholder_text: Some("Enter Question"),
								connect_activate[sender] => move |entry| {
										// let buffer = entry.buffer();
										// if !buffer.text().is_empty() {
												// sender.output(AppMsg::SaveQuestion(buffer.text())).unwrap_or_default();
									println!("new question: {:#?}", entry);
									sender.input(AppMsg::SaveQuestion(String::from("TODO")));
									//
								}
						},
						#[name ="new_answer"]
						gtk::Entry {
								set_placeholder_text: Some("Enter Answer"),
								connect_activate[sender] => move |entry| {
									// let buffer = entry.buffer();
									println!("new_answer {:#?}", entry);
									sender.input(AppMsg::SaveAnswer(String::from("TOQuestion")));
								}
						},
						// #[name ="new_answer"]
						// gtk::TextView {
						// 		connect_activate[sender] => move |entry| {
						// 				let buffer = entry.buffer();
						// 				if !buffer.text().is_empty() {
						// 						// sender.output(AppMsg::SaveAnswer(buffer.text())).unwrap_or_default();
						// 					sender.input(AppMsg::SaveQuestion(buffer.text()));
						// 				}
						// 		}
						// },
						#[name ="save_button"]
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
		let model = AppModel {
			question_to_save: String::new(),
			answer_to_save: String::new(),
			list: Vec::new(),
		};

		// Insert the macro code generation here from view! macro
		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	// Update logics for saving
	fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
		match msg {
			AppMsg::SaveQuestion(text) => {
				self.question_to_save = text;
			},
			AppMsg::SaveAnswer(text) => {
				self.answer_to_save = text;
			},
			// TODO: Here we need to add a check ... if not self.answer_to_save is empty and not
			// self.question_to_save ... is empty then
			AppMsg::SaveMemItem => self.list.push(MemItem {
				// The issue!!! clone!
				question: self.question_to_save.clone(),
				answer: self.answer_to_save.clone(),
			}),
		}
	}
}

fn main() {
	let app = RelmApp::new("relm4.experiment");
	app.run::<AppModel>(AppModel {
		question_to_save: String::new(),
		answer_to_save: String::new(),
		list: Vec::new(),
	});
}
