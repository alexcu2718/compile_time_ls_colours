// Include the generated LS_COLORS_HASHMAP from the build script output directory.
include!(concat!(env!("OUT_DIR"), "/ls_colours.rs"));
#[cfg(test)]
mod test;


pub const NO_COLOUR: &[u8] = b"\x1b[0m"; // Reset colour code
// Default colour code for symbolic links.
pub const DEFAULT_SYMLINK_COLOUR: &[u8] = b"\x1b[38;5;153m";

// Default colour code for directories.
pub const DEFAULT_DIR_COLOUR: &[u8] = b"\x1b[38;2;30;144;255m";


// Default colours if LS_COLORS is not set
pub const DEFAULT_SOCKET_COLOUR: &[u8] = b"\x1b[01;35m";      // "so" => socket
pub const DEFAULT_PIPE_COLOUR: &[u8] = b"\x1b[40;33m";        // "pi" => pipe
pub const DEFAULT_EXECUTABLE_COLOUR: &[u8] = b"\x1b[01;32m";  // "ex" => executable
pub const DEFAULT_BLOCK_DEVICE_COLOUR: &[u8] = b"\x1b[40;33;01m"; // "bd" => block device
pub const DEFAULT_CHAR_DEVICE_COLOUR: &[u8] = b"\x1b[40;33;01m";  // "cd" => char device


/// Returns the colour code for a given file extension.
/// Falls back to `or_alternative` if the extension is not found in the colour map.
#[inline]
pub fn colour_path_or_alternative<'a>(extension:&'a [u8],or_alternative:&'a [u8])->&'a [u8]{
   
    LS_COLOURS_HASHMAP.get(extension).map(|v| &**v).unwrap_or_else(||or_alternative)
}


/// Returns the colour code for a given file extension if it exists in the colour map.
/// Returns `None` if not found.
#[inline]
pub fn colour_path(extension:&'static [u8])->Option<&'static[u8]>{
    LS_COLOURS_HASHMAP.get(extension).map(|v| &**v)
}


/// Like `colour_path_or_alternative`, but defaults to `NO_COLOR` if extension is not recognized.
/// This is useful for cases where you want to ensure a reset colour code is used
/// when the file type is not recognized.
#[inline]
pub fn colour_path_or_reset<'a>(extension: &'a  [u8]) -> &'a [u8] {
    LS_COLOURS_HASHMAP.get(extension).map(|v| &**v).unwrap_or_else(|| NO_COLOUR)
}


/// Macro to generate a colour code for a file type based on its name/type.
///
/// It uses the `colour_path_or_alternative` function to retrieve a colour from
/// the internal LS_COLORS map. If the file type is not recognized, it falls
/// back to a sensible default such as `NO_COLOR`, `DEFAULT_SYMLINK_COLOUR`, or `DEFAULT_DIR_COLOUR`.
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
/// let ext = b"txt";
/// let unknown_colour: &'static [u8] = file_type_colour!(ext); // defaults to NO_COLOUR
/// 
/// let directory_colour: &'static [u8] = file_type_colour!(directory);
/// let symlink_colour: &'static [u8] = file_type_colour!(symlink);
/// ```
/// 
/// 
#[macro_export]
macro_rules! file_type_colour {
    
    (symlink) => { // if it's a symlink, use the default symlink colour
        $crate::colour_path_or_alternative(b"symlink", $crate::DEFAULT_SYMLINK_COLOUR)
    };
    (directory) => { // if it's a directory, use the default directory colour
        $crate::colour_path_or_alternative(b"directory", $crate::DEFAULT_DIR_COLOUR)
    };
    (executable) => { // for executables, use the colour from the LS_COLORS map or NO_COLOUR
        $crate::colour_path_or_alternative(b"executable", $crate::DEFAULT_EXECUTABLE_COLOUR)
    };
    (socket) => { // for sockets, use the colour from the LS_COLORS map or NO_COLOUR
        $crate::colour_path_or_alternative(b"socket", $crate::DEFAULT_SOCKET_COLOUR)
    };
    (pipe) => { // for pipes, use the colour from the LS_COLORS map or NO_COLOUR
        $crate::colour_path_or_alternative(b"pipe", $crate::DEFAULT_PIPE_COLOUR)
    };
    (block_device) => { // for block devices, use the colour from the LS_COLORS map or NO_COLOUR
        $crate::colour_path_or_alternative(b"block_device", $crate::DEFAULT_BLOCK_DEVICE_COLOUR)
    };
    (character_device) => { // for character devices, use the colour from the LS_COLORS map or NO_COLOUR
        $crate::colour_path_or_alternative(b"character_device", $crate::DEFAULT_CHAR_DEVICE_COLOUR)
    };

    ($other:ident) => { // for any other file type, use the colour from the LS_COLORS map or NO_COLOUR
        $crate::colour_path_or_alternative($other, $crate::NO_COLOUR)
    };
}

