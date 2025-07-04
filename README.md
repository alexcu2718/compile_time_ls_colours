# compile_time_ls_colours

Temporary crate for file extension colour lookups using LS_COLORS format.

The main idea is a compile time hash map (we can't use std hashmap due to complicated reasons)

It provides colour coding for file types in terminal applications. Keys are byte slices representing file extensions.

Values are byte slices representing ANSI escape sequences which are generated at build time from the LS_COLORS environment variable.

## Caveats

This doesn't make any system calls, so we cannot get any information about being an executable,
however this flag is provided, eg in the example below!

The special values for custom colouring are

```bash
    symlink,
    directory,
    executable,
    socket,
    pipe,
    block_device,
    character_device,
#Everything else will default to reset/no colour :)
```

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
 let unknown_colour: &'static [u8] = file_type_colour!(ext); // defaults to NO_COLOUR if not a keyword (directory/symlink)
 
 let directory_colour: &'static [u8] = file_type_colour!(directory);
 let symlink_colour: &'static [u8] = file_type_colour!(symlink);
let executable_colour: &'static [u8] = file_type_colour!(executable);


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
pub fn colour_path_or_reset<'a>(extension: &'a  [u8]) -> &'static [u8] {
    LS_COLOURS_HASHMAP.get(extension).map(|v| &**v).unwrap_or_else(|| NO_COLOUR)
}

/// #########################################################################################################################
/// #                                                                                                                       #
/// #                    R E A D   T H I S  BEFORE YOU GET PISSED ABOUT UNSAFE                                              #     
/// #                                                                                                                       #
/// #########################################################################################################################
/// #                                                                                                                       #
/// #   We have to cheat around limitations here.                                                                           #
/// #   The macro is NOT unsafe, because we know at compile time that certain entries like                                  #
/// #   "symlink" and "directory" will always exist in the colour table. because we auto generate them!                     #
/// #    The only unsafe access pattern is on SPECIFIED keywords,which are GUARANTEED (by my defaults)                      #
/// #    Ultimately I feel that this is `explicitly` the reason to USE unwrap_unchecked ffs(guarantees)                     #
/// #                                                                                                                       #    
/// #                                                                                                                       #    
/// #                                                                                                                       #
/// #########################################################################################################################
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
        $crate::colour_path_or_alternative($other, $crate::NO_COLOUR) //only one that's not guaranteed to exist.
        //so this can fail.
    };
}


```
