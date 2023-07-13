// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_10", feature = "dox"))]
use BaselinePosition;
use Buildable;
use Container;
use Orientable;
use Orientation;
use PackType;
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
    pub struct Box(Object<ffi::GtkBox, ffi::GtkBoxClass>): Container, Widget, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_box_get_type(),
    }
}

impl Box {
    pub fn new(orientation: Orientation, spacing: i32) -> Box {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_box_new(orientation.to_glib(), spacing)).downcast_unchecked()
        }
    }
}

pub trait BoxExt {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_baseline_position(&self) -> BaselinePosition;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_center_widget(&self) -> Option<Widget>;

    fn get_homogeneous(&self) -> bool;

    fn get_spacing(&self) -> i32;

    fn pack_end<P: IsA<Widget>>(&self, child: &P, expand: bool, fill: bool, padding: u32);

    fn pack_start<P: IsA<Widget>>(&self, child: &P, expand: bool, fill: bool, padding: u32);

    fn query_child_packing<P: IsA<Widget>>(&self, child: &P) -> (bool, bool, u32, PackType);

    fn reorder_child<P: IsA<Widget>>(&self, child: &P, position: i32);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_baseline_position(&self, position: BaselinePosition);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_center_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, widget: Q);

    fn set_child_packing<P: IsA<Widget>>(&self, child: &P, expand: bool, fill: bool, padding: u32, pack_type: PackType);

    fn set_homogeneous(&self, homogeneous: bool);

    fn set_spacing(&self, spacing: i32);

    #[doc(hidden)]
    fn get_child_expand<T: IsA<Widget>>(&self, item: &T) -> bool;

    #[doc(hidden)]
    fn set_child_expand<T: IsA<Widget>>(&self, item: &T, expand: bool);

    #[doc(hidden)]
    fn get_child_fill<T: IsA<Widget>>(&self, item: &T) -> bool;

    #[doc(hidden)]
    fn set_child_fill<T: IsA<Widget>>(&self, item: &T, fill: bool);

    #[doc(hidden)]
    fn get_child_pack_type<T: IsA<Widget>>(&self, item: &T) -> PackType;

    #[doc(hidden)]
    fn set_child_pack_type<T: IsA<Widget>>(&self, item: &T, pack_type: PackType);

    #[doc(hidden)]
    fn get_child_padding<T: IsA<Widget>>(&self, item: &T) -> u32;

    #[doc(hidden)]
    fn set_child_padding<T: IsA<Widget>>(&self, item: &T, padding: u32);

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_baseline_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Box> + IsA<Container> + IsA<glib::object::Object>> BoxExt for O {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_baseline_position(&self) -> BaselinePosition {
        unsafe {
            from_glib(ffi::gtk_box_get_baseline_position(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_center_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_box_get_center_widget(self.to_glib_none().0))
        }
    }

    fn get_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_box_get_homogeneous(self.to_glib_none().0))
        }
    }

    fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_box_get_spacing(self.to_glib_none().0)
        }
    }

    fn pack_end<P: IsA<Widget>>(&self, child: &P, expand: bool, fill: bool, padding: u32) {
        unsafe {
            ffi::gtk_box_pack_end(self.to_glib_none().0, child.to_glib_none().0, expand.to_glib(), fill.to_glib(), padding);
        }
    }

    fn pack_start<P: IsA<Widget>>(&self, child: &P, expand: bool, fill: bool, padding: u32) {
        unsafe {
            ffi::gtk_box_pack_start(self.to_glib_none().0, child.to_glib_none().0, expand.to_glib(), fill.to_glib(), padding);
        }
    }

    fn query_child_packing<P: IsA<Widget>>(&self, child: &P) -> (bool, bool, u32, PackType) {
        unsafe {
            let mut expand = mem::uninitialized();
            let mut fill = mem::uninitialized();
            let mut padding = mem::uninitialized();
            let mut pack_type = mem::uninitialized();
            ffi::gtk_box_query_child_packing(self.to_glib_none().0, child.to_glib_none().0, &mut expand, &mut fill, &mut padding, &mut pack_type);
            (from_glib(expand), from_glib(fill), padding, from_glib(pack_type))
        }
    }

    fn reorder_child<P: IsA<Widget>>(&self, child: &P, position: i32) {
        unsafe {
            ffi::gtk_box_reorder_child(self.to_glib_none().0, child.to_glib_none().0, position);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_baseline_position(&self, position: BaselinePosition) {
        unsafe {
            ffi::gtk_box_set_baseline_position(self.to_glib_none().0, position.to_glib());
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_center_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, widget: Q) {
        let widget = widget.into();
        let widget = widget.to_glib_none();
        unsafe {
            ffi::gtk_box_set_center_widget(self.to_glib_none().0, widget.0);
        }
    }

    fn set_child_packing<P: IsA<Widget>>(&self, child: &P, expand: bool, fill: bool, padding: u32, pack_type: PackType) {
        unsafe {
            ffi::gtk_box_set_child_packing(self.to_glib_none().0, child.to_glib_none().0, expand.to_glib(), fill.to_glib(), padding, pack_type.to_glib());
        }
    }

    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_box_set_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_box_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[doc(hidden)]
    fn get_child_expand<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "expand".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[doc(hidden)]
    fn set_child_expand<T: IsA<Widget>>(&self, item: &T, expand: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "expand".to_glib_none().0, Value::from(&expand).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_fill<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "fill".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[doc(hidden)]
    fn set_child_fill<T: IsA<Widget>>(&self, item: &T, fill: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "fill".to_glib_none().0, Value::from(&fill).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_pack_type<T: IsA<Widget>>(&self, item: &T) -> PackType {
        unsafe {
            let mut value = Value::from_type(<PackType as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "pack-type".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[doc(hidden)]
    fn set_child_pack_type<T: IsA<Widget>>(&self, item: &T, pack_type: PackType) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "pack-type".to_glib_none().0, Value::from(&pack_type).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_padding<T: IsA<Widget>>(&self, item: &T) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "padding".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[doc(hidden)]
    fn set_child_padding<T: IsA<Widget>>(&self, item: &T, padding: u32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "padding".to_glib_none().0, Value::from(&padding).to_glib_none().0);
        }
    }

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, Value::from(&position).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_baseline_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::baseline-position",
                transmute(notify_baseline_position_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::homogeneous",
                transmute(notify_homogeneous_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::spacing",
                transmute(notify_spacing_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
unsafe extern "C" fn notify_baseline_position_trampoline<P>(this: *mut ffi::GtkBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Box> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Box::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_homogeneous_trampoline<P>(this: *mut ffi::GtkBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Box> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Box::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_spacing_trampoline<P>(this: *mut ffi::GtkBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Box> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Box::from_glib_borrow(this).downcast_unchecked())
}