//
// Create a form with a Question TextInput and an Answer with a multi-line
// textarea.
//
// After pressing the save button a new MenuItem is created and pushed to the Vector
// holding all the MenuItems.
//
// Create a FactorVecDeque (Relm4) to show the MenuItem List
//
use gtk::prelude::{
	BoxExt, ButtonExt, EntryBufferExtManual, EntryExt, GtkWindowExt,
	OrientableExt,
};

use relm4::{
	gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent,
};

#[derive(Debug)]
struct MemItem {
	question: String,
}

#[derive(Debug, Default)]
struct AppModel {
	question: gtk::EntryBuffer,
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
			set_default_width: 400,
			set_default_height: 200,

			gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					set_spacing: 5,
					set_margin_all: 5,

					gtk::Entry {
						set_buffer: &model.question,
						set_placeholder_text: Some("Enter Question"),
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
				// check if the question is empty
				if !self.question.text().is_empty() {
					println!("The question: {:?}", self.question.text());
					self.list.push(MemItem {
						// Todo: Why is there an error here?
						question: self.question.text().to_string(),
					});
					println!("The list: {:#?}", self.list);
					// Clean up the input fields after pushing to list
					self.question.set_text("")
				} else {
					println!("The question cannot be empty");
					// here you can set the error message
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
