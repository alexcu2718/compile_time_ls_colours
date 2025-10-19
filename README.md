# compile_time_ls_colours

Quick screenshot of how outputs look (rendered from my LS_COLORS)  ((TRUNCATED FOR BREVITY))

![LS_COLORS HashMap Test Output](./colour_output.png)

Temporary crate for file extension colour lookups using LS_COLORS format.

The main idea is a compile time/runtime initialised map (compile time available under --features phf)

It provides colour coding for file types in terminal applications. Keys are byte slices representing file extensions.

Values are byte slices representing ANSI escape sequences which are generated at build time from the LS_COLORS environment variable.

You can see an example of the actual generated code, in the root directory of this github at 'ls_colours.rs' file.
This should make it trivial to validate :)

The crown jewel is a macro, defined at the bottom, it is quite esoteric though!

You can set your own colours with the env var `CUSTOM_LS_COLORS`, generating a 'new' config is not in this crate yet.
I will probably add a simple way to colour your terminal, this seems the most elegant way so far.
(That's what I could do with the main function (aka the CLI/commandline of this library))
(GOING TO DO THAT!)

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

```

Everything else will try to find it's associated colour in LS, if not, returns a reset ANSI code
 (so painting bytes with it would make it look normal, ideal!)

You can see the default colours I implement(imports from this crate), you can set your own colours by changing your LS_COLOR environment variable
However, this is basically entirely DIY right now, I'd like to make it trivial to change stuff. (ALSO, THIS NEEDS A REBUILD ON CHANGE!)

## General usage

 ```rust
use compile_time_ls_colours::{
    file_type_colour,
    LS_COLOURS_HASHMAP,
    NO_COLOUR,
    colour_path_or_reset,
};

// Using the compile-time PHF hash map
let compile_time_hashmap_initial: &'static [u8] =
    LS_COLOURS_HASHMAP.get(b"py".as_ref()).map(|v| &**v).unwrap_or_else(|| NO_COLOUR);



// Get colour for a symlink
let symlink_colour: &'static [u8] = file_type_colour!(symlink); 

// Get colour for a directory
let dir_colour: &'static [u8] = file_type_colour!(directory);

// Get fallback colour if extension is not in the map
let unknown_colour: &'static [u8] = colour_path_or_reset(b"ext"); 
let probably_a_colour_maybe: &'static [u8] = colour_path_or_reset(b"sh");

let i_love_this_language: &'static [u8] = b"js";

let colour_of_love: &'static [u8] =
    LS_COLOURS_HASHMAP
        .get(i_love_this_language)
        .map(|v| &**v)
        .unwrap_or_else(|| NO_COLOUR);

let i_should_learn_this_language: &'static [u8] = b"cpp";

let colour_of_grey_hair: &'static [u8] =
    LS_COLOURS_HASHMAP
        .get(i_should_learn_this_language)
        .map(|v| &**v)
        .unwrap_or_else(|| NO_COLOUR);

let directory_colour: &'static [u8] = file_type_colour!(directory);
let symlink_colour: &'static [u8] = file_type_colour!(symlink);
```

///
