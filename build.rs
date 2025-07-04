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

const COLOUR_SYMLINK_DEFAULT: &[u8] = ansi_bytes!(cyan  bold);
const COLOUR_DIRECTORY_DEFAULT: &[u8] = ansi_bytes!(blue bold);
const COLOUR_SOCKET_DEFAULT: &[u8] = ansi_bytes!(magenta bold);
const COLOUR_PIPE_DEFAULT: &[u8] = ansi_bytes!(yellow bold);
const COLOUR_BLOCK_DEVICE_DEFAULT: &[u8] = ansi_bytes!(red bold);
const COLOUR_CHARACTER_DEVICE_DEFAULT: &[u8] = ansi_bytes!(green bold);
const COLOUR_EXECUTABLE_DEFAULT: &[u8] = ansi_bytes!(green bold);

fn main() {
    let custom_colours = env::var("CUSTOM_LS_COLORS");
    let ls_colors = match custom_colours {
        Ok(use_this) => use_this,
        Err(_) => env::var("LS_COLORS").unwrap_or_default(),
    };
    let mut color_map = parse_ls_colours(&ls_colors);

    // Add fallback colors for common extensions
    add_new_colours(&mut color_map);
    add_defaults(&mut color_map);
    //we dont add the defaults d_types here, we handle them via a macro (becayse they're not defined as extensions!)
    // Generate the static PHF map

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("ls_colours.rs");
    let mut f = File::create(&dest_path).unwrap();

    writeln!(f, "use phf::phf_map;").unwrap();
    writeln!(f, "/// This is a compile-time hash map of file extensions to their corresponding ANSI color codes").unwrap();
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

    for (key, escape_seq) in color_map {
        // Convert the escape sequence to bytes and escape special characters
        let escaped_seq = lambda_escape_string(&escape_seq);
        // Write the key-value pair to the file
        writeln!(f, "    b\"{}\" => b\"{}\",", key, escaped_seq).unwrap();
    }

    writeln!(f, "}};").unwrap();
}

fn parse_ls_colours(ls_colours: &str) -> HashMap<String, Vec<u8>> {
    // Helper function to format ANSI escape sequences
    let format_ansi_sequence = |code: &str| -> Vec<u8> { format!("\x1b[{}m", code).into_bytes() };

    let insert_color = |map: &mut HashMap<String, Vec<u8>>, key: &str, value: &str| {
        map.insert(key.to_string(), format_ansi_sequence(value));
    };

    let mut color_map = HashMap::new();

    for entry in ls_colours.split(':') {
        // Skip empty entries
        if entry.is_empty() {
            continue;
        }

        // Split into key=value parts
        let parts: Vec<&str> = entry.splitn(2, '=').collect();
        if parts.len() != 2 {
            continue;
        }

        let (key, value) = (parts[0], parts[1]);

        // Handle directory entry
        if key == "di" {
            insert_color(&mut color_map, "directory", value);
            continue;
        }

        // Handle symlink entry
        if key == "ln" {
            insert_color(&mut color_map, "symlnk", value);
            continue;
        }
        if key == "so" {
            insert_color(&mut color_map, "socket", value);
            continue;
        }
        if key == "pi" {
            insert_color(&mut color_map, "pipe", value);
            continue;
        }
        if key == "bd" {
            insert_color(&mut color_map, "block_device", value);
            continue;
        }
        if key == "cd" {
            insert_color(&mut color_map, "character_device", value);
            continue;
        }
        if key == "ex" {
            insert_color(&mut color_map, "executable", value);
            continue;
        }

        // Handle file extensions (e.g., *.rs)
        if key.starts_with("*.") {
            let extension = key[2..].to_string();
            insert_color(&mut color_map, &extension, value);
        }
    }

    color_map
}

// Adds new colours for common file extensions if they are not already in the map.
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

//a helper function to insert multiple extensions with the same color
//this is used to avoid code duplication in the add_defaults function.
//it takes a mutable reference to the map, a slice of extensions, and a color byte
fn insert_extensions(map: &mut HashMap<String, Vec<u8>>, extensions: &[&str], color: &[u8]) {
    for ext in extensions {
        map.entry(ext.to_string()).or_insert_with(|| color.to_vec());
    }
}

//we don't need to worry about dtypes, that's handled by my macro.
//copied from my ls environment variable.
fn add_defaults(map: &mut HashMap<String, Vec<u8>>) {
    let red = ansi_bytes!(rgb(255, 80, 80)).to_vec(); // compressed
    let magenta = ansi_bytes!(rgb(200, 100, 200)).to_vec(); // images/videos
    let cyan = ansi_bytes!(rgb(0, 200, 200)).to_vec(); // audio
    let gray = ansi_bytes!(rgb(128, 128, 128)).to_vec(); // backups
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
        if !map.contains_key(key) {
            // Only insert if the key does not already exist
            // This prevents overwriting existing colours
            // for the same key.
            map.entry(key.to_string())
                .or_insert_with(|| colour.to_vec());
        }
    }
}
