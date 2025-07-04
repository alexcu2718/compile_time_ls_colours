# compile_time_ls_colours

Temporary crate for file extension colour lookups using LS_COLORS format.

The main idea is a compile time hash map (we can't use std hashmap due to complicated reasons)

It provides colour coding for file types in terminal applications. Keys are byte slices representing file extensions. Values are byte slices representing ///ANSI escape sequences. Generated at build time from the LS_COLORS environment variable.

## Quick Example

```rust
use compile_time_ls_colours::colour_path_or_reset;

// Get color code for .rs files, fallback to reset code if not found
let extension:&[u8]=b"rs";
let color = colour_path_or_reset(extension)
println!("\x1b[{}m*.rs\x1b[0m", String::from_utf8_lossy(color));
```

``` rust
 

//This is a compile-time hash map of file extensions to their corresponding ANSI color codes based on the LS_COLORS environment variable.
pub static LS_COLOURS_HASHMAP: Map<&'static [u8], &'static [u8]>


pub fn colour_path_or_alternative<'a>(extension:&'a [u8],or_alternative:&'a [u8])->&'a [u8]{
    LS_COLOURS_HASHMAP.get(extension).map(|v| &**v).unwrap_or_else(||or_alternative)
}


/// Returns the colour code for a given file extension if it exists in the color map.
/// Returns `None` if not found.
pub fn colour_path(extension:&'static [u8])->Option<&'static[u8]>{
    LS_COLOURS_HASHMAP.get(extension).map(|v| &**v)
}


/// Like `colour_path_or_alternative`, but defaults to `NO_COLOR` if extension is not recognized.
/// This is useful for cases where you want to ensure a reset color code is used
/// when the file type is not recognized.
pub fn colour_path_or_reset<'a>(extension: &'a  [u8]) -> &'a [u8] {
    LS_COLOURS_HASHMAP.get(extension).map(|v| &**v).unwrap_or_else(|| NO_COLOUR)
}



#[macro_export]
//DO NOT USE RAW LITERALS AKA b"py" IN THIS!
macro_rules! file_type_colour {
    
    (symlink) => { // if it's a symlink, use the default symlink colour
        $crate::colour_path_or_alternative(b"symlink", $crate::DEFAULT_SYMLINK_COLOUR)
    };
    (directory) => { // if it's a directory, use the default directory colour
        $crate::colour_path_or_alternative(b"directory", $crate::DEFAULT_DIR_COLOUR)
    };
    ($other:ident) => { // for any other file type, use the colour from the LS_COLORS map or NO_COLOUR
        $crate::colour_path_or_alternative($other, $crate::NO_COLOUR)
    };
}





```
