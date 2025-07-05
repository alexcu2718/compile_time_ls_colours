#![allow(clippy::all)]
#![allow(warnings)]

use ansic::ansi;
use phf_codegen::Map;
use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::thread;

use std::io::BufWriter;
macro_rules! ansi_bytes {
    ($($t:tt)*) => {
        ansi!($($t)*).as_bytes()
    };
}

const COLOUR_RS: &[u8] = ansi_bytes!(rgb(130, 200, 0));
const COLOUR_PY: &[u8] = ansi_bytes!(rgb(0, 200, 200));
const COLOUR_CPP: &[u8] = ansi_bytes!(rgb(0, 100, 200));
const COLOUR_H: &[u8] = ansi_bytes!(rgb(80, 160, 220));
const COLOUR_C: &[u8] = ansi_bytes!(rgb(150, 131, 38));
const COLOUR_LUA: &[u8] = ansi_bytes!(rgb(0, 0, 255));
const COLOUR_HTML: &[u8] = ansi_bytes!(rgb(255, 105, 180));
const COLOUR_CSS: &[u8] = ansi_bytes!(rgb(150, 200, 50));
const COLOUR_JS: &[u8] = ansi_bytes!(rgb(240, 220, 80));
const COLOUR_JSON: &[u8] = ansi_bytes!(rgb(160, 140, 200));
const COLOUR_TOML: &[u8] = ansi_bytes!(rgb(200, 120, 80));
const COLOUR_TXT: &[u8] = ansi_bytes!(rgb(128, 128, 128));
const COLOUR_MD: &[u8] = ansi_bytes!(rgb(100, 180, 100));
const COLOUR_INI: &[u8] = ansi_bytes!(rgb(180, 80, 80));
const COLOUR_CFG: &[u8] = ansi_bytes!(rgb(180, 80, 80));
const COLOUR_XML: &[u8] = ansi_bytes!(rgb(130, 90, 200));
const COLOUR_YML: &[u8] = ansi_bytes!(rgb(130, 90, 200));
const COLOUR_TS: &[u8] = ansi_bytes!(rgb(90, 150, 250));
const COLOUR_SH: &[u8] = ansi_bytes!(rgb(100, 250, 100));
const COLOUR_BAT: &[u8] = ansi_bytes!(rgb(200, 200, 0));
const COLOUR_RB: &[u8] = ansi_bytes!(rgb(200, 0, 200));
const COLOUR_PHP: &[u8] = ansi_bytes!(rgb(80, 80, 200));
const COLOUR_PL: &[u8] = ansi_bytes!(rgb(80, 80, 200));
const COLOUR_R: &[u8] = ansi_bytes!(rgb(0, 180, 0));
const COLOUR_CS: &[u8] = ansi_bytes!(rgb(50, 50, 50));
const COLOUR_JAVA: &[u8] = ansi_bytes!(rgb(150, 50, 50));
const COLOUR_GO: &[u8] = ansi_bytes!(rgb(0, 150, 150));
const COLOUR_SWIFT: &[u8] = ansi_bytes!(rgb(250, 50, 150));
const COLOUR_KT: &[u8] = ansi_bytes!(rgb(50, 150, 250));
const COLOUR_SCSS: &[u8] = ansi_bytes!(rgb(245, 166, 35));
const COLOUR_LESS: &[u8] = ansi_bytes!(rgb(245, 166, 35));
const COLOUR_CSV: &[u8] = ansi_bytes!(rgb(160, 160, 160));
const COLOUR_TSV: &[u8] = ansi_bytes!(rgb(160, 160, 160));
const COLOUR_XLS: &[u8] = ansi_bytes!(rgb(64, 128, 64));
const COLOUR_XLSX: &[u8] = ansi_bytes!(rgb(64, 128, 64));
const COLOUR_SQL: &[u8] = ansi_bytes!(rgb(100, 100, 100));

const COLOUR_SYMLINK_DEFAULT: &[u8] = ansi_bytes!(cyan  bold); // ln
const COLOUR_DIRECTORY_DEFAULT: &[u8] = ansi_bytes!(blue bold); //di
const COLOUR_SOCKET_DEFAULT: &[u8] = ansi_bytes!(magenta bold); //so
const COLOUR_PIPE_DEFAULT: &[u8] = ansi_bytes!(yellow bold); //pi
const COLOUR_BLOCK_DEVICE_DEFAULT: &[u8] = ansi_bytes!(red bold); //bd
const COLOUR_CHARACTER_DEVICE_DEFAULT: &[u8] = ansi_bytes!(green bold); //  cd
const COLOUR_EXECUTABLE_DEFAULT: &[u8] = ansi_bytes!(green bold); //    ex
const COLOUR_STICKY_DEFAULT: &[u8] = ansi_bytes!(white   blue);   // st
const COLOUR_OTHER_WRITABLE_DEFAULT: &[u8]      = ansi_bytes!(blue    green); // ow
const COLOUR_ORPHAN_SYMLINK_DEFAULT: &[u8]  = ansi_bytes!(red bold); // or
const COLOUR_SETUID_DEFAULT: &[u8]              = ansi_bytes!(white   red);      // su 
const COLOUR_SETGID_DEFAULT: &[u8]   = ansi_bytes!(white   magenta); // sg 


