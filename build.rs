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
const NO_COLOUR: &[u8] = ansi_bytes!(reset);


///A  trait on a file, basically allowing a lot less boiler plate in the main function (so the logic is more obvious)
pub trait BuildWriter{

    fn write_constant_bytes(&mut self,name:&str,colour_bytes:&'static [u8]);

    fn write_comment(&mut self,paragraph_of_stuff:&str);

    fn write_code(&mut self,be_code_please:&str);


    fn write_code_comment(&mut self,paragraph_of_stuff:&str,reference:&str);

    fn write_escape_colour_code(&mut self,key:&str,bytes:&Vec<u8>);
  

}


fn escape_bytes(byt:&Vec<u8>)->String{
    String::from_utf8_lossy(byt)
            .replace('\\', "\\\\")
            .replace('\"', "\\\"")

}

impl BuildWriter for File{


    fn write_constant_bytes(&mut self,name:&str,colour_bytes:&[u8]){
        self.write_code_comment("Generated code for",name);
         writeln!(self,"pub const {}: &[u8] = &{:?} ;\n",name,colour_bytes).unwrap();
    }

    fn write_code_comment(&mut self,paragraph_of_stuff:&str,reference:&str){
        writeln!(self,"///{paragraph_of_stuff} for {}",reference).unwrap()
    }

    fn write_comment(&mut self,paragraph_of_stuff:&str) {
        writeln!(self,"///{paragraph_of_stuff}").unwrap()
    }


    fn write_code(&mut self,be_code_please:&str) {

     
        
        writeln!(self,"{be_code_please}").unwrap()
    }






    fn write_escape_colour_code(&mut self,key:&str,bytes:&Vec<u8>){

    // Define a closure to escape special characters in the ANSI escape sequences
    //also to simplify the code, since performance doesnt matter here.


    let escaped_seq=escape_bytes(bytes);
  
        writeln!(self, "    b\"{}\" => b\"{}\",", key, escaped_seq).unwrap();



    }
}


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
    f.write_comment("Predefined colour constants");
     // Generate all colour constants using the trait method
    f.write_constant_bytes("NO_COLOUR",NO_COLOUR);
    f.write_constant_bytes("COLOUR_RS", COLOUR_RS);
    f.write_constant_bytes("COLOUR_PY", COLOUR_PY);
    f.write_constant_bytes("COLOUR_CPP", COLOUR_CPP);
    f.write_constant_bytes("COLOUR_H", COLOUR_H);
    f.write_constant_bytes("COLOUR_C", COLOUR_C);
    f.write_constant_bytes("COLOUR_LUA", COLOUR_LUA);
    f.write_constant_bytes("COLOUR_HTML", COLOUR_HTML);
    f.write_constant_bytes("COLOUR_CSS", COLOUR_CSS);
    f.write_constant_bytes("COLOUR_JS", COLOUR_JS);
    f.write_constant_bytes("COLOUR_JSON", COLOUR_JSON);
    f.write_constant_bytes("COLOUR_TOML", COLOUR_TOML);
    f.write_constant_bytes("COLOUR_TXT", COLOUR_TXT);
    f.write_constant_bytes("COLOUR_MD", COLOUR_MD);
    f.write_constant_bytes("COLOUR_INI", COLOUR_INI);
    f.write_constant_bytes("COLOUR_CFG", COLOUR_CFG);
    f.write_constant_bytes("COLOUR_XML", COLOUR_XML);
    f.write_constant_bytes("COLOUR_YML", COLOUR_YML);
    f.write_constant_bytes("COLOUR_TS", COLOUR_TS);
    f.write_constant_bytes("COLOUR_SH", COLOUR_SH);
    f.write_constant_bytes("COLOUR_BAT", COLOUR_BAT);
    f.write_constant_bytes("COLOUR_RB", COLOUR_RB);
    f.write_constant_bytes("COLOUR_PHP", COLOUR_PHP);
    f.write_constant_bytes("COLOUR_PL", COLOUR_PL);
    f.write_constant_bytes("COLOUR_R", COLOUR_R);
    f.write_constant_bytes("COLOUR_CS", COLOUR_CS);
    f.write_constant_bytes("COLOUR_JAVA", COLOUR_JAVA);
    f.write_constant_bytes("COLOUR_GO", COLOUR_GO);
    f.write_constant_bytes("COLOUR_SWIFT", COLOUR_SWIFT);
    f.write_constant_bytes("COLOUR_KT", COLOUR_KT);
    f.write_constant_bytes("COLOUR_SCSS", COLOUR_SCSS);
    f.write_constant_bytes("COLOUR_LESS", COLOUR_LESS);
    f.write_constant_bytes("COLOUR_CSV", COLOUR_CSV);
    f.write_constant_bytes("COLOUR_TSV", COLOUR_TSV);
    f.write_constant_bytes("COLOUR_XLS", COLOUR_XLS);
    f.write_constant_bytes("COLOUR_XLSX", COLOUR_XLSX);
    f.write_constant_bytes("COLOUR_SQL", COLOUR_SQL);
    f.write_constant_bytes("COLOUR_SYMLINK_DEFAULT", COLOUR_SYMLINK_DEFAULT);
    f.write_constant_bytes("COLOUR_DIRECTORY_DEFAULT", COLOUR_DIRECTORY_DEFAULT);
    f.write_constant_bytes("COLOUR_SOCKET_DEFAULT", COLOUR_SOCKET_DEFAULT);
    f.write_constant_bytes("COLOUR_PIPE_DEFAULT", COLOUR_PIPE_DEFAULT);
    f.write_constant_bytes("COLOUR_BLOCK_DEVICE_DEFAULT", COLOUR_BLOCK_DEVICE_DEFAULT);
    f.write_constant_bytes("COLOUR_CHARACTER_DEVICE_DEFAULT", COLOUR_CHARACTER_DEVICE_DEFAULT);
    f.write_constant_bytes("COLOUR_EXECUTABLE_DEFAULT", COLOUR_EXECUTABLE_DEFAULT);
    f.write_constant_bytes("COLOUR_STICKY_DEFAULT", COLOUR_STICKY_DEFAULT);
    f.write_constant_bytes("COLOUR_OTHER_WRITABLE_DEFAULT", COLOUR_OTHER_WRITABLE_DEFAULT);
    f.write_constant_bytes("COLOUR_ORPHAN_SYMLINK_DEFAULT", COLOUR_ORPHAN_SYMLINK_DEFAULT);
    f.write_constant_bytes("COLOUR_SETUID_DEFAULT", COLOUR_SETUID_DEFAULT);
    f.write_constant_bytes("COLOUR_SETGID_DEFAULT", COLOUR_SETGID_DEFAULT);
    f.write_code("use phf::phf_map;");
    f.write_comment("This is a compile-time hash map of file extensions to their corresponding ANSI colour codes");
    f.write_comment(" based on the `LS_COLORS` environment variable.\n");
    f.write_comment("It provides colour coding for file types in terminal applications.");
    f.write_comment("Keys are byte slices representing file extensions.");
    f.write_comment(" Values are byte slices representing ANSI escape sequences.");
    f.write_comment(" Generated at build time from the LS_COLORS environment variable." );


