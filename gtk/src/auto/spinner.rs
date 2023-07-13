// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
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
    pub struct Spinner(Object<ffi::GtkSpinner, ffi::GtkSpinnerClass>): Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_spinner_get_type(),
    }
}

impl Spinner {
    pub fn new() -> Spinner {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_spinner_new()).downcast_unchecked()
        }
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SpinnerExt {
    fn start(&self);

    fn stop(&self);

    fn get_property_active(&self) -> bool;

    fn set_property_active(&self, active: bool);

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Spinner> + IsA<glib::object::Object>> SpinnerExt for O {
    fn start(&self) {
        unsafe {
            ffi::gtk_spinner_start(self.to_glib_none().0);
        }
    }

    fn stop(&self) {
        unsafe {
            ffi::gtk_spinner_stop(self.to_glib_none().0);
        }
    }

    fn get_property_active(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "active".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_active(&self, active: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "active".to_glib_none().0, Value::from(&active).to_glib_none().0);
        }
    }

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::active",
                transmute(notify_active_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_active_trampoline<P>(this: *mut ffi::GtkSpinner, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Spinner> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Spinner::from_glib_borrow(this).downcast_unchecked())
}