fn main() {
    let custom_colours = env::var("CUSTOM_LS_COLORS");
    let ls_colours = match custom_colours {
        Ok(use_this) => use_this,
        Err(_) => env::var("LS_COLORS").unwrap_or_default(),
    };
    let mut colour_map = parse_ls_colours(&ls_colours);

    // Add fallback colours for common extensions
    add_new_colours(&mut colour_map);
    add_defaults(&mut colour_map);

    //this is a cargo environment variable that points to the output directory
    //where the generated file will be placed.
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("ls_colours.rs");
    let mut f = File::create(&dest_path).unwrap();

    writeln!(f, "use phf::phf_map;").unwrap();
    writeln!(f, "/// This is a compile-time hash map of file extensions to their corresponding ANSI colour codes").unwrap();
    writeln!(f, "/// based on the `LS_COLORS` environment variable.").unwrap();
    writeln!(f, "///").unwrap();
    writeln!(
        f,
        "/// It provides colour coding for file types in terminal applications."
    )
    .unwrap();
    writeln!(f, "/// Keys are byte slices representing file extensions.").unwrap();
    writeln!(
        f,
        "/// Values are byte slices representing ANSI escape sequences."
    )
    .unwrap();
    writeln!(
        f,
        "/// Generated at build time from the LS_COLORS environment variable."
    )
    .unwrap();
    writeln!(
        f,
        "pub static LS_COLOURS_HASHMAP: phf::Map<&'static [u8], &'static [u8]> = phf_map! {{"
    )
    .unwrap();

    // Define a closure to escape special characters in the ANSI escape sequences
    //also to simplif the code, since performance doesnt matter here.
    let lambda_escape_string = |s: &Vec<u8>| {
        String::from_utf8_lossy(s)
            .replace('\\', "\\\\")
            .replace('\"', "\\\"")
    };



    for (key, escape_seq) in colour_map {
        // Convert the escape sequence to bytes and escape special characters
        let escaped_seq = lambda_escape_string(&escape_seq);
        // Write the key-value pair to the file
        writeln!(f, "    b\"{}\" => b\"{}\",", key, escaped_seq).unwrap();
    }

    writeln!(f, "}};").unwrap();
}







fn parse_ls_colours(ls_colours: &str) -> HashMap<String, Vec<u8>> {
    let format_ansi_sequence = |code: &str| -> Vec<u8> {
        format!("\x1b[{}m", code).into_bytes()
    };

    let insert_colour = |map: &mut HashMap<String, Vec<u8>>, key: &str, value: &str| {
        map.insert(key.to_string(), format_ansi_sequence(value));
    };

    let mut colour_map = HashMap::new();

    for entry in ls_colours.split(':') {
        if entry.is_empty() {
            continue;
        }

        let parts: Vec<&str> = entry.splitn(2, '=').collect();
        if parts.len() != 2 {
            continue;
        }

        let (key, value) = (parts[0], parts[1]);

        match key {
            "di" => insert_colour(&mut colour_map, "directory", value), //directory
            "ln" => insert_colour(&mut colour_map, "symlink", value),
            "so" => insert_colour(&mut colour_map, "socket", value),
            "pi" => insert_colour(&mut colour_map, "pipe", value),
            "bd" => insert_colour(&mut colour_map, "block_device", value),
            "cd" => insert_colour(&mut colour_map, "character_device", value),
            "ex" => insert_colour(&mut colour_map, "executable", value),
            "st" => insert_colour(&mut colour_map, "sticky", value), //st
            "ow" => insert_colour(&mut colour_map, "other_writable", value),
            "or" => insert_colour(&mut colour_map, "orphan_symlink", value),
            "su" => insert_colour(&mut colour_map, "setuid", value),
            "sg" => insert_colour(&mut colour_map, "setgid", value),
            "tw" => insert_colour(&mut colour_map, "other_writable", value),
            _ if key.starts_with("*.") => {
                let extension = &key[2..];
                insert_colour(&mut colour_map, extension, value);
            }
            _ => {} // Ignore invalids
        }
    }

    colour_map
}


