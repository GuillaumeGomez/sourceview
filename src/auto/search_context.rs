// This file was generated by gir (d933f9a) from gir-files (469db10)
// DO NOT EDIT

#[cfg(any(feature = "v3_10", feature = "dox"))]
use Buffer;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use Error;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use SearchSettings;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use Style;
use ffi;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use gio;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use gio_ffi;
use glib;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use gtk;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct SearchContext(Object<ffi::GtkSourceSearchContext, ffi::GtkSourceSearchContextClass>);

    match fn {
        get_type => || ffi::gtk_source_search_context_get_type(),
    }
}

impl SearchContext {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn new<'a, P: Into<Option<&'a SearchSettings>>>(buffer: &Buffer, settings: P) -> SearchContext {
        let settings = settings.into();
        let settings = settings.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_source_search_context_new(buffer.to_glib_none().0, settings.0))
        }
    }
}

pub trait SearchContextExt {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn backward(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter)>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn backward2(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter, bool)>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn backward_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: Fn(Result<(gtk::TextIter, gtk::TextIter), Error>) + Send + Sync + 'static>(&self, iter: &gtk::TextIter, cancellable: P, callback: Q);

    //#[cfg(any(feature = "v3_22", feature = "dox"))]
    //fn backward_finish2<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<(gtk::TextIter, gtk::TextIter, bool), Error>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn forward(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter)>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn forward2(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter, bool)>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn forward_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: Fn(Result<(gtk::TextIter, gtk::TextIter), Error>) + Send + Sync + 'static>(&self, iter: &gtk::TextIter, cancellable: P, callback: Q);

