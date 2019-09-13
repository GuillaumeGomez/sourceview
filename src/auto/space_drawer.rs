// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_24", feature = "dox"))]
use gio;
#[cfg(any(feature = "v3_24", feature = "dox"))]
use glib;
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v3_24", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v3_24", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
#[cfg(any(feature = "v3_24", feature = "dox"))]
use glib_sys;
use gtk_source_sys;
#[cfg(any(feature = "v3_24", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v3_24", feature = "dox"))]
use std::mem::transmute;
#[cfg(any(feature = "v3_24", feature = "dox"))]
use SpaceLocationFlags;
#[cfg(any(feature = "v3_24", feature = "dox"))]
use SpaceTypeFlags;

glib_wrapper! {
    pub struct SpaceDrawer(Object<gtk_source_sys::GtkSourceSpaceDrawer, gtk_source_sys::GtkSourceSpaceDrawerClass, SpaceDrawerClass>);

    match fn {
        get_type => || gtk_source_sys::gtk_source_space_drawer_get_type(),
    }
}

impl SpaceDrawer {
    #[cfg(any(feature = "v3_24", feature = "dox"))]
    pub fn new() -> SpaceDrawer {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_source_sys::gtk_source_space_drawer_new()) }
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
impl Default for SpaceDrawer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SpaceDrawerBuilder {
    #[cfg(any(feature = "v3_24", feature = "dox"))]
    enable_matrix: Option<bool>,
    #[cfg(any(feature = "v3_24", feature = "dox"))]
    matrix: Option<glib::Variant>,
}

impl SpaceDrawerBuilder {
    pub fn new() -> Self {
        Self {
            #[cfg(any(feature = "v3_24", feature = "dox"))]
            enable_matrix: None,
            #[cfg(any(feature = "v3_24", feature = "dox"))]
            matrix: None,
        }
    }

    pub fn build(self) -> SpaceDrawer {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v3_24", feature = "dox"))]
        {
            if let Some(ref enable_matrix) = self.enable_matrix {
                properties.push(("enable-matrix", enable_matrix));
            }
        }
        #[cfg(any(feature = "v3_24", feature = "dox"))]
        {
            if let Some(ref matrix) = self.matrix {
                properties.push(("matrix", matrix));
            }
        }
        glib::Object::new(SpaceDrawer::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    pub fn enable_matrix(mut self, enable_matrix: bool) -> Self {
        self.enable_matrix = Some(enable_matrix);
        self
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    pub fn matrix(mut self, matrix: &glib::Variant) -> Self {
        self.matrix = Some(matrix.clone());
        self
    }
}

pub const NONE_SPACE_DRAWER: Option<&SpaceDrawer> = None;

pub trait SpaceDrawerExt: 'static {
    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn bind_matrix_setting<P: IsA<gio::Settings>>(
        &self,
        settings: &P,
        key: &str,
        flags: gio::SettingsBindFlags,
    );

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_enable_matrix(&self) -> bool;

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_matrix(&self) -> Option<glib::Variant>;

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_types_for_locations(&self, locations: SpaceLocationFlags) -> SpaceTypeFlags;

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_enable_matrix(&self, enable_matrix: bool);

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_matrix(&self, matrix: Option<&glib::Variant>);

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_types_for_locations(&self, locations: SpaceLocationFlags, types: SpaceTypeFlags);

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn connect_property_enable_matrix_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn connect_property_matrix_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SpaceDrawer>> SpaceDrawerExt for O {
    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn bind_matrix_setting<P: IsA<gio::Settings>>(
        &self,
        settings: &P,
        key: &str,
        flags: gio::SettingsBindFlags,
    ) {
        unsafe {
            gtk_source_sys::gtk_source_space_drawer_bind_matrix_setting(
                self.as_ref().to_glib_none().0,
                settings.as_ref().to_glib_none().0,
                key.to_glib_none().0,
                flags.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_enable_matrix(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_space_drawer_get_enable_matrix(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_matrix(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_space_drawer_get_matrix(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_types_for_locations(&self, locations: SpaceLocationFlags) -> SpaceTypeFlags {
        unsafe {
            from_glib(
                gtk_source_sys::gtk_source_space_drawer_get_types_for_locations(
                    self.as_ref().to_glib_none().0,
                    locations.to_glib(),
                ),
            )
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_enable_matrix(&self, enable_matrix: bool) {
        unsafe {
            gtk_source_sys::gtk_source_space_drawer_set_enable_matrix(
                self.as_ref().to_glib_none().0,
                enable_matrix.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_matrix(&self, matrix: Option<&glib::Variant>) {
        unsafe {
            gtk_source_sys::gtk_source_space_drawer_set_matrix(
                self.as_ref().to_glib_none().0,
                matrix.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_types_for_locations(&self, locations: SpaceLocationFlags, types: SpaceTypeFlags) {
        unsafe {
            gtk_source_sys::gtk_source_space_drawer_set_types_for_locations(
                self.as_ref().to_glib_none().0,
                locations.to_glib(),
                types.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn connect_property_enable_matrix_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_matrix_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceSpaceDrawer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SpaceDrawer>,
        {
            let f: &F = &*(f as *const F);
            f(&SpaceDrawer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enable-matrix\0".as_ptr() as *const _,
                Some(transmute(
                    notify_enable_matrix_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn connect_property_matrix_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_matrix_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceSpaceDrawer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SpaceDrawer>,
        {
            let f: &F = &*(f as *const F);
            f(&SpaceDrawer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::matrix\0".as_ptr() as *const _,
                Some(transmute(notify_matrix_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SpaceDrawer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SpaceDrawer")
    }
}
