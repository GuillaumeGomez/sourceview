// This file was generated by gir (fdeaa47) from gir-files (2e2a9ca)
// DO NOT EDIT

use ffi;
use glib::translate::*;

#[cfg(feature = "v3_16")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BackgroundPatternType {
    None,
    Grid,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[cfg(feature = "v3_16")]
#[doc(hidden)]
impl ToGlib for BackgroundPatternType {
    type GlibType = ffi::GtkSourceBackgroundPatternType;

    fn to_glib(&self) -> ffi::GtkSourceBackgroundPatternType {
        match *self {
            BackgroundPatternType::None => ffi::GTK_SOURCE_BACKGROUND_PATTERN_TYPE_NONE,
            BackgroundPatternType::Grid => ffi::GTK_SOURCE_BACKGROUND_PATTERN_TYPE_GRID,
            BackgroundPatternType::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[cfg(feature = "v3_16")]
#[doc(hidden)]
impl FromGlib<ffi::GtkSourceBackgroundPatternType> for BackgroundPatternType {
    fn from_glib(value: ffi::GtkSourceBackgroundPatternType) -> Self {
        match value {
            ffi::GTK_SOURCE_BACKGROUND_PATTERN_TYPE_NONE => BackgroundPatternType::None,
            ffi::GTK_SOURCE_BACKGROUND_PATTERN_TYPE_GRID => BackgroundPatternType::Grid,
        }
    }
}

#[cfg(feature = "v3_12")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ChangeCaseType {
    Lower,
    Upper,
    Toggle,
    Title,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[cfg(feature = "v3_12")]
#[doc(hidden)]
impl ToGlib for ChangeCaseType {
    type GlibType = ffi::GtkSourceChangeCaseType;

    fn to_glib(&self) -> ffi::GtkSourceChangeCaseType {
        match *self {
            ChangeCaseType::Lower => ffi::GTK_SOURCE_CHANGE_CASE_LOWER,
            ChangeCaseType::Upper => ffi::GTK_SOURCE_CHANGE_CASE_UPPER,
            ChangeCaseType::Toggle => ffi::GTK_SOURCE_CHANGE_CASE_TOGGLE,
            ChangeCaseType::Title => ffi::GTK_SOURCE_CHANGE_CASE_TITLE,
            ChangeCaseType::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[cfg(feature = "v3_12")]
#[doc(hidden)]
impl FromGlib<ffi::GtkSourceChangeCaseType> for ChangeCaseType {
    fn from_glib(value: ffi::GtkSourceChangeCaseType) -> Self {
        match value {
            ffi::GTK_SOURCE_CHANGE_CASE_LOWER => ChangeCaseType::Lower,
            ffi::GTK_SOURCE_CHANGE_CASE_UPPER => ChangeCaseType::Upper,
            ffi::GTK_SOURCE_CHANGE_CASE_TOGGLE => ChangeCaseType::Toggle,
            ffi::GTK_SOURCE_CHANGE_CASE_TITLE => ChangeCaseType::Title,
        }
    }
}

#[cfg(feature = "v3_14")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CompressionType {
    None,
    Gzip,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[cfg(feature = "v3_14")]
#[doc(hidden)]
impl ToGlib for CompressionType {
    type GlibType = ffi::GtkSourceCompressionType;

    fn to_glib(&self) -> ffi::GtkSourceCompressionType {
        match *self {
            CompressionType::None => ffi::GTK_SOURCE_COMPRESSION_TYPE_NONE,
            CompressionType::Gzip => ffi::GTK_SOURCE_COMPRESSION_TYPE_GZIP,
            CompressionType::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[cfg(feature = "v3_14")]
#[doc(hidden)]
impl FromGlib<ffi::GtkSourceCompressionType> for CompressionType {
    fn from_glib(value: ffi::GtkSourceCompressionType) -> Self {
        match value {
            ffi::GTK_SOURCE_COMPRESSION_TYPE_NONE => CompressionType::None,
            ffi::GTK_SOURCE_COMPRESSION_TYPE_GZIP => CompressionType::Gzip,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GutterRendererAlignmentMode {
    Cell,
    First,
    Last,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for GutterRendererAlignmentMode {
    type GlibType = ffi::GtkSourceGutterRendererAlignmentMode;

    fn to_glib(&self) -> ffi::GtkSourceGutterRendererAlignmentMode {
        match *self {
            GutterRendererAlignmentMode::Cell => ffi::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_CELL,
            GutterRendererAlignmentMode::First => ffi::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_FIRST,
            GutterRendererAlignmentMode::Last => ffi::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_LAST,
            GutterRendererAlignmentMode::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceGutterRendererAlignmentMode> for GutterRendererAlignmentMode {
    fn from_glib(value: ffi::GtkSourceGutterRendererAlignmentMode) -> Self {
        match value {
            ffi::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_CELL => GutterRendererAlignmentMode::Cell,
            ffi::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_FIRST => GutterRendererAlignmentMode::First,
            ffi::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_LAST => GutterRendererAlignmentMode::Last,
        }
    }
}

#[cfg(feature = "v3_14")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum NewlineType {
    Lf,
    Cr,
    CrLf,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[cfg(feature = "v3_14")]
#[doc(hidden)]
impl ToGlib for NewlineType {
    type GlibType = ffi::GtkSourceNewlineType;

    fn to_glib(&self) -> ffi::GtkSourceNewlineType {
        match *self {
            NewlineType::Lf => ffi::GTK_SOURCE_NEWLINE_TYPE_LF,
            NewlineType::Cr => ffi::GTK_SOURCE_NEWLINE_TYPE_CR,
            NewlineType::CrLf => ffi::GTK_SOURCE_NEWLINE_TYPE_CR_LF,
            NewlineType::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[cfg(feature = "v3_14")]
#[doc(hidden)]
impl FromGlib<ffi::GtkSourceNewlineType> for NewlineType {
    fn from_glib(value: ffi::GtkSourceNewlineType) -> Self {
        match value {
            ffi::GTK_SOURCE_NEWLINE_TYPE_LF => NewlineType::Lf,
            ffi::GTK_SOURCE_NEWLINE_TYPE_CR => NewlineType::Cr,
            ffi::GTK_SOURCE_NEWLINE_TYPE_CR_LF => NewlineType::CrLf,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SmartHomeEndType {
    Disabled,
    Before,
    After,
    Always,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for SmartHomeEndType {
    type GlibType = ffi::GtkSourceSmartHomeEndType;

    fn to_glib(&self) -> ffi::GtkSourceSmartHomeEndType {
        match *self {
            SmartHomeEndType::Disabled => ffi::GTK_SOURCE_SMART_HOME_END_DISABLED,
            SmartHomeEndType::Before => ffi::GTK_SOURCE_SMART_HOME_END_BEFORE,
            SmartHomeEndType::After => ffi::GTK_SOURCE_SMART_HOME_END_AFTER,
            SmartHomeEndType::Always => ffi::GTK_SOURCE_SMART_HOME_END_ALWAYS,
            SmartHomeEndType::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceSmartHomeEndType> for SmartHomeEndType {
    fn from_glib(value: ffi::GtkSourceSmartHomeEndType) -> Self {
        match value {
            ffi::GTK_SOURCE_SMART_HOME_END_DISABLED => SmartHomeEndType::Disabled,
            ffi::GTK_SOURCE_SMART_HOME_END_BEFORE => SmartHomeEndType::Before,
            ffi::GTK_SOURCE_SMART_HOME_END_AFTER => SmartHomeEndType::After,
            ffi::GTK_SOURCE_SMART_HOME_END_ALWAYS => SmartHomeEndType::Always,
        }
    }
}
