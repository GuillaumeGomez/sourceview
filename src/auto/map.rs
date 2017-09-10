// This file was generated by gir (e95e5d8) from gir-files (db49619)
// DO NOT EDIT

use View;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use gtk_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Map(Object<ffi::GtkSourceMap>): [
        View,
        gtk::TextView => gtk_ffi::GtkTextView,
        gtk::Widget => gtk_ffi::GtkWidget,
    ];

    match fn {
        get_type => || ffi::gtk_source_map_get_type(),
    }
}

impl Map {
    #[cfg(feature = "v3_18")]
    pub fn new() -> Map {
        unsafe {
            gtk::Widget::from_glib_none(ffi::gtk_source_map_new()).downcast_unchecked()
        }
    }
}

#[cfg(feature = "v3_18")]
impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

pub trait MapExt {
    #[cfg(feature = "v3_18")]
    fn get_view(&self) -> Option<View>;

    #[cfg(feature = "v3_18")]
    fn set_view<P: IsA<View>>(&self, view: &P);

    fn get_property_view(&self) -> Option<View>;

    fn set_property_view<P: IsA<View> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, view: Option<&P>);

    fn connect_property_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Map> + IsA<glib::object::Object>> MapExt for O {
    #[cfg(feature = "v3_18")]
    fn get_view(&self) -> Option<View> {
        unsafe {
            from_glib_none(ffi::gtk_source_map_get_view(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_18")]
    fn set_view<P: IsA<View>>(&self, view: &P) {
        unsafe {
            ffi::gtk_source_map_set_view(self.to_glib_none().0, view.to_glib_none().0);
        }
    }

    fn get_property_view(&self) -> Option<View> {
        let mut value = Value::from(None::<&View>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "view".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_view<P: IsA<View> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, view: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "view".to_glib_none().0, Value::from(view).to_glib_none().0);
        }
    }

    fn connect_property_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::view",
                transmute(notify_view_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_view_trampoline<P>(this: *mut ffi::GtkSourceMap, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Map> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Map::from_glib_borrow(this).downcast_unchecked())
}
