use re_ui::UiExt as _;

/// Generic editor for a boolean value.
pub fn edit_bool(
    _ctx: &re_viewer_context::ViewerContext<'_>,
    ui: &mut egui::Ui,
    value: &mut impl std::ops::DerefMut<Target = re_types::datatypes::Bool>,
) -> egui::Response {
    edit_bool_impl(ui, &mut value.deref_mut().0)
}

/// Generic editor for a boolean value that is not wrapped into [`re_types::datatypes::Bool`].
pub fn edit_bool_raw(
    _ctx: &re_viewer_context::ViewerContext<'_>,
    ui: &mut egui::Ui,
    value: &mut impl std::ops::DerefMut<Target = bool>,
) -> egui::Response {
    edit_bool_impl(ui, value)
}

/// Non monomorphized implementation of [`edit_bool`].
fn edit_bool_impl(ui: &mut egui::Ui, value: &mut bool) -> egui::Response {
    ui.scope(move |ui| ui.re_checkbox(value, "")).inner
}
