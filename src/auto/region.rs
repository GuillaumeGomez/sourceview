// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_22", feature = "dox"))]
use glib::object::Cast;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use glib::object::IsA;
use glib::translate::*;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use glib::ToValue;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use gtk;
use gtk_source_sys;
use std::fmt;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use RegionIter;

glib_wrapper! {
    pub struct Region(Object<gtk_source_sys::GtkSourceRegion, gtk_source_sys::GtkSourceRegionClass, RegionClass>);

    match fn {
        get_type => || gtk_source_sys::gtk_source_region_get_type(),
    }
}

impl Region {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn new<P: IsA<gtk::TextBuffer>>(buffer: &P) -> Region {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_region_new(
                buffer.as_ref().to_glib_none().0,
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct RegionBuilder {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    buffer: Option<gtk::TextBuffer>,
}

impl RegionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Region {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v3_22", feature = "dox"))]
        {
            if let Some(ref buffer) = self.buffer {
                properties.push(("buffer", buffer));
            }
        }
        glib::Object::new(Region::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn buffer<P: IsA<gtk::TextBuffer>>(mut self, buffer: &P) -> Self {
        self.buffer = Some(buffer.clone().upcast());
        self
    }
}

pub const NONE_REGION: Option<&Region> = None;

pub trait RegionExt: 'static {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn add_region<P: IsA<Region>>(&self, region_to_add: Option<&P>);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn add_subregion(&self, _start: &gtk::TextIter, _end: &gtk::TextIter);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_bounds(&self) -> Option<(gtk::TextIter, gtk::TextIter)>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_buffer(&self) -> Option<gtk::TextBuffer>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_start_region_iter(&self) -> RegionIter;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn intersect_region<P: IsA<Region>>(&self, region2: Option<&P>) -> Option<Region>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn intersect_subregion(&self, _start: &gtk::TextIter, _end: &gtk::TextIter) -> Option<Region>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn is_empty(&self) -> bool;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn subtract_region<P: IsA<Region>>(&self, region_to_subtract: Option<&P>);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn subtract_subregion(&self, _start: &gtk::TextIter, _end: &gtk::TextIter);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn to_string(&self) -> GString;
}

impl<O: IsA<Region>> RegionExt for O {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn add_region<P: IsA<Region>>(&self, region_to_add: Option<&P>) {
        unsafe {
            gtk_source_sys::gtk_source_region_add_region(
                self.as_ref().to_glib_none().0,
                region_to_add.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn add_subregion(&self, _start: &gtk::TextIter, _end: &gtk::TextIter) {
        unsafe {
            gtk_source_sys::gtk_source_region_add_subregion(
                self.as_ref().to_glib_none().0,
                _start.to_glib_none().0,
                _end.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_bounds(&self) -> Option<(gtk::TextIter, gtk::TextIter)> {
        unsafe {
            let mut start = gtk::TextIter::uninitialized();
            let mut end = gtk::TextIter::uninitialized();
            let ret = from_glib(gtk_source_sys::gtk_source_region_get_bounds(
                self.as_ref().to_glib_none().0,
                start.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
            ));
            if ret {
                Some((start, end))
            } else {
                None
            }
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_buffer(&self) -> Option<gtk::TextBuffer> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_region_get_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_start_region_iter(&self) -> RegionIter {
        unsafe {
            let mut iter = RegionIter::uninitialized();
            gtk_source_sys::gtk_source_region_get_start_region_iter(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
            );
            iter
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn intersect_region<P: IsA<Region>>(&self, region2: Option<&P>) -> Option<Region> {
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_region_intersect_region(
                self.as_ref().to_glib_none().0,
                region2.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn intersect_subregion(&self, _start: &gtk::TextIter, _end: &gtk::TextIter) -> Option<Region> {
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_region_intersect_subregion(
                self.as_ref().to_glib_none().0,
                _start.to_glib_none().0,
                _end.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn is_empty(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_region_is_empty(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn subtract_region<P: IsA<Region>>(&self, region_to_subtract: Option<&P>) {
        unsafe {
            gtk_source_sys::gtk_source_region_subtract_region(
                self.as_ref().to_glib_none().0,
                region_to_subtract.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn subtract_subregion(&self, _start: &gtk::TextIter, _end: &gtk::TextIter) {
        unsafe {
            gtk_source_sys::gtk_source_region_subtract_subregion(
                self.as_ref().to_glib_none().0,
                _start.to_glib_none().0,
                _end.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn to_string(&self) -> GString {
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_region_to_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Region")
    }
}