fn add_new_colours(colour_map: &mut HashMap<String, Vec<u8>>) {
    // Only add fallback if the extension isn't already in the map
    let fallbacks = vec![
        ("rs", COLOUR_RS),
        ("py", COLOUR_PY),
        ("cpp", COLOUR_CPP),
        ("h", COLOUR_H),
        ("c", COLOUR_C),
        ("lua", COLOUR_LUA),
        ("html", COLOUR_HTML),
        ("css", COLOUR_CSS),
        ("js", COLOUR_JS),
        ("json", COLOUR_JSON),
        ("toml", COLOUR_TOML),
        ("txt", COLOUR_TXT),
        ("md", COLOUR_MD),
        ("ini", COLOUR_INI),
        ("cfg", COLOUR_CFG),
        ("xml", COLOUR_XML),
        ("yml", COLOUR_YML),
        ("ts", COLOUR_TS),
        ("sh", COLOUR_SH),
        ("bat", COLOUR_BAT),
        ("rb", COLOUR_RB),
        ("php", COLOUR_PHP),
        ("pl", COLOUR_PL),
        ("r", COLOUR_R),
        ("cs", COLOUR_CS),
        ("java", COLOUR_JAVA),
        ("go", COLOUR_GO),
        ("swift", COLOUR_SWIFT),
        ("kt", COLOUR_KT),
        ("scss", COLOUR_SCSS),
        ("less", COLOUR_LESS),
        ("csv", COLOUR_CSV),
        ("tsv", COLOUR_TSV),
        ("xls", COLOUR_XLS),
        ("xlsx", COLOUR_XLSX),
        ("sql", COLOUR_SQL),
    ];

    for (ext, colour) in fallbacks {
        colour_map
            .entry(ext.to_string())
            .or_insert_with(|| colour.to_vec());
    }
}

// This function adds default colours for common file extensions and special file types
//copied from my ls environment variable.
fn add_defaults(map: &mut HashMap<String, Vec<u8>>) {
    let red = ansi_bytes!(rgb(255, 80, 80)).to_vec(); // compressed
    let magenta = ansi_bytes!(rgb(200, 100, 200)).to_vec(); // images/videos
    let cyan = ansi_bytes!(rgb(0, 200, 200)).to_vec(); // audio
    let gray = ansi_bytes!(rgb(128, 128, 128)).to_vec(); // backups
    // Define a closure to insert extensions with their corresponding colours
    let insert_extensions = |map: &mut HashMap<String, Vec<u8>>, extensions: &[&str], colour: &[u8]| {
    for ext in extensions {
        map.entry(ext.to_string()).or_insert_with(|| colour.to_vec());
    }
    };

    let compressed = [
        "7z", "ace", "alz", "apk", "arc", "arj", "bz", "bz2", "cab", "cpio", "crate", "deb",
        "drpm", "dwm", "dz", "ear", "egg", "esd", "gz", "jar", "lha", "lrz", "lz", "lz4", "lzh",
        "lzma", "lzo", "pyz", "rar", "rpm", "rz", "sar", "swm", "t7z", "tar", "taz", "tbz", "tbz2",
        "tgz", "tlz", "txz", "tz", "tzo", "tzst", "udeb", "war", "whl", "wim", "xz", "z", "zip",
        "zoo", "zst",
    ];
    let media = [
        "avif", "jpg", "jpeg", "jxl", "mjpg", "mjpeg", "gif", "bmp", "pbm", "pgm", "ppm", "tga",
        "xbm", "xpm", "tif", "tiff", "png", "svg", "svgz", "mng", "pcx", "mov", "mpg", "mpeg",
        "m2v", "mkv", "webm", "webp", "ogm", "mp4", "m4v", "mp4v", "vob", "qt", "nuv", "wmv",
        "asf", "rm", "rmvb", "flc", "avi", "fli", "flv", "gl", "dl", "xcf", "xwd", "yuv", "cgm",
        "emf", "ogv", "ogx",
    ];

    let audio = [
        "aac", "au", "flac", "m4a", "mid", "midi", "mka", "mp3", "mpc", "ogg", "ra", "wav", "oga",
        "opus", "spx", "xspf",
    ];

    let backups = [
        "bak",
        "crdownload",
        "dpkg-dist",
        "dpkg-new",
        "dpkg-old",
        "dpkg-tmp",
        "old",
        "orig",
        "part",
        "rej",
        "rpmnew",
        "rpmorig",
        "rpmsave",
        "swp",
        "tmp",
        "ucf-dist",
        "ucf-new",
        "ucf-old",
    ];

    insert_extensions(map, &compressed, &red);

    insert_extensions(map, &media, &magenta);

    insert_extensions(map, &audio, &cyan);

    insert_extensions(map, &backups, &gray);
    let specials = [
        // Special file types with default colours
        ("symlink", COLOUR_SYMLINK_DEFAULT),
        ("directory", COLOUR_DIRECTORY_DEFAULT),
        ("socket", COLOUR_SOCKET_DEFAULT),
        ("pipe", COLOUR_PIPE_DEFAULT),
        ("block_device", COLOUR_BLOCK_DEVICE_DEFAULT),
        ("character_device", COLOUR_CHARACTER_DEVICE_DEFAULT),
        ("executable", COLOUR_EXECUTABLE_DEFAULT),
    ];

    for (key, colour) in specials {
      
            map.entry(key.to_string())
                .or_insert_with(|| colour.to_vec());
            //insert with checks if the key already exists
            //this is to avoid overwriting existing values.
        }
    }

