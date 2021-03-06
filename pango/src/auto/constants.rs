// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use std::ffi::CStr;

#[cfg_attr(feature = "v1_38", deprecated = "Since 1.38")]
#[doc(alias = "PANGO_ENGINE_TYPE_LANG")]
pub static ENGINE_TYPE_LANG: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(ffi::PANGO_ENGINE_TYPE_LANG)
            .to_str()
            .unwrap()
    });
#[cfg_attr(feature = "v1_38", deprecated = "Since 1.38")]
#[doc(alias = "PANGO_ENGINE_TYPE_SHAPE")]
pub static ENGINE_TYPE_SHAPE: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(ffi::PANGO_ENGINE_TYPE_SHAPE)
            .to_str()
            .unwrap()
    });
#[cfg_attr(feature = "v1_38", deprecated = "Since 1.38")]
#[doc(alias = "PANGO_RENDER_TYPE_NONE")]
pub static RENDER_TYPE_NONE: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(ffi::PANGO_RENDER_TYPE_NONE)
            .to_str()
            .unwrap()
    });
