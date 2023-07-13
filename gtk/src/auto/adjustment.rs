// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib;
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
    pub struct Adjustment(Object<ffi::GtkAdjustment, ffi::GtkAdjustmentClass>);

    match fn {
        get_type => || ffi::gtk_adjustment_get_type(),
    }
}

impl Adjustment {
    pub fn new(value: f64, lower: f64, upper: f64, step_increment: f64, page_increment: f64, page_size: f64) -> Adjustment {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_adjustment_new(value, lower, upper, step_increment, page_increment, page_size))
        }
    }
}

pub trait AdjustmentExt {
    #[cfg_attr(feature = "v3_18", deprecated)]
    fn changed(&self);

    fn clamp_page(&self, lower: f64, upper: f64);

    fn configure(&self, value: f64, lower: f64, upper: f64, step_increment: f64, page_increment: f64, page_size: f64);

    fn get_lower(&self) -> f64;

    fn get_minimum_increment(&self) -> f64;

    fn get_page_increment(&self) -> f64;

    fn get_page_size(&self) -> f64;

    fn get_step_increment(&self) -> f64;

    fn get_upper(&self) -> f64;

    fn get_value(&self) -> f64;

    fn set_lower(&self, lower: f64);

    fn set_page_increment(&self, page_increment: f64);

    fn set_page_size(&self, page_size: f64);

    fn set_step_increment(&self, step_increment: f64);

    fn set_upper(&self, upper: f64);

    fn set_value(&self, value: f64);

    #[cfg_attr(feature = "v3_18", deprecated)]
    fn value_changed(&self);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_value_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_lower_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_page_increment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_step_increment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_upper_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Adjustment> + IsA<glib::object::Object>> AdjustmentExt for O {
    fn changed(&self) {
        unsafe {
            ffi::gtk_adjustment_changed(self.to_glib_none().0);
        }
    }

    fn clamp_page(&self, lower: f64, upper: f64) {
        unsafe {
            ffi::gtk_adjustment_clamp_page(self.to_glib_none().0, lower, upper);
        }
    }

    fn configure(&self, value: f64, lower: f64, upper: f64, step_increment: f64, page_increment: f64, page_size: f64) {
        unsafe {
            ffi::gtk_adjustment_configure(self.to_glib_none().0, value, lower, upper, step_increment, page_increment, page_size);
        }
    }

    fn get_lower(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_lower(self.to_glib_none().0)
        }
    }

    fn get_minimum_increment(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_minimum_increment(self.to_glib_none().0)
        }
    }

    fn get_page_increment(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_page_increment(self.to_glib_none().0)
        }
    }

    fn get_page_size(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_page_size(self.to_glib_none().0)
        }
    }

    fn get_step_increment(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_step_increment(self.to_glib_none().0)
        }
    }

    fn get_upper(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_upper(self.to_glib_none().0)
        }
    }

    fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_value(self.to_glib_none().0)
        }
    }

    fn set_lower(&self, lower: f64) {
        unsafe {
            ffi::gtk_adjustment_set_lower(self.to_glib_none().0, lower);
        }
    }

    fn set_page_increment(&self, page_increment: f64) {
        unsafe {
            ffi::gtk_adjustment_set_page_increment(self.to_glib_none().0, page_increment);
        }
    }

    fn set_page_size(&self, page_size: f64) {
        unsafe {
            ffi::gtk_adjustment_set_page_size(self.to_glib_none().0, page_size);
        }
    }

    fn set_step_increment(&self, step_increment: f64) {
        unsafe {
            ffi::gtk_adjustment_set_step_increment(self.to_glib_none().0, step_increment);
        }
    }

    fn set_upper(&self, upper: f64) {
        unsafe {
            ffi::gtk_adjustment_set_upper(self.to_glib_none().0, upper);
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_adjustment_set_value(self.to_glib_none().0, value);
        }
    }

    fn value_changed(&self) {
        unsafe {
            ffi::gtk_adjustment_value_changed(self.to_glib_none().0);
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_value_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "value-changed",
                transmute(value_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_lower_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::lower",
                transmute(notify_lower_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_page_increment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::page-increment",
                transmute(notify_page_increment_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::page-size",
                transmute(notify_page_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_step_increment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::step-increment",
                transmute(notify_step_increment_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_upper_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::upper",
                transmute(notify_upper_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::value",
                transmute(notify_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline<P>(this: *mut ffi::GtkAdjustment, f: glib_ffi::gpointer)
where P: IsA<Adjustment> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Adjustment::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn value_changed_trampoline<P>(this: *mut ffi::GtkAdjustment, f: glib_ffi::gpointer)
where P: IsA<Adjustment> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Adjustment::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_lower_trampoline<P>(this: *mut ffi::GtkAdjustment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Adjustment> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Adjustment::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_page_increment_trampoline<P>(this: *mut ffi::GtkAdjustment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Adjustment> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Adjustment::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_page_size_trampoline<P>(this: *mut ffi::GtkAdjustment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Adjustment> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Adjustment::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_step_increment_trampoline<P>(this: *mut ffi::GtkAdjustment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Adjustment> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Adjustment::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_upper_trampoline<P>(this: *mut ffi::GtkAdjustment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Adjustment> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Adjustment::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_value_trampoline<P>(this: *mut ffi::GtkAdjustment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Adjustment> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Adjustment::from_glib_borrow(this).downcast_unchecked())
}
