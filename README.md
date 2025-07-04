# compile_time_ls_colours

Temporary crate for file extension colour lookups using LS_COLORS format.

The main idea is a compile time hash map (we can't use std hashmap due to complicated reasons)

It provides colour coding for file types in terminal applications. Keys are byte slices representing file extensions. Values are byte slices representing ///ANSI escape sequences. Generated at build time from the LS_COLORS environment variable.

## Quick Example

 ```rust
 use compile_time_ls_colours::{file_type_colour};
///
/// Get colour for a symlink
let symlink_colour: &[u8] = file_type_colour!(symlink);

// Get colour for a directory
 let dir_colour: &'static [u8] = file_type_colour!(directory);
 let ext_rs= b"rs";
 // Get colour for a known extension (e.g., b"rs")
 let rs_colour: &'static [u8] = file_type_colour!(ext_rs);
  Get fallback colour if extension is not in the map
 let ext = b"txt";
 let unknown_colour: &'static [u8] = file_type_colour!(ext); // defaults to NO_COLOUR
 
 let directory_colour: &'static [u8] = file_type_colour!(directory);
 let symlink_colour: &'static [u8] = file_type_colour!(symlink);


```

```rust
//function definitions below


//This is a compile-time hash map of file extensions to their corresponding ANSI color codes based on the LS_COLORS environment variable.
pub static LS_COLOURS_HASHMAP: Map<&'static [u8], &'static [u8]>


// Provide the colour byte pattern for the extension and provide an alternative if it doesn't exist.
pub fn colour_path_or_alternative<'a>(extension:&'a [u8],or_alternative:&'a [u8])->&'a [u8]{
    LS_COLOURS_HASHMAP.get(extension).map(|v| &**v).unwrap_or_else(||or_alternative)
}


/// Returns the colour code for a given file extension if it exists in the color map.
/// Returns `None` if not found.
pub fn colour_path(extension:&'static [u8])->Option<&'static[u8]>{
    LS_COLOURS_HASHMAP.get(extension).map(|v| &**v)
}


/// Like `colour_path_or_alternative`, but defaults to `NO_COLOUR` if extension is not recognized.
/// This is useful for cases where you want to ensure a reset colour code is used
/// when the file type is not recognized.
pub fn colour_path_or_reset<'a>(extension: &'a  [u8]) -> &'a [u8] {
    LS_COLOURS_HASHMAP.get(extension).map(|v| &**v).unwrap_or_else(|| NO_COLOUR)
}

///i could probably look into avoiding collisions in the future, not really a performance issue i'd assume (printing in general is intense IO)
#[macro_export]
///we have to use special tricks for d_types, as they're not extensions, so we macro it!
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




```
