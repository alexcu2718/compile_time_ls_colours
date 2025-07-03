# compile_time_ls_colours

i will update docs soon, just check tests

Temporary crate for file extension color lookups using LS_COLORS format.

The main idea is a compile time hash map (we can't use std)

It provides colour coding for file types in terminal applications. Keys are byte slices representing file extensions. Values are byte slices representing ///ANSI escape sequences. Generated at build time from the LS_COLORS environment variable.

## Quick Example

```rust
use compile_time_ls_colours::

// Get color code for .rs files, fallback to reset code if not found
let color = colour_hashmap(b"rs", b"\x1b[0m");
println!("\x1b[{}m*.rs\x1b[0m", String::from_utf8_lossy(color));
```

``` rust
 

//This is a compile-time hash map of file extensions to their corresponding ANSI color codes based on the LS_COLORS environment variable.
pub static LS_COLOURS_HASHMAP: Map<&'static [u8], &'static [u8]>


/// I've added some convenience functions to use below :)

////// Returns the color code for a given file extension.
/// If the extension is not found in the LS_COLORS_MAP, it returns the provided alternative
pub fn colour_hashmap<'a>(ext:&'a[u8],or_alternative:&'a[u8])->&'a[u8]{
    LS_COLOURS_HASHMAP.get(ext).map(|v| &**v).unwrap_or_else(||or_alternative)
}


// Returns the color code for if it exists
pub fn colour_map<'a>(ext:&'a[u8])->Option<&'a[u8]>{
    LS_COLOURS_HASHMAP.get(ext).map(|v| &**v)
}





```
