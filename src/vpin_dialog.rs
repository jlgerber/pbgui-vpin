//! The DistributionDialog allows the user to generate one or more pins for a distribution
use qt_core::{Signal, Slot};
use qt_widgets::{
    cpp_core::{CastInto, CppBox, DynamicCast, MutPtr},
    q_abstract_item_view::SelectionMode,
    q_dialog_button_box::StandardButton,
    QComboBox, QDialog, QDialogButtonBox, QFrame, QGroupBox, QHBoxLayout, QLabel, QLayout,
    QLineEdit, QListWidget, QVBoxLayout, QWidget,
};

pub use rustqt_utils::{create_hlayout, create_vlayout, qs};

pub struct VpinDialog<'a> {
    dialog: CppBox<QDialog>,
    roles_filter: MutPtr<QLineEdit>,
    roles_list: MutPtr<QListWidget>,
    sites_cbox: MutPtr<QComboBox>,
    buttons: MutPtr<QDialogButtonBox>,
    accepted: Slot<'a>,
}
//Option<MutPtr<QWidget>>
impl<'a> VpinDialog<'a> {
    /// Create a new VpinDialog
    pub fn create<'b: 'a>(
        distribution: &str,
        parent: impl CastInto<MutPtr<QWidget>>,
    ) -> VpinDialog {
        unsafe {
            let mut dialog = QDialog::new_1a(parent);
            //dialog.set_window_title(&qs("Add Version-Pin"));
            let mut layout = create_vlayout();
            let mut layout_ptr = layout.as_mut_ptr();
            Self::add_entry_label(layout_ptr);
            Self::add_distribution_label(distribution, layout_ptr);

            // hlayout will contain the two column  vertical layouts (left and right)
            let mut hlayout = create_hlayout();
            let hlayout_ptr = hlayout.as_mut_ptr();
            layout_ptr.add_layout_1a(hlayout.into_ptr());

            let left_layout = Self::add_left_layout(hlayout_ptr);
            let group_box = Self::add_select_roles_groupbox(left_layout);

            let roles_filter = Self::add_roles_filter(group_box.layout());
            let roles_list = Self::add_roles_listwidget(group_box.layout());
            let roles_list_cpy = roles_list.as_ptr();
            let mut right_layout = Self::add_right_layout(hlayout_ptr);
            let seq_shot_group_box = Self::add_select_level_groupbox(right_layout);
            let seqs_cbox = Self::add_seq_cbox(seq_shot_group_box.layout());
            let shots_cbox = Self::add_shot_cbox(seq_shot_group_box.layout());

            let mut sel_sites_group_box = Self::add_select_site_groupbox(right_layout);
            let sites_cbox = Self::add_site_cbox(sel_sites_group_box.layout());
            right_layout.add_stretch_1a(1);
            layout.add_stretch_1a(1);

            let mut button_box = QDialogButtonBox::from_q_flags_standard_button(
                StandardButton::Ok | StandardButton::Cancel,
            );
            let buttons = button_box.as_mut_ptr();
            layout.add_widget(button_box.into_ptr());
            dialog.set_layout(layout.into_ptr());
            dialog.set_modal(true);
            let mut dialog_cpy = dialog.as_mut_ptr();

            let accepted = Slot::new(move || {
                let mut items = roles_list_cpy.selected_items();
                if !items.is_empty() {
                    for _c in 0..items.length() {
                        let item = items.take_first();
                        println!("{}", item.text().to_std_string());
                    }
                }
                println!("calling accept");

                dialog_cpy.accept();
            });
            let dialog = VpinDialog {
                dialog,
                roles_filter,
                roles_list,
                sites_cbox,
                buttons,
                accepted,
            };
            buttons.accepted().connect(&dialog.accepted);
            buttons.rejected().connect(dialog.dialog.slot_reject());
            dialog
        }
    }

    /// Return the accepted signal from the button
    pub unsafe fn accepted(&self) -> Signal<()> {
        self.buttons.accepted()
    }

    /// Dismiss the dialog using accept
    pub unsafe fn accept(&mut self) {
        self.dialog.accept()
    }

