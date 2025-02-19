use libc::{c_int, size_t};
use std::io::prelude::*;
use std::ptr::NonNull;
use std::sync::mpsc;

use super::*;

pub struct VTerm {
    pub ptr: NonNull<ffi::VTerm>,

    pub screen_callbacks: Option<ffi::VTermScreenCallbacks>,
    pub screen_event_rx: Option<mpsc::Receiver<ScreenEvent>>,
    pub screen_event_tx: Option<mpsc::Sender<ScreenEvent>>,
    pub screen_ptr: NonNull<ffi::VTermScreen>,

    pub state_callbacks: Option<ffi::VTermStateCallbacks>,
    pub state_event_rx: Option<mpsc::Receiver<StateEvent>>,
    pub state_event_tx: Option<mpsc::Sender<StateEvent>>,
    pub state_ptr: NonNull<ffi::VTermState>,
}
unsafe impl Send for VTerm {}

impl VTerm {
    pub fn new(size: &Size) -> VTerm {
        // TODO how to detect error?
        let mut vterm_ptr = unsafe {
            NonNull::new(ffi::vterm_new(size.height as c_int, size.width as c_int)).unwrap()
        };
        let screen_ptr =
            unsafe { NonNull::new(ffi::vterm_obtain_screen(vterm_ptr.as_mut())).unwrap() };
        let state_ptr =
            unsafe { NonNull::new(ffi::vterm_obtain_state(vterm_ptr.as_mut())).unwrap() };

        let mut vterm = VTerm {
            ptr: vterm_ptr,
            screen_callbacks: None,
            screen_event_rx: None,
            screen_event_tx: None,
            screen_ptr: screen_ptr,

            state_callbacks: None,
            state_event_rx: None,
            state_event_tx: None,
            state_ptr: state_ptr,
        };

        vterm.screen_reset(true);

        vterm
    }

    pub fn get_size(&self) -> Size {
        let mut cols: c_int = 0;
        let mut rows: c_int = 0;
        unsafe {
            ffi::vterm_get_size(self.ptr.as_ptr(), &mut rows, &mut cols);
        }
        Size {
            height: rows as usize,
            width: cols as usize,
        }
    }

    pub fn set_size(&mut self, size: &Size) {
        unsafe {
            ffi::vterm_set_size(self.ptr.as_ptr(), size.height as c_int, size.width as c_int);
        }
    }

    pub fn get_utf8(&self) -> bool {
        unsafe { super::int_to_bool(ffi::vterm_get_utf8(self.ptr.as_ptr())) }
    }

    pub fn set_utf8(&mut self, is_utf8: bool) {
        unsafe { ffi::vterm_set_utf8(self.ptr.as_ptr(), super::bool_to_int(is_utf8)) }
    }
}

impl Write for VTerm {
    fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
        let size = unsafe {
            ffi::vterm_input_write(self.ptr.as_ptr(), buf.as_ptr(), buf.len() as size_t) as usize
        };
        Ok(size)
    }

    fn flush(&mut self) -> ::std::io::Result<()> {
        self.screen_flush_damage();
        Ok(())
    }
}

impl Drop for VTerm {
    fn drop(&mut self) {
        unsafe { ffi::vterm_free(self.ptr.as_ptr()) }
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::super::*;
    use std::io::prelude::*;

    #[test]
    fn vterm_can_create_and_destroy() {
        let vterm: VTerm = VTerm::new(&Size {
            height: 2,
            width: 2,
        });
        drop(vterm);
    }

    #[test]
    fn vterm_can_get_size() {
        let vterm: VTerm = VTerm::new(&Size {
            height: 2,
            width: 3,
        });
        let size = vterm.get_size();
        assert_eq!((2, 3), (size.height, size.width));
    }

    #[test]
    fn vterm_can_set_size() {
        let mut vterm: VTerm = VTerm::new(&Size {
            height: 2,
            width: 3,
        });
        vterm.set_size(&Size {
            height: 1,
            width: 2,
        });
        let size = vterm.get_size();
        assert_eq!((1, 2), (size.height, size.width));
    }

    #[test]
    fn vterm_can_get_and_set_utf8() {
        let mut vterm: VTerm = VTerm::new(&Size {
            height: 2,
            width: 2,
        });
        vterm.set_utf8(true);
        assert_eq!(true, vterm.get_utf8());

        vterm.set_utf8(false);
        assert_eq!(false, vterm.get_utf8());
    }

    #[test]
    fn vterm_can_write() {
        let mut vterm: VTerm = VTerm::new(&Size {
            height: 2,
            width: 2,
        });
        let input: &[u8] = "abcd".as_bytes();
        let result = vterm.write(input);
        assert!(result.is_ok());
        assert_eq!(4, result.unwrap());
    }
}
