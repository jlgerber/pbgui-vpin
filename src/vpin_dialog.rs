use crate::inner_vpin_dialog::InnerVpinDialog;
pub use crate::inner_vpin_dialog::LevelMap;
use qt_core::{Signal, SlotOfQString};
use qt_widgets::{
    cpp_core::{CastInto, MutPtr, Ptr},
    QDialog, QWidget,
};
use std::cell::RefCell;

pub struct VpinDialog<'a> {
    dialog: RefCell<InnerVpinDialog<'a>>,
    seq_changed: SlotOfQString<'a>,
}

impl<'a> VpinDialog<'a> {
    pub unsafe fn create(distribution: &str, parent: impl CastInto<MutPtr<QWidget>>) -> VpinDialog {
        let inner_vpin_dialog = RefCell::new(InnerVpinDialog::create(distribution, parent));
        let dialog = VpinDialog {
            dialog: inner_vpin_dialog,
        };
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

    pub unsafe fn finished(&self) -> Signal<(std::os::raw::c_int,)> {
        self.dialog.borrow().finished()
    }
    /// Get a ponter to the dialog
    pub fn dialog(&self) -> Ptr<QDialog> {
        unsafe { self.dialog.borrow().dialog() }
    }

    /// Get a mutable pointer to the dialog
    pub fn dialog_mut(&self) -> MutPtr<QDialog> {
        unsafe { self.dialog.borrow_mut().dialog_mut() }
    }

    /// Return the rejected signal
    pub unsafe fn rejected(&self) -> Signal<()> {
        self.dialog.borrow().rejected()
    }

    /// Return a lsit of selected item names
    pub unsafe fn selected_roles(&self) -> Vec<String> {
        self.dialog.borrow().selected_roles()
    }

    /// Retrieve the current site
    pub unsafe fn selected_site(&self) -> String {
        self.dialog.borrow().selected_site()
    }

    /// Return the selected Sequence/shot if applicable
    pub unsafe fn selected_level(&self) -> Option<String> {
        None
    }

    /// Load the stylesheet
    pub unsafe fn set_default_stylesheet(&self) {
        self.dialog.borrow_mut().set_default_stylesheet();
    }

    /// Set the sites
    pub fn set_sites(&self, sites: Vec<&str>) {
        self.dialog.borrow_mut().set_sites(sites);
    }

    /// set the list of rols
    pub fn set_roles(&self, roles: Vec<&str>) {
        self.dialog.borrow_mut().set_roles(roles);
    }

    /// Given a new LevelMap, repalace the existing one
    pub fn set_levels(&self, levels: LevelMap) {
        self.dialog.borrow_mut().set_levels(levels);
    }
}
