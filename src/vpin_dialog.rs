use crate::inner_vpin_dialog::InnerVpinDialog;
pub use crate::inner_vpin_dialog::LevelMap;
use qt_core::{QString, Signal, SlotOfQString};
use qt_widgets::{
    cpp_core::{CastInto, MutPtr, Ptr, Ref},
    QComboBox, QDialog, QWidget,
};
use std::cell::RefCell;
use std::rc::Rc;

/// VpinDialog combines an InnerVpinDialog instance, which it exposes through
/// an immutable interace using the Rusty Interior Mutability Pattern, along with
/// a Slot. This nested construction serves two purposes - limit the
///  periods of mutability, and provide a convenient interface for Slots.
///
/// On the second point, we would not be able to call any methods on the InnerVpinDialog
/// if we didnt split up the impl between the core qt hierarchy and slots.
pub struct VpinDialog<'a> {
    dialog: Rc<RefCell<InnerVpinDialog<'a>>>,
    seq_changed: SlotOfQString<'a>,
}

impl<'a> VpinDialog<'a> {
    pub unsafe fn create<I: Into<String>>(
        show: I,
        distribution: &str,
        parent: impl CastInto<MutPtr<QWidget>>,
    ) -> VpinDialog {
        let inner_vpin_dialog = Rc::new(RefCell::new(InnerVpinDialog::create(
            show,
            distribution,
            parent,
        )));
        let ivd = inner_vpin_dialog.clone();
        let seq_changed = SlotOfQString::new(move |idx: Ref<QString>| {
            let sequence = idx.to_std_string();
            ivd.borrow().set_shots_for_seq(sequence.as_str());
        });
        let dialog = VpinDialog {
            dialog: inner_vpin_dialog,
            seq_changed,
        };
        dialog
            .seqs_cb()
            .current_index_changed2()
            .connect(&dialog.seq_changed);
        dialog
    }

    /// Return the accepted signal from the button. This is provided as a convenience
    /// for hooking up a slot from this struct.
    pub unsafe fn accepted(&self) -> Signal<()> {
        self.dialog.borrow().accepted()
    }

    /// Dismiss the dialog using accept. This is a convenience for consumrs
    /// of this struct, to avoid having to drill down
    pub unsafe fn accept(&self) {
        self.dialog.borrow_mut().accept()
    }

    /// Return the finished Signal so that connections to slots
    /// may be made
    pub unsafe fn finished(&self) -> Signal<(std::os::raw::c_int,)> {
        self.dialog.borrow().finished()
    }

    /// Get a pointer to the dialog
    pub fn dialog(&self) -> Ptr<QDialog> {
        self.dialog.borrow().dialog()
    }

    /// Get a mutable pointer to the dialog
    pub fn dialog_mut(&self) -> MutPtr<QDialog> {
        self.dialog.borrow_mut().dialog_mut()
    }

    /// Return the rejected signal
    pub unsafe fn rejected(&self) -> Signal<()> {
        self.dialog.borrow().rejected()
    }

    /// Return a lsit of selected item names
    pub unsafe fn selected_roles(&self) -> Option<Vec<String>> {
        self.dialog.borrow().selected_roles()
    }

    /// Retrieve the current site
    pub unsafe fn selected_site(&self) -> String {
        self.dialog.borrow().selected_site()
    }

    pub fn show_name(&self) -> String {
        self.dialog.borrow().show_name().to_string()
    }

    /// Return the selected Sequence/shot if applicable
    pub unsafe fn selected_level(&self) -> Option<String> {
        self.dialog.borrow().selected_level()
    }

    /// Load the stylesheet
    pub unsafe fn set_default_stylesheet(&self) {
        self.dialog.borrow_mut().set_default_stylesheet();
    }

    /// Set the sites
    pub fn set_sites(&self, sites: Vec<&str>) {
        self.dialog.borrow().set_sites(sites);
    }

    /// set the list of rols
    pub fn set_roles(&self, roles: Vec<&str>) {
        self.dialog.borrow().set_roles(roles);
    }

    /// Retrieve a mutable pointer to the sequences QComboBox
    pub fn seqs_cb(&self) -> MutPtr<QComboBox> {
        self.dialog.borrow().seqs_cb()
    }

    /// Given a new LevelMap, repalace the existing one
    pub fn set_levels_map(&self, levels: LevelMap) {
        self.dialog.borrow_mut().set_levels_map(levels);
    }

    /// Given a vector of Strings, set levels
    pub fn set_levels(&self, levels: Vec<String>) {
        //let levels = self.dialog.borrow().seqs();
        self.dialog.borrow().set_levels(levels);
    }

    pub fn set_levels_alt(&self) {
        self.dialog.borrow().set_levels_alt();
    }
}
