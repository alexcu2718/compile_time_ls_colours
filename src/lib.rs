// Include the generated LS_COLORS_HASHMAP from the build script output directory.
include!(concat!(env!("OUT_DIR"), "/ls_colours.rs"));
#[cfg(test)]
mod test;

pub const NO_COLOUR: &[u8] = b"\x1b[0m"; // Reset colour code

/// Returns the colour code for a given file extension.
/// Falls back to `or_alternative` if the extension is not found in the colour map.
#[inline]
pub fn colour_path_or_alternative<'a>(extension: &'a [u8], or_alternative: &'a [u8]) -> &'a [u8] {
    //has at least the lifetime of the extension
    LS_COLOURS_HASHMAP
        .get(extension)
        .map(|v| &**v)
        .unwrap_or_else(|| or_alternative)
}

/// Returns the colour code for a given file extension if it exists in the colour map.
/// Returns `None` if not found.
#[inline]
pub fn colour_path<'a>(extension: &'a [u8]) -> Option<&'a [u8]> {
    LS_COLOURS_HASHMAP.get(extension).map(|v| &**v)
}

/// Like `colour_path_or_alternative`, but defaults to `NO_COLOR` if extension is not recognized.
/// This is useful for cases where you want to ensure a reset colour code is used
/// when the file type is not recognized.
#[inline]
pub fn colour_path_or_reset<'a>(extension: &'a [u8]) -> &'a [u8] {
    LS_COLOURS_HASHMAP
        .get(extension)
        .map(|v| &**v)
        .unwrap_or_else(|| NO_COLOUR)
}


/// Macro to generate a colour code for a file type based on its name/type.
///
/// It uses the `colour_path_or_alternative` function to retrieve a colour from
/// the internal LS_COLORS map. If the file type is not recognized,
///
/// Please note, you cannot use raw string literals arguments with this macro.
///
/// # Usage
///
/// ```rust
/// use compile_time_ls_colours::{file_type_colour, NO_COLOUR};
///
/// // Get colour for a symlink
/// let symlink_colour: &[u8] = file_type_colour!(symlink);
///
/// // Get colour for a directory
/// let dir_colour: &'static [u8] = file_type_colour!(directory);
/// let ext_rs= b"rs";
/// // Get colour for a known extension (e.g., b"rs")
/// let rs_colour: &'static [u8] = file_type_colour!(ext_rs);
/// // Get fallback colour if extension is not in the map
/// let unknown_colour: &'static [u8] = file_type_colour!(b"ext"); // defaults to NO_COLOUR if this extension is not recognised
///
///
/// let directory_colour: &'static [u8] = file_type_colour!(directory);
/// let symlink_colour: &'static [u8] = file_type_colour!(symlink);
/// ```
///
///
#[macro_export]
macro_rules! file_type_colour {
    (symlink) => {
        unsafe { $crate::colour_path(b"symlink").unwrap_unchecked() }
    };
    (directory) => {
        unsafe { $crate::colour_path(b"directory").unwrap_unchecked() }
    };
    (executable) => {
        unsafe { $crate::colour_path(b"executable").unwrap_unchecked() }
    };
    (socket) => {
        unsafe { $crate::colour_path(b"socket").unwrap_unchecked() }
    };
    (pipe) => {
        unsafe { $crate::colour_path(b"pipe").unwrap_unchecked() }
    };
    (block_device) => {
        unsafe { $crate::colour_path(b"block_device").unwrap_unchecked() }
    };
    (character_device) => {
        unsafe { $crate::colour_path(b"character_device").unwrap_unchecked() }
    };
    (sticky) => {
        unsafe { $crate::colour_path(b"sticky").unwrap_unchecked() }
    };
    (orphan_symlink) => {
        unsafe { $crate::colour_path(b"orphan_symlink").unwrap_unchecked()}
    };
    (setuid) => {
        unsafe { $crate::colour_path(b"setuid").unwrap_unchecked() }
    };
    (setgid) => {
        unsafe { $crate::colour_path(b"setgid").unwrap_unchecked() }
    };
    (other_writable) => {
        unsafe { $crate::colour_path(b"other_writable").unwrap_unchecked() }
    };
    ($other:expr) => {
        $crate::colour_path_or_alternative($other, $crate::NO_COLOUR)
    };
}