    f.write_code("pub static LS_COLOURS_HASHMAP: phf::Map<&'static [u8], &'static [u8]> = phf_map! {");



    for (key, escape_seq) in &colour_map {
       
       
        f.write_escape_colour_code(&key,&escape_seq)
    }

    f.write_code("};");


    f.write_code("use std::collections::HashMap;");
    f.write_code("use std::hash::BuildHasherDefault;");
    f.write_code("use std::hash::DefaultHasher;");
    f.write_code("use std::sync::LazyLock;");
    f.write_comment("This is a compile-time generated array of file extensions and their corresponding ANSI colour codes");
    f.write_comment("based on the `LS_COLORS` environment variable and default fallbacks.");
    f.write_comment("It provides colour coding for file types in terminal applications.");
    f.write_comment(" Keys are byte slices representing file extensions.");
    f.write_comment("Values are byte slices representing ANSI escape sequences.");
    f.write_comment("Generated at build time from the LS_COLORS environment variable.");
    f.write_code("pub const LS_COLOURS_DATA: &[(&'static [u8], &'static [u8])] = &[");

        for (key, escape_seq) in &colour_map {
        let escaped_seq = escape_bytes(&escape_seq);
        // Write the key-value pair as a tuple in the const array
        writeln!(f, "    (b\"{}\", b\"{}\"),", key, escaped_seq).unwrap();
    }

    f.write_code( "];\n");

    f.write_comment("This is a lazily initialized HashMap of file extensions to their corresponding ANSI colour codes.");
    f.write_comment(" It is built once at runtime from the `LS_COLORS_CUSTOM` ");
    f.write_comment("the default (LS_COLOR) is used if the environment variable is not set. This is an optional feature to allow custom colours easily.");

      
       writeln!(
        f,
        "pub static LS_COLOURS_HASHMAP_RUNTIME: LazyLock<HashMap<&'static [u8], &'static [u8], BuildHasherDefault<DefaultHasher>>> = LazyLock::new(|| {{"
    )
    .unwrap();
    writeln!(
        f,
        "    let mut map = HashMap::with_capacity_and_hasher(LS_COLOURS_DATA.len(), BuildHasherDefault::new());"
    )
    .unwrap();
    writeln!(f, "    for (key, value) in LS_COLOURS_DATA {{").unwrap();
    f.write_code("map.insert(*key, *value);");
    writeln!(f, "    }}");
    f.write_code( "    map");
    writeln!(f, "}});").unwrap();



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

