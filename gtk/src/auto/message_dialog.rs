// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Dialog;
use MessageType;
use Widget;
use Window;
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
    pub struct MessageDialog(Object<ffi::GtkMessageDialog, ffi::GtkMessageDialogClass>): Dialog, Window, Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_message_dialog_get_type(),
    }
}

impl MessageDialog {
    //pub fn new<'a, 'b, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(parent: Q, flags: DialogFlags, type_: MessageType, buttons: ButtonsType, message_format: R, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> MessageDialog {
    //    unsafe { TODO: call ffi::gtk_message_dialog_new() }
    //}

    //pub fn new_with_markup<'a, 'b, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(parent: Q, flags: DialogFlags, type_: MessageType, buttons: ButtonsType, message_format: R, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> MessageDialog {
    //    unsafe { TODO: call ffi::gtk_message_dialog_new_with_markup() }
    //}
}

pub trait MessageDialogExt {
    //fn format_secondary_markup(&self, message_format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn format_secondary_text<'a, P: Into<Option<&'a str>>>(&self, message_format: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg_attr(feature = "v3_12", deprecated)]
    fn get_image(&self) -> Option<Widget>;

    fn get_message_area(&self) -> Option<Widget>;

    #[cfg_attr(feature = "v3_12", deprecated)]
    fn set_image<P: IsA<Widget>>(&self, image: &P);

    fn set_markup(&self, str: &str);

    fn get_property_message_type(&self) -> MessageType;

    fn set_property_message_type(&self, message_type: MessageType);

    fn get_property_secondary_text(&self) -> Option<String>;

    fn set_property_secondary_text(&self, secondary_text: Option<&str>);

    fn get_property_secondary_use_markup(&self) -> bool;

    fn set_property_secondary_use_markup(&self, secondary_use_markup: bool);

    fn get_property_text(&self) -> Option<String>;

    fn set_property_text(&self, text: Option<&str>);

    fn get_property_use_markup(&self) -> bool;

    fn set_property_use_markup(&self, use_markup: bool);

    fn connect_property_buttons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_12", deprecated)]
    fn connect_property_image_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_message_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_message_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_secondary_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_secondary_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MessageDialog> + IsA<glib::object::Object>> MessageDialogExt for O {
    //fn format_secondary_markup(&self, message_format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_message_dialog_format_secondary_markup() }
    //}

    //fn format_secondary_text<'a, P: Into<Option<&'a str>>>(&self, message_format: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_message_dialog_format_secondary_text() }
    //}

    fn get_image(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_message_dialog_get_image(self.to_glib_none().0))
        }
    }

    fn get_message_area(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_message_dialog_get_message_area(self.to_glib_none().0))
        }
    }

    fn set_image<P: IsA<Widget>>(&self, image: &P) {
        unsafe {
            ffi::gtk_message_dialog_set_image(self.to_glib_none().0, image.to_glib_none().0);
        }
    }

    fn set_markup(&self, str: &str) {
        unsafe {
            ffi::gtk_message_dialog_set_markup(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    fn get_property_message_type(&self) -> MessageType {
        unsafe {
            let mut value = Value::from_type(<MessageType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "message-type".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_message_type(&self, message_type: MessageType) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "message-type".to_glib_none().0, Value::from(&message_type).to_glib_none().0);
        }
    }

    fn get_property_secondary_text(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "secondary-text".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_secondary_text(&self, secondary_text: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "secondary-text".to_glib_none().0, Value::from(secondary_text).to_glib_none().0);
        }
    }

    fn get_property_secondary_use_markup(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "secondary-use-markup".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_secondary_use_markup(&self, secondary_use_markup: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "secondary-use-markup".to_glib_none().0, Value::from(&secondary_use_markup).to_glib_none().0);
        }
    }

    fn get_property_text(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "text".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "text".to_glib_none().0, Value::from(text).to_glib_none().0);
        }
    }

    fn get_property_use_markup(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "use-markup".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_use_markup(&self, use_markup: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "use-markup".to_glib_none().0, Value::from(&use_markup).to_glib_none().0);
        }
    }

    fn connect_property_buttons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::buttons",
                transmute(notify_buttons_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_image_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::image",
                transmute(notify_image_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_message_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::message-area",
                transmute(notify_message_area_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_message_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::message-type",
                transmute(notify_message_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_secondary_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::secondary-text",
                transmute(notify_secondary_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_secondary_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::secondary-use-markup",
                transmute(notify_secondary_use_markup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text",
                transmute(notify_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-markup",
                transmute(notify_use_markup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_buttons_trampoline<P>(this: *mut ffi::GtkMessageDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MessageDialog> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MessageDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_image_trampoline<P>(this: *mut ffi::GtkMessageDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MessageDialog> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MessageDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_message_area_trampoline<P>(this: *mut ffi::GtkMessageDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MessageDialog> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MessageDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_message_type_trampoline<P>(this: *mut ffi::GtkMessageDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MessageDialog> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MessageDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_secondary_text_trampoline<P>(this: *mut ffi::GtkMessageDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MessageDialog> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MessageDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_secondary_use_markup_trampoline<P>(this: *mut ffi::GtkMessageDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MessageDialog> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MessageDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_text_trampoline<P>(this: *mut ffi::GtkMessageDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MessageDialog> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MessageDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_use_markup_trampoline<P>(this: *mut ffi::GtkMessageDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MessageDialog> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MessageDialog::from_glib_borrow(this).downcast_unchecked())
}