    /// Return the rejected signal
    pub unsafe fn rejected(&self) -> Signal<()> {
        self.buttons.rejected()
    }
    /// Return a lsit of selected item names
    pub unsafe fn selected_roles(&self) -> Vec<String> {
        let mut results = Vec::new();
        let mut items = self.roles_list.selected_items();
        if !items.is_empty() {
            for _c in 0..items.length() {
                let item = items.take_first();
                results.push(item.text().to_std_string());
            }
        }
        results
    }

    /// Retrieve the current site
    pub unsafe fn selected_site(&self) -> String {
        self.sites_cbox.current_text().to_std_string()
    }

    /// Return the selected Sequence/shot if applicable
    pub unsafe fn selected_level(&self) -> Option<String> {
        None
    }

    /// Set the sites
    pub fn set_sites(&mut self, sites: Vec<&str>) {
        unsafe {
            self.sites_cbox.clear();
            for site in sites {
                self.sites_cbox.add_item_q_string(&qs(site));
            }
        }
    }

    /// set the list of rols
    pub fn set_roles(&mut self, roles: Vec<&str>) {
        unsafe {
            self.roles_list.clear();
            for role in roles {
                self.roles_list.add_item_q_string(&qs(role));
            }
        }
    }

    /// display the dialog
    pub fn show(&mut self) {
        unsafe {
            self.dialog.show();
        }
    }
    pub fn exec(&mut self) -> std::os::raw::c_int {
        unsafe { self.dialog.exec() }
    }
    pub fn open(&mut self) {
        unsafe { self.dialog.open() }
    }
    pub fn result(&self) -> std::os::raw::c_int {
        unsafe { self.dialog.result() }
    }
    pub fn close(&mut self) -> bool {
        unsafe { self.dialog.close() }
    }

    pub fn finished(&self) -> qt_core::Signal<(std::os::raw::c_int,)> {
        self.dialog.finished()
    }

    unsafe fn add_site_cbox(mut parent: MutPtr<QLayout>) -> MutPtr<QComboBox> {
        let mut sites_cbox = QComboBox::new_0a();
        sites_cbox.set_object_name(&qs("SelectLocationComboBox"));
        let sites_cbox_ptr = sites_cbox.as_mut_ptr();
        parent.add_widget(sites_cbox.into_ptr());
        sites_cbox_ptr
    }

    unsafe fn add_select_site_groupbox(mut parent: MutPtr<QVBoxLayout>) -> MutPtr<QGroupBox> {
        let mut group_box = QGroupBox::from_q_string(&qs("Select Site"));
        let group_box_ptr = group_box.as_mut_ptr();
        group_box.set_object_name(&qs("SelectSiteGroupBox"));
        let layout = create_vlayout();
        group_box.set_layout(layout.into_ptr());
        parent.add_widget(group_box.into_ptr());
        group_box_ptr
    }

    unsafe fn add_seq_cbox(mut parent: MutPtr<QLayout>) -> MutPtr<QComboBox> {
        let mut seqs_cbox = QComboBox::new_0a();
        seqs_cbox.set_object_name(&qs("AddSeqsComboBox"));
        seqs_cbox.add_item_q_string(&qs("All Sequences"));
        let seqs_cbox_ptr = seqs_cbox.as_mut_ptr();
        parent.add_widget(seqs_cbox.into_ptr());
        seqs_cbox_ptr
    }

    unsafe fn add_shot_cbox(mut parent: MutPtr<QLayout>) -> MutPtr<QComboBox> {
        let mut shots_cbox = QComboBox::new_0a();
        shots_cbox.set_object_name(&qs("AddShotsComboBox"));
        shots_cbox.add_item_q_string(&qs("All Shots"));
        let shots_cbox_ptr = shots_cbox.as_mut_ptr();
        parent.add_widget(shots_cbox.into_ptr());
        shots_cbox_ptr
    }

    unsafe fn add_select_level_groupbox(mut parent: MutPtr<QVBoxLayout>) -> MutPtr<QGroupBox> {
        let mut group_box = QGroupBox::from_q_string(&qs("Select Sequence/Shot"));
        let group_box_ptr = group_box.as_mut_ptr();
        group_box.set_object_name(&qs("SelectLevelsGroupBox"));
        let layout = create_vlayout();
        group_box.set_layout(layout.into_ptr());
        parent.add_widget(group_box.into_ptr());
        group_box_ptr
    }

