use crate::inner_vpin_dialog::InnerVpinDialog;
pub use crate::inner_vpin_dialog::LevelMap;
use qt_core::Signal;
use qt_widgets::{
    cpp_core::{CastInto, MutPtr, Ptr},
    QDialog, QWidget,
};
pub struct VpinDialog<'a> {
    dialog: InnerVpinDialog<'a>,
}

impl<'a> VpinDialog<'a> {
    pub unsafe fn create(distribution: &str, parent: impl CastInto<MutPtr<QWidget>>) -> VpinDialog {
        let inner_vpin_dialog = InnerVpinDialog::create(distribution, parent);

        let dialog = VpinDialog {
            dialog: inner_vpin_dialog,
        };
        dialog
    }

    /// Return the accepted signal from the button. This is provided as a convenience
    /// for hooking up a slot from this struct.
    pub unsafe fn accepted(&self) -> Signal<()> {
        self.dialog.accepted()
    }

    /// Dismiss the dialog using accept. This is a convenience for consumrs
    /// of this struct, to avoid having to drill down
    pub unsafe fn accept(&mut self) {
        self.dialog.accept()
    }

    pub unsafe fn finished(&self) -> Signal<(std::os::raw::c_int,)> {
        self.dialog.finished()
    }
    /// Get a ponter to the dialog
    pub fn dialog(&self) -> Ptr<QDialog> {
        unsafe { self.dialog.dialog() }
    }

    /// Get a mutable pointer to the dialog
    pub fn dialog_mut(&mut self) -> MutPtr<QDialog> {
        unsafe { self.dialog.dialog_mut() }
    }

    /// Return the rejected signal
    pub unsafe fn rejected(&self) -> Signal<()> {
        self.dialog.rejected()
    }

    /// Return a lsit of selected item names
    pub unsafe fn selected_roles(&self) -> Vec<String> {
        self.dialog.selected_roles()
    }

    /// Retrieve the current site
    pub unsafe fn selected_site(&self) -> String {
        self.dialog.selected_site()
    }

    /// Return the selected Sequence/shot if applicable
    pub unsafe fn selected_level(&self) -> Option<String> {
        None
    }

    /// Load the stylesheet
    pub unsafe fn set_default_stylesheet(&mut self) {
        self.dialog.set_default_stylesheet();
    }

    /// Set the sites
    pub fn set_sites(&mut self, sites: Vec<&str>) {
        self.dialog.set_sites(sites);
    }

    /// set the list of rols
    pub fn set_roles(&mut self, roles: Vec<&str>) {
        self.dialog.set_roles(roles);
    }

    /// Given a new LevelMap, repalace the existing one
    pub fn set_levels(&mut self, levels: LevelMap) {
        self.dialog.set_levels(levels);
    }
}
