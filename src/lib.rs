// Include the generated LS_COLORS_HASHMAP from the build script output directory.
include!(concat!(env!("OUT_DIR"), "/ls_colours.rs"));
#[cfg(test)]
mod test;


pub const NO_COLOUR: &[u8] = b"\x1b[0m"; // Reset colour code


/// Returns the colour code for a given file extension.
/// Falls back to `or_alternative` if the extension is not found in the colour map.
#[inline]
pub fn colour_path_or_alternative<'a>(extension:&'a [u8],or_alternative:&'a [u8])->&'a [u8]{
   
    LS_COLOURS_HASHMAP.get(extension).map(|v| &**v).unwrap_or_else(||or_alternative)
}


/// Returns the colour code for a given file extension if it exists in the colour map.
/// Returns `None` if not found.
#[inline]
pub fn colour_path<'a>(extension:&'a [u8])->Option<&'static[u8]>{
    LS_COLOURS_HASHMAP.get(extension).map(|v| &**v)
}


/// Like `colour_path_or_alternative`, but defaults to `NO_COLOR` if extension is not recognized.
/// This is useful for cases where you want to ensure a reset colour code is used
/// when the file type is not recognized.
#[inline]
pub fn colour_path_or_reset<'a>(extension: &'a  [u8]) -> &'static [u8] {
    LS_COLOURS_HASHMAP.get(extension).map(|v| &**v).unwrap_or_else(|| NO_COLOUR)
}


///you shouldnt use this, unfortunately there's limitations because i dont want to make the macro unsafe,
/// because it would panic at compile time if the extension is not found.
/// DO NOT USE THIS EVER THIS IS INTERNAL FOR MACRO IMPLEMENTATION
#[inline]
pub  fn colour_path_unchecked<'a>(extension: &'a [u8]) -> &'static [u8] {
    // This function is unsafe because it assumes the extension exists in the LS_COLORS_HASHMAP.
    // Use with caution, as it will panic if the extension is not found.
    unsafe { LS_COLOURS_HASHMAP.get(extension).unwrap_unchecked() }
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
/// let ext = b"txt";
/// let unknown_colour: &'static [u8] = file_type_colour!(ext); // defaults to NO_COLOUR
/// 
/// 
/// let directory_colour: &'static [u8] = file_type_colour!(directory);
/// let symlink_colour: &'static [u8] = file_type_colour!(symlink);
/// ```
/// 
/// 
#[macro_export]
macro_rules! file_type_colour {
    
    
    (symlink) => { // if it's a symlink, use the default symlink colour
        $crate::colour_path_unchecked(b"symlink") //we know it's safe because we have a default colour for symlinks
    };
    (directory) => { // if it's a directory, use the default directory colour
        $crate::colour_path_unchecked(b"directory")  //we know it's safe because we have a default colour for directories
    };
    (executable) => { // for executables, use the colour from the LS_COLORS map 
       $crate::colour_path_unchecked(b"executable") //same as above
    };
    (socket) => { // for sockets, use the colour from the LS_COLORS map 
       $crate::colour_path_unchecked(b"socket") //etc
    };
    (pipe) => { // for pipes, use the colour from the LS_COLORS map 
          $crate::colour_path_unchecked(b"pipe") //et
    };
    (block_device) => { // for block devices, use the colour from the LS_COLORS map 
           $crate::colour_path_unchecked(b"block_device") //etc
    };
    (character_device) => { // for character devices, use the colour from the LS_COLORS map or NO_COLOUR
        $crate::colour_path_unchecked(b"character_device")
    };

    ($other:ident) => { // for any other file type, use the colour from the LS_COLORS map or NO_COLOUR
        $crate::colour_path_or_alternative($other, $crate::NO_COLOUR)
    };
}