    unsafe fn add_roles_listwidget(mut parent: MutPtr<QLayout>) -> MutPtr<QListWidget> {
        let mut list_widget = QListWidget::new_0a();
        list_widget.set_selection_mode(SelectionMode::ExtendedSelection);
        let list_widget_ptr = list_widget.as_mut_ptr();
        parent.add_widget(list_widget.into_ptr());
        list_widget_ptr
    }

    unsafe fn add_roles_filter(parent: MutPtr<QLayout>) -> MutPtr<QLineEdit> {
        let mut hlayout = create_hlayout();
        let mut hlayout_ptr = hlayout.as_mut_ptr();
        let mut parent: MutPtr<QVBoxLayout> = parent.dynamic_cast_mut();
        if parent.is_null() {
            panic!("unable to cast layout");
        }
        parent.add_layout_1a(hlayout.into_ptr());
        hlayout_ptr.add_widget(QLabel::from_q_string(&qs("Filter:")).into_ptr());
        let mut line_edit = QLineEdit::new();
        line_edit.set_object_name(&qs("RolesFilterLineEdit"));
        let line_edit_ptr = line_edit.as_mut_ptr();
        hlayout_ptr.add_widget(line_edit.into_ptr());
        line_edit_ptr
    }

    unsafe fn add_select_roles_groupbox(mut parent: MutPtr<QVBoxLayout>) -> MutPtr<QGroupBox> {
        let mut group_box = QGroupBox::from_q_string(&qs("Select Roles"));
        let mut group_box_ptr = group_box.as_mut_ptr();
        group_box.set_object_name(&qs("SelectRolesGroupBox"));
        parent.add_widget(group_box.into_ptr());
        let layout = create_vlayout();
        group_box_ptr.set_layout(layout.into_ptr());
        group_box_ptr
    }
    unsafe fn add_right_layout(mut parent: MutPtr<QHBoxLayout>) -> MutPtr<QVBoxLayout> {
        let mut layout_right = create_vlayout();
        let layout_right_ptr = layout_right.as_mut_ptr();
        parent.add_layout_1a(layout_right.into_ptr());
        layout_right_ptr
    }
    unsafe fn add_left_layout(mut parent: MutPtr<QHBoxLayout>) -> MutPtr<QVBoxLayout> {
        let mut layout_left = create_vlayout();
        let layout_left_ptr = layout_left.as_mut_ptr();
        parent.add_layout_1a(layout_left.into_ptr());
        layout_left_ptr
    }
    // add the add_entry label to the left hand side
    unsafe fn add_entry_label(mut parent: MutPtr<QVBoxLayout>) {
        // add label
        let mut entry_frame = QFrame::new_0a();
        let mut entry_frame_ptr = entry_frame.as_mut_ptr();
        parent.add_widget(entry_frame.into_ptr());
        let mut add_entries = QLabel::from_q_string(&qs("Add Entry"));
        add_entries.set_object_name(&qs("AddEntriesLabel"));
        let mut add_entry_layout = create_vlayout();
        add_entry_layout.add_widget(add_entries.into_ptr());
        entry_frame_ptr.set_layout(add_entry_layout.into_ptr());
    }
    // add the distribution label in the middle of the dialog
    unsafe fn add_distribution_label(distribution: &str, mut parent: MutPtr<QVBoxLayout>) {
        // layout is the top level layout for the dialog
        let mut dist_frame = QFrame::new_0a();
        let mut distribution = QLabel::from_q_string(&qs(distribution));
        distribution.set_object_name(&qs("DistributionLabel"));
        let mut add_entry_layout = create_hlayout();
        add_entry_layout.add_stretch_1a(1);
        add_entry_layout.add_widget(distribution.into_ptr());
        add_entry_layout.add_stretch_1a(1);
        dist_frame.set_layout(add_entry_layout.into_ptr());
        parent.add_widget(dist_frame.into_ptr());
    }
}
