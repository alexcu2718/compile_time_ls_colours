// Include the generated LS_COLORS_HASHMAP from the build script output directory.
include!(concat!(env!("OUT_DIR"), "/ls_colors.rs"));
#[cfg(test)]
mod test;


pub const NO_COLOUR: &[u8] = b"\x1b[0m"; // Reset color code
// Default color code for symbolic links.
pub const DEFAULT_SYMLINK_COLOR: &[u8] = b"\x1b[38;2;230;150;60m";

// Default colour code for directories.
pub const DEFAULT_DIR_COLOUR: &[u8] = b"\x1b[38;2;30;144;255m";

/// Returns the colour code for a given file extension.
/// Falls back to `or_alternative` if the extension is not found in the color map.
#[inline]
pub fn colour_path_or_alternative<'a>(extension:&'a [u8],or_alternative:&'a [u8])->&'a [u8]{
    LS_COLOURS_HASHMAP.get(extension).map(|v| &**v).unwrap_or_else(||or_alternative)
}


/// Returns the colour code for a given file extension if it exists in the color map.
/// Returns `None` if not found.
#[inline]
pub fn colour_path(extension:&'static [u8])->Option<&'static[u8]>{
    LS_COLOURS_HASHMAP.get(extension).map(|v| &**v)
}


/// Like `colour_path_or_alternative`, but defaults to `NO_COLOR` if extension is not recognized.
/// This is useful for cases where you want to ensure a reset color code is used
/// when the file type is not recognized.
#[inline]
pub fn colour_path_or_reset<'a>(extension: &'a  [u8]) -> &'a [u8] {
    LS_COLOURS_HASHMAP.get(extension).map(|v| &**v).unwrap_or_else(|| NO_COLOUR)
}

/// Macro to generate a color code for a file type based on its name/type.
///
/// It uses the `colour_path_or_alternative` function to retrieve a color from
/// the internal LS_COLORS map. If the file type is not recognized, it falls
/// back to a sensible default such as `NO_COLOR`, `DEFAULT_SYMLINK_COLOR`, or `DEFAULT_DIR_COLOR`.
///
/// Please note, you cannot use raw string literals arguments with this macro.
/// 
/// # Usage
///
/// ```rust
/// use compile_time_ls_colours::{file_type_colour, NO_COLOUR};
///
/// // Get color for a symlink
/// let symlink_color: &[u8] = file_type_colour!(symlink);
///
/// // Get color for a directory
/// let dir_color: &[u8] = file_type_colour!(directory);
/// let ext_rs= b"rs";
/// // Get color for a known extension (e.g., b"rs")
/// let rs_color: &[u8] = file_type_colour!(ext_rs);
/// // Get fallback color if extension is not in the map
/// let ext = b"txt";
/// let unknown_color: &[u8] = file_type_colour!(ext); // defaults to NO_COLOUR
/// ```
/// 
/// 
#[macro_export]
macro_rules! file_type_colour {
    
    (symlink) => { // if it's a symlink, use the default symlink colour
        $crate::colour_path_or_alternative(b"symlink", $crate::DEFAULT_SYMLINK_COLOR)
    };
    (directory) => { // if it's a directory, use the default directory colour
        $crate::colour_path_or_alternative(b"directory", $crate::DEFAULT_DIR_COLOUR)
    };
    ($other:ident) => { // for any other file type, use the colour from the LS_COLORS map or NO_COLOUR
        $crate::colour_path_or_alternative($other, $crate::NO_COLOUR)
    };
}
