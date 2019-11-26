// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use cairo;
use gdk;
use gdk_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk;
use gtk_source_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use GutterRendererAlignmentMode;
use GutterRendererState;

glib_wrapper! {
    pub struct GutterRenderer(Object<gtk_source_sys::GtkSourceGutterRenderer, gtk_source_sys::GtkSourceGutterRendererClass, GutterRendererClass>);

    match fn {
        get_type => || gtk_source_sys::gtk_source_gutter_renderer_get_type(),
    }
}

#[derive(Clone, Default)]
pub struct GutterRendererBuilder {
    alignment_mode: Option<GutterRendererAlignmentMode>,
    background_rgba: Option<gdk::RGBA>,
    background_set: Option<bool>,
    size: Option<i32>,
    visible: Option<bool>,
    xalign: Option<f32>,
    xpad: Option<i32>,
    yalign: Option<f32>,
    ypad: Option<i32>,
}

impl GutterRendererBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> GutterRenderer {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref alignment_mode) = self.alignment_mode {
            properties.push(("alignment-mode", alignment_mode));
        }
        if let Some(ref background_rgba) = self.background_rgba {
            properties.push(("background-rgba", background_rgba));
        }
        if let Some(ref background_set) = self.background_set {
            properties.push(("background-set", background_set));
        }
        if let Some(ref size) = self.size {
            properties.push(("size", size));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref xalign) = self.xalign {
            properties.push(("xalign", xalign));
        }
        if let Some(ref xpad) = self.xpad {
            properties.push(("xpad", xpad));
        }
        if let Some(ref yalign) = self.yalign {
            properties.push(("yalign", yalign));
        }
        if let Some(ref ypad) = self.ypad {
            properties.push(("ypad", ypad));
        }
        glib::Object::new(GutterRenderer::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn alignment_mode(mut self, alignment_mode: GutterRendererAlignmentMode) -> Self {
        self.alignment_mode = Some(alignment_mode);
        self
    }

    pub fn background_rgba(mut self, background_rgba: &gdk::RGBA) -> Self {
        self.background_rgba = Some(background_rgba.clone());
        self
    }

    pub fn background_set(mut self, background_set: bool) -> Self {
        self.background_set = Some(background_set);
        self
    }

    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn xalign(mut self, xalign: f32) -> Self {
        self.xalign = Some(xalign);
        self
    }

    pub fn xpad(mut self, xpad: i32) -> Self {
        self.xpad = Some(xpad);
        self
    }

    pub fn yalign(mut self, yalign: f32) -> Self {
        self.yalign = Some(yalign);
        self
    }

    pub fn ypad(mut self, ypad: i32) -> Self {
        self.ypad = Some(ypad);
        self
    }
}

pub const NONE_GUTTER_RENDERER: Option<&GutterRenderer> = None;

