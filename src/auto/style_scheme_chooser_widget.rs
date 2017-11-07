// This file was generated by gir (0409d73) from gir-files (469db10)
// DO NOT EDIT

use StyleSchemeChooser;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use gtk_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct StyleSchemeChooserWidget(Object<ffi::GtkSourceStyleSchemeChooserWidget, ffi::GtkSourceStyleSchemeChooserWidgetClass>): [
        gtk::Widget => gtk_ffi::GtkWidget,
        StyleSchemeChooser,
    ];

    match fn {
        get_type => || ffi::gtk_source_style_scheme_chooser_widget_get_type(),
    }
}

impl StyleSchemeChooserWidget {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn new() -> StyleSchemeChooserWidget {
        unsafe {
            gtk::Widget::from_glib_none(ffi::gtk_source_style_scheme_chooser_widget_new()).downcast_unchecked()
        }
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
impl Default for StyleSchemeChooserWidget {
    fn default() -> Self {
        Self::new()
    }
}
