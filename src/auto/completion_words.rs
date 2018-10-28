// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_10", feature = "dox"))]
use CompletionActivation;
use CompletionProvider;
use ffi;
use gdk_pixbuf;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CompletionWords(Object<ffi::GtkSourceCompletionWords, ffi::GtkSourceCompletionWordsClass>): CompletionProvider;

    match fn {
        get_type => || ffi::gtk_source_completion_words_get_type(),
    }
}

impl CompletionWords {
    pub fn new<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b gdk_pixbuf::Pixbuf>>>(name: P, icon: Q) -> CompletionWords {
        assert_initialized_main_thread!();
        let name = name.into();
        let name = name.to_glib_none();
        let icon = icon.into();
        let icon = icon.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_source_completion_words_new(name.0, icon.0))
        }
    }
}

pub trait CompletionWordsExt {
    fn register<P: IsA<gtk::TextBuffer>>(&self, buffer: &P);

    fn unregister<P: IsA<gtk::TextBuffer>>(&self, buffer: &P);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_property_activation(&self, activation: CompletionActivation);

    fn set_property_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>);

    fn set_property_interactive_delay(&self, interactive_delay: i32);

    fn get_property_minimum_word_size(&self) -> u32;

    fn set_property_minimum_word_size(&self, minimum_word_size: u32);

    fn set_property_name<'a, P: Into<Option<&'a str>>>(&self, name: P);

    fn set_property_priority(&self, priority: i32);

    fn get_property_proposals_batch_size(&self) -> u32;

    fn set_property_proposals_batch_size(&self, proposals_batch_size: u32);

    fn get_property_scan_batch_size(&self) -> u32;

    fn set_property_scan_batch_size(&self, scan_batch_size: u32);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_activation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_interactive_delay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_minimum_word_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_proposals_batch_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scan_batch_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CompletionWords> + IsA<glib::object::Object>> CompletionWordsExt for O {
    fn register<P: IsA<gtk::TextBuffer>>(&self, buffer: &P) {
        unsafe {
            ffi::gtk_source_completion_words_register(self.to_glib_none().0, buffer.to_glib_none().0);
        }
    }

    fn unregister<P: IsA<gtk::TextBuffer>>(&self, buffer: &P) {
        unsafe {
            ffi::gtk_source_completion_words_unregister(self.to_glib_none().0, buffer.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_property_activation(&self, activation: CompletionActivation) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "activation".to_glib_none().0, Value::from(&activation).to_glib_none().0);
        }
    }

    fn set_property_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon".to_glib_none().0, Value::from(icon).to_glib_none().0);
        }
    }

    fn set_property_interactive_delay(&self, interactive_delay: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "interactive-delay".to_glib_none().0, Value::from(&interactive_delay).to_glib_none().0);
        }
    }

    fn get_property_minimum_word_size(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "minimum-word-size".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_minimum_word_size(&self, minimum_word_size: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "minimum-word-size".to_glib_none().0, Value::from(&minimum_word_size).to_glib_none().0);
        }
    }

    fn set_property_name<'a, P: Into<Option<&'a str>>>(&self, name: P) {
        let name = name.into();
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "name".to_glib_none().0, Value::from(name).to_glib_none().0);
        }
    }

    fn set_property_priority(&self, priority: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "priority".to_glib_none().0, Value::from(&priority).to_glib_none().0);
        }
    }

    fn get_property_proposals_batch_size(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "proposals-batch-size".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_proposals_batch_size(&self, proposals_batch_size: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "proposals-batch-size".to_glib_none().0, Value::from(&proposals_batch_size).to_glib_none().0);
        }
    }

    fn get_property_scan_batch_size(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "scan-batch-size".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_scan_batch_size(&self, scan_batch_size: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "scan-batch-size".to_glib_none().0, Value::from(&scan_batch_size).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_activation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::activation",
                transmute(notify_activation_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icon",
                transmute(notify_icon_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_interactive_delay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::interactive-delay",
                transmute(notify_interactive_delay_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_minimum_word_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::minimum-word-size",
                transmute(notify_minimum_word_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::name",
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::priority",
                transmute(notify_priority_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_proposals_batch_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::proposals-batch-size",
                transmute(notify_proposals_batch_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_scan_batch_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::scan-batch-size",
                transmute(notify_scan_batch_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
unsafe extern "C" fn notify_activation_trampoline<P>(this: *mut ffi::GtkSourceCompletionWords, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CompletionWords> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionWords::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_icon_trampoline<P>(this: *mut ffi::GtkSourceCompletionWords, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CompletionWords> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionWords::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_interactive_delay_trampoline<P>(this: *mut ffi::GtkSourceCompletionWords, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CompletionWords> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionWords::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_minimum_word_size_trampoline<P>(this: *mut ffi::GtkSourceCompletionWords, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CompletionWords> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionWords::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::GtkSourceCompletionWords, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CompletionWords> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionWords::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_priority_trampoline<P>(this: *mut ffi::GtkSourceCompletionWords, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CompletionWords> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionWords::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_proposals_batch_size_trampoline<P>(this: *mut ffi::GtkSourceCompletionWords, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CompletionWords> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionWords::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_scan_batch_size_trampoline<P>(this: *mut ffi::GtkSourceCompletionWords, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CompletionWords> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionWords::from_glib_borrow(this).downcast_unchecked())
}