pub trait GutterRendererExt: 'static {
    fn activate(&self, iter: &mut gtk::TextIter, area: &mut gdk::Rectangle, event: &mut gdk::Event);

    fn begin(
        &self,
        cr: &cairo::Context,
        background_area: &mut gdk::Rectangle,
        cell_area: &mut gdk::Rectangle,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
    );

    fn draw(
        &self,
        cr: &cairo::Context,
        background_area: &mut gdk::Rectangle,
        cell_area: &mut gdk::Rectangle,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
        state: GutterRendererState,
    );

    fn end(&self);

    fn get_alignment(&self) -> (f32, f32);

    fn get_alignment_mode(&self) -> GutterRendererAlignmentMode;

    fn get_background(&self) -> Option<gdk::RGBA>;

    fn get_padding(&self) -> (i32, i32);

    fn get_size(&self) -> i32;

    fn get_view(&self) -> Option<gtk::TextView>;

    fn get_visible(&self) -> bool;

    fn get_window_type(&self) -> gtk::TextWindowType;

    fn query_activatable(
        &self,
        iter: &mut gtk::TextIter,
        area: &mut gdk::Rectangle,
        event: &mut gdk::Event,
    ) -> bool;

    fn query_data(
        &self,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
        state: GutterRendererState,
    );

    fn query_tooltip(
        &self,
        iter: &mut gtk::TextIter,
        area: &mut gdk::Rectangle,
        x: i32,
        y: i32,
        tooltip: &gtk::Tooltip,
    ) -> bool;

    fn queue_draw(&self);

    fn set_alignment(&self, xalign: f32, yalign: f32);

    fn set_alignment_mode(&self, mode: GutterRendererAlignmentMode);

    fn set_background(&self, color: Option<&gdk::RGBA>);

    fn set_padding(&self, xpad: i32, ypad: i32);

    fn set_size(&self, size: i32);

    fn set_visible(&self, visible: bool);

    fn get_property_background_rgba(&self) -> Option<gdk::RGBA>;

    fn set_property_background_rgba(&self, background_rgba: Option<&gdk::RGBA>);

    fn get_property_background_set(&self) -> bool;

    fn set_property_background_set(&self, background_set: bool);

    fn get_property_xalign(&self) -> f32;

    fn set_property_xalign(&self, xalign: f32);

    fn get_property_xpad(&self) -> i32;

    fn set_property_xpad(&self, xpad: i32);

    fn get_property_yalign(&self) -> f32;

    fn set_property_yalign(&self, yalign: f32);

    fn get_property_ypad(&self) -> i32;

    fn set_property_ypad(&self, ypad: i32);

    fn connect_activate<F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_query_activatable<
        F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_query_data<
        F: Fn(&Self, &gtk::TextIter, &gtk::TextIter, GutterRendererState) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_query_tooltip<
        F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, i32, i32, &gtk::Tooltip) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_queue_draw<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_alignment_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_background_rgba_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_background_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_window_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_xpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ypad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GutterRenderer>> GutterRendererExt for O {
    fn activate(
        &self,
        iter: &mut gtk::TextIter,
        area: &mut gdk::Rectangle,
        event: &mut gdk::Event,
    ) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_renderer_activate(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                area.to_glib_none_mut().0,
                event.to_glib_none_mut().0,
            );
        }
    }

    fn begin(
        &self,
        cr: &cairo::Context,
        background_area: &mut gdk::Rectangle,
        cell_area: &mut gdk::Rectangle,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
    ) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_renderer_begin(
                self.as_ref().to_glib_none().0,
                mut_override(cr.to_glib_none().0),
                background_area.to_glib_none_mut().0,
                cell_area.to_glib_none_mut().0,
                start.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
            );
        }
    }

    fn draw(
        &self,
        cr: &cairo::Context,
        background_area: &mut gdk::Rectangle,
        cell_area: &mut gdk::Rectangle,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
        state: GutterRendererState,
    ) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_renderer_draw(
                self.as_ref().to_glib_none().0,
                mut_override(cr.to_glib_none().0),
                background_area.to_glib_none_mut().0,
                cell_area.to_glib_none_mut().0,
                start.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
                state.to_glib(),
            );
        }
    }

    fn end(&self) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_renderer_end(self.as_ref().to_glib_none().0);
        }
    }

    fn get_alignment(&self) -> (f32, f32) {
        unsafe {
            let mut xalign = mem::MaybeUninit::uninit();
            let mut yalign = mem::MaybeUninit::uninit();
            gtk_source_sys::gtk_source_gutter_renderer_get_alignment(
                self.as_ref().to_glib_none().0,
                xalign.as_mut_ptr(),
                yalign.as_mut_ptr(),
            );
            let xalign = xalign.assume_init();
            let yalign = yalign.assume_init();
            (xalign, yalign)
        }
    }

    fn get_alignment_mode(&self) -> GutterRendererAlignmentMode {
        unsafe {
            from_glib(
                gtk_source_sys::gtk_source_gutter_renderer_get_alignment_mode(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_background(&self) -> Option<gdk::RGBA> {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            let ret = from_glib(gtk_source_sys::gtk_source_gutter_renderer_get_background(
                self.as_ref().to_glib_none().0,
                color.to_glib_none_mut().0,
            ));
            if ret {
                Some(color)
            } else {
                None
            }
        }
    }

    fn get_padding(&self) -> (i32, i32) {
        unsafe {
            let mut xpad = mem::MaybeUninit::uninit();
            let mut ypad = mem::MaybeUninit::uninit();
            gtk_source_sys::gtk_source_gutter_renderer_get_padding(
                self.as_ref().to_glib_none().0,
                xpad.as_mut_ptr(),
                ypad.as_mut_ptr(),
            );
            let xpad = xpad.assume_init();
            let ypad = ypad.assume_init();
            (xpad, ypad)
        }
    }

    fn get_size(&self) -> i32 {
        unsafe {
            gtk_source_sys::gtk_source_gutter_renderer_get_size(self.as_ref().to_glib_none().0)
        }
    }

    fn get_view(&self) -> Option<gtk::TextView> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_gutter_renderer_get_view(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_visible(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_gutter_renderer_get_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_window_type(&self) -> gtk::TextWindowType {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_gutter_renderer_get_window_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn query_activatable(
        &self,
        iter: &mut gtk::TextIter,
        area: &mut gdk::Rectangle,
        event: &mut gdk::Event,
    ) -> bool {
        unsafe {
            from_glib(
                gtk_source_sys::gtk_source_gutter_renderer_query_activatable(
                    self.as_ref().to_glib_none().0,
                    iter.to_glib_none_mut().0,
                    area.to_glib_none_mut().0,
                    event.to_glib_none_mut().0,
                ),
            )
        }
    }

    fn query_data(
        &self,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
        state: GutterRendererState,
    ) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_renderer_query_data(
                self.as_ref().to_glib_none().0,
                start.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
                state.to_glib(),
            );
        }
    }

    fn query_tooltip(
        &self,
        iter: &mut gtk::TextIter,
        area: &mut gdk::Rectangle,
        x: i32,
        y: i32,
        tooltip: &gtk::Tooltip,
    ) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_gutter_renderer_query_tooltip(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                area.to_glib_none_mut().0,
                x,
                y,
                tooltip.to_glib_none().0,
            ))
        }
    }

    fn queue_draw(&self) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_renderer_queue_draw(self.as_ref().to_glib_none().0);
        }
    }

    fn set_alignment(&self, xalign: f32, yalign: f32) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_renderer_set_alignment(
                self.as_ref().to_glib_none().0,
                xalign,
                yalign,
            );
        }
    }

    fn set_alignment_mode(&self, mode: GutterRendererAlignmentMode) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_renderer_set_alignment_mode(
                self.as_ref().to_glib_none().0,
                mode.to_glib(),
            );
        }
    }

    fn set_background(&self, color: Option<&gdk::RGBA>) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_renderer_set_background(
                self.as_ref().to_glib_none().0,
                color.to_glib_none().0,
            );
        }
    }

    fn set_padding(&self, xpad: i32, ypad: i32) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_renderer_set_padding(
                self.as_ref().to_glib_none().0,
                xpad,
                ypad,
            );
        }
    }

    fn set_size(&self, size: i32) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_renderer_set_size(
                self.as_ref().to_glib_none().0,
                size,
            );
        }
    }

    fn set_visible(&self, visible: bool) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_renderer_set_visible(
                self.as_ref().to_glib_none().0,
                visible.to_glib(),
            );
        }
    }

    fn get_property_background_rgba(&self) -> Option<gdk::RGBA> {
        unsafe {
            let mut value = Value::from_type(<gdk::RGBA as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"background-rgba\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `background-rgba` getter")
        }
    }

    fn set_property_background_rgba(&self, background_rgba: Option<&gdk::RGBA>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"background-rgba\0".as_ptr() as *const _,
                Value::from(background_rgba).to_glib_none().0,
            );
        }
    }

    fn get_property_background_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"background-set\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `background-set` getter")
                .unwrap()
        }
    }

    fn set_property_background_set(&self, background_set: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"background-set\0".as_ptr() as *const _,
                Value::from(&background_set).to_glib_none().0,
            );
        }
    }

    fn get_property_xalign(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"xalign\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `xalign` getter")
                .unwrap()
        }
    }

    fn set_property_xalign(&self, xalign: f32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"xalign\0".as_ptr() as *const _,
                Value::from(&xalign).to_glib_none().0,
            );
        }
    }

    fn get_property_xpad(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"xpad\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `xpad` getter")
                .unwrap()
        }
    }

    fn set_property_xpad(&self, xpad: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"xpad\0".as_ptr() as *const _,
                Value::from(&xpad).to_glib_none().0,
            );
        }
    }

    fn get_property_yalign(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"yalign\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `yalign` getter")
                .unwrap()
        }
    }

    fn set_property_yalign(&self, yalign: f32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"yalign\0".as_ptr() as *const _,
                Value::from(&yalign).to_glib_none().0,
            );
        }
    }

    fn get_property_ypad(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"ypad\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `ypad` getter")
                .unwrap()
        }
    }

    fn set_property_ypad(&self, ypad: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"ypad\0".as_ptr() as *const _,
                Value::from(&ypad).to_glib_none().0,
            );
        }
    }

    fn connect_activate<F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<
            P,
            F: Fn(&P, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) + 'static,
        >(
            this: *mut gtk_source_sys::GtkSourceGutterRenderer,
            iter: *mut gtk_sys::GtkTextIter,
            area: *mut gdk_sys::GdkRectangle,
            event: *mut gdk_sys::GdkEvent,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GutterRenderer::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(iter),
                &from_glib_borrow(area),
                &from_glib_none(event),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(transmute(activate_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_query_activatable<
        F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn query_activatable_trampoline<
            P,
            F: Fn(&P, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) -> bool + 'static,
        >(
            this: *mut gtk_source_sys::GtkSourceGutterRenderer,
            iter: *mut gtk_sys::GtkTextIter,
            area: *mut gdk_sys::GdkRectangle,
            event: *mut gdk_sys::GdkEvent,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GutterRenderer::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(iter),
                &from_glib_borrow(area),
                &from_glib_none(event),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"query-activatable\0".as_ptr() as *const _,
                Some(transmute(query_activatable_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_query_data<
        F: Fn(&Self, &gtk::TextIter, &gtk::TextIter, GutterRendererState) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn query_data_trampoline<
            P,
            F: Fn(&P, &gtk::TextIter, &gtk::TextIter, GutterRendererState) + 'static,
        >(
            this: *mut gtk_source_sys::GtkSourceGutterRenderer,
            start: *mut gtk_sys::GtkTextIter,
            end: *mut gtk_sys::GtkTextIter,
            state: gtk_source_sys::GtkSourceGutterRendererState,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GutterRenderer::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(start),
                &from_glib_borrow(end),
                from_glib(state),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"query-data\0".as_ptr() as *const _,
                Some(transmute(query_data_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_query_tooltip<
        F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, i32, i32, &gtk::Tooltip) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn query_tooltip_trampoline<
            P,
            F: Fn(&P, &gtk::TextIter, &gdk::Rectangle, i32, i32, &gtk::Tooltip) -> bool + 'static,
        >(
            this: *mut gtk_source_sys::GtkSourceGutterRenderer,
            iter: *mut gtk_sys::GtkTextIter,
            area: *mut gdk_sys::GdkRectangle,
            x: libc::c_int,
            y: libc::c_int,
            tooltip: *mut gtk_sys::GtkTooltip,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GutterRenderer::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(iter),
                &from_glib_borrow(area),
                x,
                y,
                &from_glib_borrow(tooltip),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"query-tooltip\0".as_ptr() as *const _,
                Some(transmute(query_tooltip_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_queue_draw<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn queue_draw_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceGutterRenderer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"queue-draw\0".as_ptr() as *const _,
                Some(transmute(queue_draw_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_alignment_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_alignment_mode_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceGutterRenderer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::alignment-mode\0".as_ptr() as *const _,
                Some(transmute(
                    notify_alignment_mode_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_background_rgba_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_background_rgba_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceGutterRenderer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::background-rgba\0".as_ptr() as *const _,
                Some(transmute(
                    notify_background_rgba_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_background_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_background_set_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceGutterRenderer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::background-set\0".as_ptr() as *const _,
                Some(transmute(
                    notify_background_set_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceGutterRenderer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::size\0".as_ptr() as *const _,
                Some(transmute(notify_size_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_view_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceGutterRenderer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::view\0".as_ptr() as *const _,
                Some(transmute(notify_view_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceGutterRenderer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible\0".as_ptr() as *const _,
                Some(transmute(notify_visible_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_window_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_window_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceGutterRenderer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::window-type\0".as_ptr() as *const _,
                Some(transmute(notify_window_type_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_xalign_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceGutterRenderer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::xalign\0".as_ptr() as *const _,
                Some(transmute(notify_xalign_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_xpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_xpad_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceGutterRenderer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::xpad\0".as_ptr() as *const _,
                Some(transmute(notify_xpad_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_yalign_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceGutterRenderer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::yalign\0".as_ptr() as *const _,
                Some(transmute(notify_yalign_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_ypad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ypad_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceGutterRenderer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ypad\0".as_ptr() as *const _,
                Some(transmute(notify_ypad_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for GutterRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GutterRenderer")
    }
}
