extern crate libc;

use libc::{c_int};

pub enum VTerm {}
pub enum VTermScreen {}
pub enum VTermScreenCell {}

#[repr(C)]
pub struct VTermPos {
    row: c_int,
    col: c_int,
}

extern {
    pub fn vterm_new(rows: c_int, cols: c_int) -> *mut VTerm;
    pub fn vterm_free(vterm: *mut VTerm);
    pub fn vterm_get_size(vterm: *const VTerm, rowsp: *mut c_int, colsp: *mut c_int);
    pub fn vterm_set_size(vterm: *mut VTerm, rows: c_int, cols: c_int);
    pub fn vterm_get_utf8(vterm: *const VTerm) -> c_int;
    pub fn vterm_set_utf8(vterm: *mut VTerm, is_utf8: c_int);
    pub fn vterm_obtain_screen(vterm: *mut VTerm) -> *mut VTermScreen;
    pub fn vterm_input_write(vterm: *mut VTerm, bytes: *const libc::c_char, len: libc::size_t) -> libc::size_t;

    pub fn vterm_screen_reset(screen: *mut VTermScreen, hard: c_int);
    pub fn vterm_screen_get_cell(screen: *const VTermScreen, pos: VTermPos, cell: *mut VTermScreenCell) -> c_int;

    // These are my rust ffi bitfield workarounds
    pub fn vterm_cell_new(vterm: *const VTerm) -> *mut VTermScreenCell;
    pub fn vterm_cell_free(vterm: *const VTerm, cell: *mut VTermScreenCell);
    pub fn vterm_cell_get_width(cell: *const VTermScreenCell) -> libc::c_char;
    pub fn vterm_cell_set_width(cell: *mut VTermScreenCell, width: libc::c_char);
    pub fn vterm_cell_get_bold(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_bold(cell: *mut VTermScreenCell, is_bold: libc::c_uint);
    pub fn vterm_cell_get_underline(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_underline(cell: *mut VTermScreenCell, underline_value: libc::c_uint);
    pub fn vterm_cell_get_italic(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_italic(cell: *mut VTermScreenCell, is_italic: libc::c_uint);
    pub fn vterm_cell_get_blink(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_blink(cell: *mut VTermScreenCell, is_blink: libc::c_uint);
    pub fn vterm_cell_get_reverse(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_reverse(cell: *mut VTermScreenCell, is_reverse: libc::c_uint);
    pub fn vterm_cell_get_strike(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_strike(cell: *mut VTermScreenCell, is_strike: libc::c_uint);
    pub fn vterm_cell_get_font(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_font(cell: *mut VTermScreenCell, font_value: libc::c_uint);
    pub fn vterm_cell_get_dwl(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_dwl(cell: *mut VTermScreenCell, dwl: libc::c_uint);
    pub fn vterm_cell_get_dhl(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_dhl(cell: *mut VTermScreenCell, dhl: libc::c_uint);
}

mod tests {
    extern crate libc;

    use libc::{c_int};
    use super::*;

    #[test]
    fn vterm_can_create_and_destroy() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn vterm_can_get_size() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let mut cols: c_int = 0;
            let mut rows: c_int = 0;
            vterm_get_size(vterm_ptr, &mut cols, &mut rows);
            assert_eq!((2, 2), (cols, rows));

            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn vterm_can_set_size() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            vterm_set_size(vterm_ptr, 1, 1);

            let mut cols: c_int = 0;
            let mut rows: c_int = 0;
            vterm_get_size(vterm_ptr, &mut cols, &mut rows);
            assert_eq!((1, 1), (cols, rows));

            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn vterm_can_get_and_set_utf8() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);

            vterm_set_utf8(vterm_ptr, 1);
            let val = vterm_get_utf8(vterm_ptr);
            assert_eq!(1, val); // not sure why this doesnt work

            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn vterm_can_obtain_screen() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            vterm_obtain_screen(vterm_ptr);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn vterm_can_write_input() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);

            // there probably a nicer way to do this
            let input = [
                b'a' as libc::c_char,
                b'b' as libc::c_char,
                b'c' as libc::c_char,
            ];
            let bytes_read = vterm_input_write(vterm_ptr, input.as_ptr(), 3);
            assert_eq!(3, bytes_read);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn screen_can_reset() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let screen_ptr = vterm_obtain_screen(vterm_ptr);
            vterm_screen_reset(screen_ptr, 1);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn screen_can_get_cell() {
        unsafe {
            // TODO: write something so the cell will have a known value
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let screen_ptr = vterm_obtain_screen(vterm_ptr);
            let pos = VTermPos { row: 1, col: 0 };
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new(vterm_ptr);
            let ret = vterm_screen_get_cell(screen_ptr, pos, cell_ptr);
            assert_eq!(0, ret);

            vterm_cell_free(vterm_ptr, cell_ptr);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn cell_can_create_and_destroy() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new(vterm_ptr);
            vterm_cell_free(vterm_ptr, cell_ptr);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn cell_can_get_and_set_width() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new(vterm_ptr);

            vterm_cell_set_width(cell_ptr, 2);
            assert_eq!(2, vterm_cell_get_width(cell_ptr));

            vterm_cell_set_width(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_width(cell_ptr));

            vterm_cell_free(vterm_ptr, cell_ptr);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn cell_can_get_and_set_bold() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new(vterm_ptr);

            vterm_cell_set_bold(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_bold(cell_ptr));

            vterm_cell_set_bold(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_bold(cell_ptr));

            vterm_cell_free(vterm_ptr, cell_ptr);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn cell_can_get_and_set_underline() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new(vterm_ptr);

            vterm_cell_set_underline(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_underline(cell_ptr));
            vterm_cell_set_underline(cell_ptr, 2);
            assert_eq!(2, vterm_cell_get_underline(cell_ptr));
            vterm_cell_set_underline(cell_ptr, 3);
            assert_eq!(3, vterm_cell_get_underline(cell_ptr));
            vterm_cell_set_underline(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_underline(cell_ptr));

            vterm_cell_free(vterm_ptr, cell_ptr);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn cell_can_get_and_set_italic() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new(vterm_ptr);

            vterm_cell_set_italic(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_italic(cell_ptr));
            vterm_cell_set_italic(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_italic(cell_ptr));

            vterm_cell_free(vterm_ptr, cell_ptr);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn cell_can_get_and_set_blink() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new(vterm_ptr);

            vterm_cell_set_blink(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_blink(cell_ptr));
            vterm_cell_set_blink(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_blink(cell_ptr));

            vterm_cell_free(vterm_ptr, cell_ptr);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn cell_can_get_and_set_reverse() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new(vterm_ptr);

            vterm_cell_set_reverse(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_reverse(cell_ptr));
            vterm_cell_set_reverse(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_reverse(cell_ptr));

            vterm_cell_free(vterm_ptr, cell_ptr);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn cell_can_get_and_set_strike() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new(vterm_ptr);

            vterm_cell_set_strike(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_strike(cell_ptr));
            vterm_cell_set_strike(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_strike(cell_ptr));

            vterm_cell_free(vterm_ptr, cell_ptr);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn cell_can_get_and_set_font() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new(vterm_ptr);

            vterm_cell_set_font(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_font(cell_ptr));
            vterm_cell_set_font(cell_ptr, 5);
            assert_eq!(5, vterm_cell_get_font(cell_ptr));
            vterm_cell_set_font(cell_ptr, 9);
            assert_eq!(9, vterm_cell_get_font(cell_ptr));
            vterm_cell_set_font(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_font(cell_ptr));

            vterm_cell_free(vterm_ptr, cell_ptr);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn cell_can_get_and_set_dwl() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new(vterm_ptr);

            vterm_cell_set_dwl(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_dwl(cell_ptr));
            vterm_cell_set_dwl(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_dwl(cell_ptr));

            vterm_cell_free(vterm_ptr, cell_ptr);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn cell_can_get_and_set_dhl() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new(vterm_ptr);

            vterm_cell_set_dhl(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_dhl(cell_ptr));
            vterm_cell_set_dhl(cell_ptr, 2);
            assert_eq!(2, vterm_cell_get_dhl(cell_ptr));
            vterm_cell_set_dhl(cell_ptr, 3);
            assert_eq!(3, vterm_cell_get_dhl(cell_ptr));
            vterm_cell_set_dhl(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_dhl(cell_ptr));

            vterm_cell_free(vterm_ptr, cell_ptr);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn cell_bit_fields_dont_interact_with_each_other() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new(vterm_ptr);

            vterm_cell_set_bold(cell_ptr, 1);
            vterm_cell_set_underline(cell_ptr, 0);
            vterm_cell_set_italic(cell_ptr, 1);
            vterm_cell_set_blink(cell_ptr, 0);
            vterm_cell_set_reverse(cell_ptr, 1);
            vterm_cell_set_strike(cell_ptr, 1);
            vterm_cell_set_font(cell_ptr, 7);
            vterm_cell_set_dwl(cell_ptr, 1);
            vterm_cell_set_dhl(cell_ptr, 2);
            assert_eq!(1, vterm_cell_get_bold(cell_ptr));
            assert_eq!(0, vterm_cell_get_underline(cell_ptr));
            assert_eq!(1, vterm_cell_get_italic(cell_ptr));
            assert_eq!(0, vterm_cell_get_blink(cell_ptr));
            assert_eq!(1, vterm_cell_get_reverse(cell_ptr));
            assert_eq!(1, vterm_cell_get_strike(cell_ptr));
            assert_eq!(7, vterm_cell_get_font(cell_ptr));
            assert_eq!(1, vterm_cell_get_dwl(cell_ptr));
            assert_eq!(2, vterm_cell_get_dhl(cell_ptr));

            vterm_cell_set_blink(cell_ptr, 1);
            vterm_cell_set_strike(cell_ptr, 0);
            vterm_cell_set_dwl(cell_ptr, 0);
            vterm_cell_set_dhl(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_bold(cell_ptr));
            assert_eq!(0, vterm_cell_get_underline(cell_ptr));
            assert_eq!(1, vterm_cell_get_italic(cell_ptr));
            assert_eq!(1, vterm_cell_get_blink(cell_ptr));
            assert_eq!(1, vterm_cell_get_reverse(cell_ptr));
            assert_eq!(0, vterm_cell_get_strike(cell_ptr));
            assert_eq!(7, vterm_cell_get_font(cell_ptr));
            assert_eq!(0, vterm_cell_get_dwl(cell_ptr));
            assert_eq!(1, vterm_cell_get_dhl(cell_ptr));

            vterm_cell_free(vterm_ptr, cell_ptr);
            vterm_free(vterm_ptr);
        }
    }
}
