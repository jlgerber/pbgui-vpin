use pbgui_vpin::vpin_dialog;
use pbgui_vpin::vpin_dialog::create_vlayout;
use qt_core::{Slot, SlotOfInt};
use qt_widgets::cpp_core::MutPtr;
use qt_widgets::QApplication;
use qt_widgets::QFrame;
use qt_widgets::{QMainWindow, QPushButton};

fn main() {
    QApplication::init(|_app| unsafe {
        let mut main = QMainWindow::new_0a();
        let mut main_ptr = main.as_mut_ptr();
        let mut button = QPushButton::new();
        let mut button_ptr = button.as_mut_ptr();
        main.set_central_widget(button.into_ptr());
        let pbslot = Slot::new(move || {
            let mut dialog = vpin_dialog::VpinDialog::create("modelpublish-1.2.0", main_ptr);
            dialog.set_roles(vec![
                "anim", "integ", "model", "fx", "cfx", "light", "comp", "roto",
            ]);
            dialog.set_sites(vec!["hyderabad", "montreal", "playa", "vancouver"]);
            let finished_slot = SlotOfInt::new(move |result: std::os::raw::c_int| {
                println!("result {}", result);
            });
            dialog.finished().connect(&finished_slot);
            let result = dialog.exec();
            println!("result {}", result);
            //dialog.open();
        });
        button_ptr.pressed().connect(&pbslot);
        main_ptr.show();
        QApplication::exec()
    });
}
