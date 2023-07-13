// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::IsA;
use glib::translate::*;

use PadActionEntry;
use PadController;

pub trait PadControllerExtManual {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_action_entries(&self, entries: &[PadActionEntry]);
}

impl<O: IsA<PadController>> PadControllerExtManual for O {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_action_entries(&self, entries: &[PadActionEntry]) {
        let n_entries = entries.len() as i32;
        let entries = entries.as_ptr();
        unsafe {
            ffi::gtk_pad_controller_set_action_entries(self.to_glib_none().0,
                                                       mut_override(entries as *const _),
                                                       n_entries);
        }
    }
}
