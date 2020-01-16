use crate::inner_vpin_dialog::InnerVpinDialog;
pub use crate::inner_vpin_dialog::LevelMap;
use crate::inner_vpin_dialog::{DEFAULT_SEQ, DEFAULT_SHOT};
use qt_core::{QString, Signal, SlotOfQString};
use qt_widgets::{
    cpp_core::{CastInto, MutPtr, Ptr, Ref},
    QDialog, QWidget,
};
use rustqt_utils::qs;
use std::cell::RefCell;
use std::rc::Rc;

pub struct VpinDialog<'a> {
    dialog: Rc<RefCell<InnerVpinDialog<'a>>>,
    seq_changed: SlotOfQString<'a>,
    shot_changed: SlotOfQString<'a>,
}

impl<'a> VpinDialog<'a> {
    pub unsafe fn create(
        show_name: &str,
        distribution: &str,
        parent: impl CastInto<MutPtr<QWidget>>,
    ) -> VpinDialog {
        let inner_vpin_dialog = Rc::new(RefCell::new(InnerVpinDialog::create(
            show_name,
            distribution,
            parent,
        )));
        let mut shots_cbox = inner_vpin_dialog.clone().borrow_mut().shots_cbox();

        let seq_changed = SlotOfQString::new(move |idx: Ref<QString>| {
            let sequence = idx.to_std_string();
            println!("seq changed {}", sequence);
            shots_cbox.clear();
            shots_cbox.add_item_q_string(&qs(DEFAULT_SHOT));
            // if let Some(shots) = levels.get(sequence.as_str()) {
            //     for shot in shots {
            //         shots_cbox.add_item_q_string(&qs(shot));
            //     }
            // }
        });
        let shot_changed = SlotOfQString::new(|idx: Ref<QString>| {
            println!("shot changed {}", idx.to_std_string());
        });
        let dialog = VpinDialog {
            dialog: inner_vpin_dialog.clone(),
            seq_changed,
            shot_changed,
        };
        //Set up signals / slots
        dialog
            .current_seq_index_changed()
            .connect(&dialog.seq_changed);

        dialog
            .current_shot_index_changed()
            .connect(&dialog.shot_changed);

        // dialog
        //     .seqs_cb()
        //     .current_index_changed2()
        //     .connect(&dialog.seq_changed);
        dialog
    }

    /// Return the accepted signal from the button. This is provided as a convenience
    /// for hooking up a slot from this struct.
    pub unsafe fn accepted(&self) -> Signal<()> {
        self.dialog.borrow().accepted()
    }

    /// Dismiss the dialog using accept. This is a convenience for consumrs
    /// of this struct, to avoid having to drill down
    pub unsafe fn accept(&mut self) {
        self.dialog.borrow_mut().accept()
    }

    pub unsafe fn finished(&self) -> Signal<(std::os::raw::c_int,)> {
        self.dialog.borrow().finished()
    }
    /// Get a ponter to the dialog
    pub fn dialog(&self) -> Ptr<QDialog> {
        self.dialog.borrow().dialog()
    }

    /// Get a mutable pointer to the dialog
    pub fn dialog_mut(&mut self) -> MutPtr<QDialog> {
        self.dialog.borrow_mut().dialog_mut()
    }

    /// Return the rejected signal
    pub unsafe fn rejected(&self) -> Signal<()> {
        self.dialog.borrow().rejected()
    }

    /// Return a lsit of selected item names
    /// ption<
    pub unsafe fn selected_roles(&self) -> Option<Vec<String>> {
        self.dialog.borrow().selected_roles()
    }

    /// Retrieve the current site
    pub unsafe fn selected_site(&self) -> String {
        self.dialog.borrow().selected_site()
    }

    /// Return the selected Sequence/shot if applicable
    pub unsafe fn selected_level(&self) -> Option<String> {
        let show = self.dialog.borrow().show_name().to_string();
        if let Some(sequence) = self.dialog.borrow().selected_seq() {
            if let Some(shot) = self.dialog.borrow().selected_shot() {
                return Some(format!("{}.{}.{}", show, sequence, shot));
            }
            return Some(format!("{}.{}", show, sequence));
        }
        None
    }

    /// Load the stylesheet
    pub unsafe fn set_default_stylesheet(&mut self) {
        self.dialog.borrow_mut().set_default_stylesheet();
    }

    /// Set the sites
    pub fn set_sites(&mut self, sites: Vec<&str>) {
        self.dialog.borrow_mut().set_sites(sites);
    }

    /// set the list of rols
    pub fn set_roles(&mut self, roles: Vec<&str>) {
        self.dialog.borrow_mut().set_roles(roles);
    }
    /// Given a new LevelMap, repalace the existing one
    pub fn set_levels(&mut self, levels: LevelMap) {
        self.dialog.borrow_mut().set_levels(levels);
    }
    ///
    pub unsafe fn current_seq_index_changed(&self) -> Signal<(*const QString,)> {
        self.dialog.borrow().current_seq_index_changed()
    }
    pub unsafe fn current_shot_index_changed(&self) -> Signal<(*const QString,)> {
        self.dialog.borrow().current_shot_index_changed()
    }
}
