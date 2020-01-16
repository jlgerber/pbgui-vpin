use pbgui_vpin::vpin_dialog;
use pbgui_vpin::vpin_dialog::LevelMap;
use qt_core::{QString, Slot, SlotOfInt};
use qt_thread_conductor::{conductor::Conductor, qt_utils::qs, traits::*};
use qt_widgets::QApplication;
use qt_widgets::{QMainWindow, QPushButton};
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::spawn;

#[derive(Debug)]
pub enum Msg {
    NewJokeRequest,
    Quit,
}
use qt_thread_conductor::traits::*;
use qt_widgets::cpp_core::{CppBox, Ref};

#[derive(Debug, PartialEq)]
pub enum Event {
    DbJokeUpdate,
    DbPunchlineUpdate,
}

const DDJOKEUPDATE: &'static str = "DbJokeUpdate";
const DDPUNCHLINEUPDATE: &'static str = "DbPunchlineUpdate";

impl ToQString for Event {
    fn to_qstring(&self) -> CppBox<QString> {
        match &self {
            &Event::DbPunchlineUpdate => QString::from_std_str(DDPUNCHLINEUPDATE),
            &Event::DbJokeUpdate => QString::from_std_str(DDJOKEUPDATE),
        }
    }
}

impl FromQString for Event {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match qs.to_std_string().as_str() {
            DDJOKEUPDATE => Event::DbJokeUpdate,
            DDPUNCHLINEUPDATE => Event::DbPunchlineUpdate,
            _ => panic!("Unable to convert to Event"),
        }
    }
}

fn main() {
    let mut handles = Vec::new();
    // sender, receiver for communicating from secondary thread to primary ui thread
    let (sender, receiver) = channel();
    // sender and receiver for communicating from ui thread to secondary thread
    let (to_thread_sender, to_thread_receiver): (Sender<Msg>, Receiver<Msg>) = channel();
    // sender to handle quitting
    let to_thread_sender_quit = to_thread_sender.clone();
    let quit_slot = Slot::new(move || {
        to_thread_sender_quit
            .send(Msg::Quit)
            .expect("couldn't send");
    });

    QApplication::init(|_app| unsafe {
        let mut main = QMainWindow::new_0a();
        let mut main_ptr = main.as_mut_ptr();
        let mut button = QPushButton::new();
        let button_ptr = button.as_mut_ptr();
        main.set_central_widget(button.into_ptr());

        let dialog = Rc::new(vpin_dialog::VpinDialog::create(
            "modelpublish-1.2.0",
            main_ptr,
        ));
        dialog.set_default_stylesheet();
        dialog.set_roles(vec![
            "anim", "integ", "model", "fx", "cfx", "light", "comp", "roto",
        ]);
        let levelmap = initialize_levelmap();
        dialog.set_levels_map(levelmap);
        dialog.set_levels_alt();
        dialog.set_sites(vec!["hyderabad", "montreal", "playa", "vancouver"]);
        let finished_slot = SlotOfInt::new(move |result: std::os::raw::c_int| {
            println!("result {}", result);
        });

        dialog.finished().connect(&finished_slot);

        let dialog_c = dialog.clone();
        // we need to create a slot that is triggered when OK is presswed
        let accepted_slot = Slot::new(move || {
            println!("accepted slot");
            if let Some(roles) = dialog_c.selected_roles() {
                println!("roles: {:?}", roles);
            } else {
                println!("roles: any");
            }
            if let Some(selected_level) = dialog_c.selected_level() {
                println!("level override: {:?}", selected_level);
            }
            println!("calling accept");
            dialog_c.accept();
        });

        // here is where we can cheat. We rely on the fact that
        // dialogb will outlive the borrow as mutable. We pass thiw
        // in to the pressed slot.
        dialog.accepted().connect(&accepted_slot);
        //dialog.set_roles_focus();
        //let mut dialogb = dialog.dialog.as_mut_ptr();
        let mut dialogb = dialog.dialog_mut();

        let exec_dialog_slot = Slot::new(move || {
            let result = dialogb.exec(); //
            println!("result {}", result);
        });

        button_ptr.pressed().connect(&exec_dialog_slot);
        main_ptr.show();
        QApplication::exec()
    });
}

fn initialize_levelmap() -> LevelMap {
    let mut lm = LevelMap::new();
    lm.insert(
        "RD".to_string(),
        vec![
            "0001".to_string(),
            "0002".to_string(),
            "0003".to_string(),
            "9999".to_string(),
        ],
    );
    lm.insert(
        "AA".to_string(),
        vec![
            "0001".to_string(),
            "0002".to_string(),
            "0003".to_string(),
            "0004".to_string(),
        ],
    );
    lm
}
