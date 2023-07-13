// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Clipboard;
use SelectionData;
use TargetEntry;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi::gpointer;
use libc::c_uint;
use std::boxed::Box as Box_;
use std::mem::transmute;

pub trait ClipboardExtManual {
    fn set_with_data<F: Fn(&Clipboard, &SelectionData, u32) + 'static>(&self, targets: &[TargetEntry], f: F) -> bool;
}

impl<O: IsA<Clipboard>> ClipboardExtManual for O {
    fn set_with_data<F: Fn(&Clipboard, &SelectionData, u32) + 'static>(&self, targets: &[TargetEntry], f: F) -> bool {
        let stashed_targets: Vec<_> = targets.iter().map(|e| e.to_glib_none()).collect();
        let mut t = Vec::with_capacity(stashed_targets.len());
        for stash in &stashed_targets {
            unsafe {
                t.push(ffi::GtkTargetEntry {
                    target: (*stash.0).target,
                    flags: (*stash.0).flags,
                    info: (*stash.0).info,
                });
            }
        }
        let t_ptr: *mut ffi::GtkTargetEntry = t.as_mut_ptr();
        let f: Box_<Box_<Fn(&Clipboard, &SelectionData, u32) + 'static>> = Box_::new(Box_::new(f));
        let user_data = Box_::into_raw(f) as *mut _;
        let success : bool = unsafe { from_glib(ffi::gtk_clipboard_set_with_data(self.to_glib_none().0,
                                             t_ptr, t.len() as c_uint, 
                                             Some(trampoline), Some(cleanup), user_data))
        };
        if !success {
            // Cleanup function is not called in case of a failure.
            unsafe { Box_::<Box_<Fn(&Clipboard, &SelectionData, u32) + 'static>>::from_raw(user_data as *mut _); }
        }
        success
    }
}

unsafe extern "C" fn trampoline(clipboard: *mut ffi::GtkClipboard, selection_data: *mut ffi::GtkSelectionData, info: c_uint, user_data: gpointer) {
    let f: &&(Fn(&Clipboard, &SelectionData, u32) + 'static) = transmute(user_data);
    f(&from_glib_borrow(clipboard), &from_glib_borrow(selection_data), info);
}


unsafe extern "C" fn cleanup(_clipboard: *mut ffi::GtkClipboard, user_data: gpointer) {
    Box_::<Box_<Fn(&Clipboard, &SelectionData, u32) + 'static>>::from_raw(user_data as *mut _);
}
