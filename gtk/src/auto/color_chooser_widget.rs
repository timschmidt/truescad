// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Box;
use Buildable;
use ColorChooser;
use Container;
use Orientable;
use Widget;
use ffi;
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
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ColorChooserWidget(Object<ffi::GtkColorChooserWidget, ffi::GtkColorChooserWidgetClass>): Box, Container, Widget, Buildable, Orientable, ColorChooser;

    match fn {
        get_type => || ffi::gtk_color_chooser_widget_get_type(),
    }
}

impl ColorChooserWidget {
    pub fn new() -> ColorChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_color_chooser_widget_new()).downcast_unchecked()
        }
    }
}

impl Default for ColorChooserWidget {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ColorChooserWidgetExt {
    fn get_property_show_editor(&self) -> bool;

    fn set_property_show_editor(&self, show_editor: bool);

    fn connect_property_show_editor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ColorChooserWidget> + IsA<glib::object::Object>> ColorChooserWidgetExt for O {
    fn get_property_show_editor(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-editor".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_show_editor(&self, show_editor: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-editor".to_glib_none().0, Value::from(&show_editor).to_glib_none().0);
        }
    }

    fn connect_property_show_editor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-editor",
                transmute(notify_show_editor_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_show_editor_trampoline<P>(this: *mut ffi::GtkColorChooserWidget, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ColorChooserWidget> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ColorChooserWidget::from_glib_borrow(this).downcast_unchecked())
}