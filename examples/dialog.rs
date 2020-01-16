use pbgui_vpin::vpin_dialog;
use pbgui_vpin::vpin_dialog::LevelMap;
use qt_core::{Slot, SlotOfInt};
use qt_widgets::QApplication;
use qt_widgets::{QMainWindow, QPushButton};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
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
        //dialog.set_levels(vec!["AA".to_string(), "BB".to_string()]);
        dialog.set_sites(vec!["hyderabad", "montreal", "playa", "vancouver"]);
        let finished_slot = SlotOfInt::new(move |result: std::os::raw::c_int| {
            println!("result {}", result);
        });

        dialog.finished().connect(&finished_slot);

        let dialog_c = dialog.clone();
        // we need to create a slot that is triggered when OK is presswed
        let accepted_slot = Slot::new(move || {
            println!("accepted slot");
            let roles = dialog_c.selected_roles();
            println!("{:?}", roles);
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