    //#[cfg(any(feature = "v3_22", feature = "dox"))]
    //fn forward_finish2<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<(gtk::TextIter, gtk::TextIter, bool), Error>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_buffer(&self) -> Option<Buffer>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_highlight(&self) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_match_style(&self) -> Option<Style>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_occurrence_position(&self, match_start: &gtk::TextIter, match_end: &gtk::TextIter) -> i32;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_occurrences_count(&self) -> i32;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_regex_error(&self) -> Option<Error>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_settings(&self) -> Option<SearchSettings>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn replace(&self, match_start: &gtk::TextIter, match_end: &gtk::TextIter, replace: &str) -> Result<(), Error>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn replace2(&self, match_start: &mut gtk::TextIter, match_end: &mut gtk::TextIter, replace: &str) -> Result<(), Error>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn replace_all(&self, replace: &str) -> Result<(), Error>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_highlight(&self, highlight: bool);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_match_style<'a, P: Into<Option<&'a Style>>>(&self, match_style: P);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_settings<'a, P: Into<Option<&'a SearchSettings>>>(&self, settings: P);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_highlight_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_match_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_occurrences_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_regex_error_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_settings_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SearchContext> + IsA<glib::object::Object>> SearchContextExt for O {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn backward(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter)> {
        unsafe {
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let ret = from_glib(ffi::gtk_source_search_context_backward(self.to_glib_none().0, iter.to_glib_none().0, match_start.to_glib_none_mut().0, match_end.to_glib_none_mut().0));
            if ret { Some((match_start, match_end)) } else { None }
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn backward2(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter, bool)> {
        unsafe {
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let mut has_wrapped_around = mem::uninitialized();
            let ret = from_glib(ffi::gtk_source_search_context_backward2(self.to_glib_none().0, iter.to_glib_none().0, match_start.to_glib_none_mut().0, match_end.to_glib_none_mut().0, &mut has_wrapped_around));
            if ret { Some((match_start, match_end, from_glib(has_wrapped_around))) } else { None }
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn backward_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: Fn(Result<(gtk::TextIter, gtk::TextIter), Error>) + Send + Sync + 'static>(&self, iter: &gtk::TextIter, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Fn(Result<(gtk::TextIter, gtk::TextIter), Error>) + Send + Sync + 'static>> = Box::new(Box::new(callback));
        extern "C" fn backward_async_trampoline(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            unsafe {
                let mut error = ptr::null_mut();
                let mut match_start = gtk::TextIter::uninitialized();
                let mut match_end = gtk::TextIter::uninitialized();
                let _ = ffi::gtk_source_search_context_backward_finish(_source_object as *mut _, res, &mut match_start, &mut match_end, &mut error);
                let result = if error.is_null() { Ok((from_glib_none(match_start), from_glib_none(match_end))) } else { Err(from_glib_full(error)) };
                let callback: &&(Fn(Result<(gtk::TextIter, gtk::TextIter), Error>) + Send + Sync + 'static) = transmute(user_data);
                callback(result);
            }
        }
        let callback = backward_async_trampoline;
        unsafe {
            ffi::gtk_source_search_context_backward_async(self.to_glib_none().0, iter.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    //#[cfg(any(feature = "v3_22", feature = "dox"))]
    //fn backward_finish2<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<(gtk::TextIter, gtk::TextIter, bool), Error> {
    //    unsafe { TODO: call ffi::gtk_source_search_context_backward_finish2() }
    //}

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn forward(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter)> {
        unsafe {
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let ret = from_glib(ffi::gtk_source_search_context_forward(self.to_glib_none().0, iter.to_glib_none().0, match_start.to_glib_none_mut().0, match_end.to_glib_none_mut().0));
            if ret { Some((match_start, match_end)) } else { None }
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn forward2(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter, bool)> {
        unsafe {
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let mut has_wrapped_around = mem::uninitialized();
            let ret = from_glib(ffi::gtk_source_search_context_forward2(self.to_glib_none().0, iter.to_glib_none().0, match_start.to_glib_none_mut().0, match_end.to_glib_none_mut().0, &mut has_wrapped_around));
            if ret { Some((match_start, match_end, from_glib(has_wrapped_around))) } else { None }
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn forward_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: Fn(Result<(gtk::TextIter, gtk::TextIter), Error>) + Send + Sync + 'static>(&self, iter: &gtk::TextIter, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Fn(Result<(gtk::TextIter, gtk::TextIter), Error>) + Send + Sync + 'static>> = Box::new(Box::new(callback));
        extern "C" fn forward_async_trampoline(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            unsafe {
                let mut error = ptr::null_mut();
                let mut match_start = gtk::TextIter::uninitialized();
                let mut match_end = gtk::TextIter::uninitialized();
                let _ = ffi::gtk_source_search_context_forward_finish(_source_object as *mut _, res, &mut match_start, &mut match_end, &mut error);
                let result = if error.is_null() { Ok((from_glib_none(match_start), from_glib_none(match_end))) } else { Err(from_glib_full(error)) };
                let callback: &&(Fn(Result<(gtk::TextIter, gtk::TextIter), Error>) + Send + Sync + 'static) = transmute(user_data);
                callback(result);
            }
        }
        let callback = forward_async_trampoline;
        unsafe {
            ffi::gtk_source_search_context_forward_async(self.to_glib_none().0, iter.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    //#[cfg(any(feature = "v3_22", feature = "dox"))]
    //fn forward_finish2<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<(gtk::TextIter, gtk::TextIter, bool), Error> {
    //    unsafe { TODO: call ffi::gtk_source_search_context_forward_finish2() }
    //}

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_none(ffi::gtk_source_search_context_get_buffer(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_highlight(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_search_context_get_highlight(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_match_style(&self) -> Option<Style> {
        unsafe {
            from_glib_none(ffi::gtk_source_search_context_get_match_style(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_occurrence_position(&self, match_start: &gtk::TextIter, match_end: &gtk::TextIter) -> i32 {
        unsafe {
            ffi::gtk_source_search_context_get_occurrence_position(self.to_glib_none().0, match_start.to_glib_none().0, match_end.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_occurrences_count(&self) -> i32 {
        unsafe {
            ffi::gtk_source_search_context_get_occurrences_count(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_regex_error(&self) -> Option<Error> {
        unsafe {
            from_glib_full(ffi::gtk_source_search_context_get_regex_error(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_settings(&self) -> Option<SearchSettings> {
        unsafe {
            from_glib_none(ffi::gtk_source_search_context_get_settings(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn replace(&self, match_start: &gtk::TextIter, match_end: &gtk::TextIter, replace: &str) -> Result<(), Error> {
        let replace_length = replace.len() as i32;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_source_search_context_replace(self.to_glib_none().0, match_start.to_glib_none().0, match_end.to_glib_none().0, replace.to_glib_none().0, replace_length, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn replace2(&self, match_start: &mut gtk::TextIter, match_end: &mut gtk::TextIter, replace: &str) -> Result<(), Error> {
        let replace_length = replace.len() as i32;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_source_search_context_replace2(self.to_glib_none().0, match_start.to_glib_none_mut().0, match_end.to_glib_none_mut().0, replace.to_glib_none().0, replace_length, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn replace_all(&self, replace: &str) -> Result<(), Error> {
        let replace_length = replace.len() as i32;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_source_search_context_replace_all(self.to_glib_none().0, replace.to_glib_none().0, replace_length, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_highlight(&self, highlight: bool) {
        unsafe {
            ffi::gtk_source_search_context_set_highlight(self.to_glib_none().0, highlight.to_glib());
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_match_style<'a, P: Into<Option<&'a Style>>>(&self, match_style: P) {
        let match_style = match_style.into();
        let match_style = match_style.to_glib_none();
        unsafe {
            ffi::gtk_source_search_context_set_match_style(self.to_glib_none().0, match_style.0);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_settings<'a, P: Into<Option<&'a SearchSettings>>>(&self, settings: P) {
        let settings = settings.into();
        let settings = settings.to_glib_none();
        unsafe {
            ffi::gtk_source_search_context_set_settings(self.to_glib_none().0, settings.0);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::buffer",
                transmute(notify_buffer_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_highlight_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::highlight",
                transmute(notify_highlight_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_match_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::match-style",
                transmute(notify_match_style_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_occurrences_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::occurrences-count",
                transmute(notify_occurrences_count_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_regex_error_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::regex-error",
                transmute(notify_regex_error_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_settings_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::settings",
                transmute(notify_settings_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
unsafe extern "C" fn notify_buffer_trampoline<P>(this: *mut ffi::GtkSourceSearchContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SearchContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchContext::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
unsafe extern "C" fn notify_highlight_trampoline<P>(this: *mut ffi::GtkSourceSearchContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SearchContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchContext::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_match_style_trampoline<P>(this: *mut ffi::GtkSourceSearchContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SearchContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchContext::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
unsafe extern "C" fn notify_occurrences_count_trampoline<P>(this: *mut ffi::GtkSourceSearchContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SearchContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchContext::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
unsafe extern "C" fn notify_regex_error_trampoline<P>(this: *mut ffi::GtkSourceSearchContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SearchContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchContext::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
unsafe extern "C" fn notify_settings_trampoline<P>(this: *mut ffi::GtkSourceSearchContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SearchContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchContext::from_glib_borrow(this).downcast_unchecked())
}
