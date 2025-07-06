#[cfg(test)]
mod tests {
    use crate::LS_COLOURS_HASHMAP;
    use std::io::{self, BufWriter, Write};

    #[test]
    fn test_common_extensions() {
        let mut writer = BufWriter::new(io::stdout());

        let test_extensions = [
            "rs", "py", "txt", "jpg", "mp3", "zip", "gz", "html", "css", "js", "json", "md", "png",
            "svg", "mov", "pdf", "docx", "xlsx",
        ];

        println!("Testing common extensions:");
        for ext in test_extensions {
            if let Some(escape_seq) = LS_COLOURS_HASHMAP.get(ext.as_bytes()) {
                writer.write(escape_seq).unwrap(); // Write the escape sequence to the buffer
                writer.flush().unwrap();
                print!("\x1b[0m.{}", String::from_utf8_lossy(escape_seq));
                print!("{}", ext);
                print!("\x1b[0m ");
            } else {
                print!(".{} ", ext);
            }
        }
        println!("\n");
    }

    #[test]
    fn test_file_type_colours() {
        use crate::file_type_colour;
        // Test special file types
        let special_types = [
            ("symlink", file_type_colour!(symlink)),
            ("directory", file_type_colour!(directory)),
            ("socket", file_type_colour!(socket)),
            ("pipe", file_type_colour!(pipe)),
            ("block_device", file_type_colour!(block_device)),
            ("character_device", file_type_colour!(character_device)),
            ("executable", file_type_colour!(executable)),
        ];

        println!("Testing special file type colours:");
        for (name, colour_seq) in special_types {
            // verify ANSI escape sequence
            assert!(!colour_seq.is_empty(), "{} colour is empty", name);
            assert_eq!(
                colour_seq[0], 0x1B,
                "{} colour doesn't start with escape",
                name
            );

            // check the visual output
            print!("\x1b[0m{}: ", name);
            print!("{}", String::from_utf8_lossy(colour_seq));
            println!("Sample Text\x1b[0m");
        }
    }

    #[test]
    fn test_edge_cases() {
        let edge_cases = [
            "",                                        // empty string
            " ",                                       // space
            "tar.gz",                                  // compound extension
            ".hidden",                                 // dotfile
            "UPPER",                                   // uppercase
            "MiXeD",                                   // mixed case
            "with space",                              // contains space
            "with.dot",                                // contains dot
            "verylongextensionnametotestbufferlimits", // very long
        ];

        println!("Testing edge cases:");
        for case in edge_cases {
            match LS_COLOURS_HASHMAP.get(case.as_bytes()) {
                Some(seq) => {
                    println!("Case '{}' has colour sequence: {:?}", case, seq);
                    print!("\x1b[0mExample '{}'", case);
                    print!("{}", String::from_utf8_lossy(seq));
                    println!("\x1b[0m");
                }
                None => {
                    println!("Case '{}' has no colour mapping", case);
                }
            }
        }
    }

    #[test]
    fn test_map_integrity() {
        // Verify the map contains expected entries
        let must_have = ["rs", "py", "sh", "go", "c", "cpp"];

        for ext in must_have {
            assert!(
                LS_COLOURS_HASHMAP.contains_key(ext.as_bytes()),
                "LS_COLOURS_HASHMAP should contain extension '{}'",
                ext
            );
        }
    }
    #[test]
    fn test_dtypes() {
        use std::io::{self, BufWriter, Write};

        // Check if the LS_COLOURS_HASHMAP contains entries for all expected dtypes
        // this is necessary to not crash at runtime if the hashmap is missing any of these keys.
        println!("Testing dtype presence in LS_COLOURS_HASHMAP:");
        let mut writer = BufWriter::new(io::stdout());
        let dtypes = [
            "symlink",
            "directory",
            "socket",
            "pipe",
            "block_device",
            "character_device",
            "executable",
        ];

        for dtype in dtypes {
            assert!(
                LS_COLOURS_HASHMAP.contains_key(dtype.as_bytes()),
                "LS_COLOURS_HASHMAP should contain dtype '{}'",
                dtype
            );
        }

        writeln!(
            writer,
            "\n--- LS_COLOURS_HASHMAP Contents & Applied Colours ---"
        )
        .unwrap();

        for (key_bytes, value_bytes) in LS_COLOURS_HASHMAP.into_iter() {
            if let Ok(key_str) = std::str::from_utf8(key_bytes) {
                // Convert the value bytes (ANSI code) to a string for display purposes.
                // If it's not valid UTF-8 then something is wrong.

                let colour_code_str =
                    std::str::from_utf8(value_bytes).unwrap_or("<NON-UTF8 ANSI CODE>");

                // --- Applying the colour for visual output ---
                let coloured_example = format!(
                    "{}{}{}",
                    colour_code_str, // The actual colour code from the hashmap
                    "THIS TEXT IS COLOURED",
                    "\x1b[0m" // Reset colour to default
                );

                writeln!(
                    writer,
                    "Key: {:<20}   | Applied: {}",
                    key_str,
                    // Shows the raw bytes of the ANSI code
                    coloured_example // Shows the text rendered with the actual colour
                )
                .unwrap();
            } else {
                unreachable!(
                    // This should never happen as we expect all keys to be valid UTF-8
                    "Key bytes {:?} are not valid UTF-8",
                    key_bytes
                );
            }
        }
        writeln!(writer, "-----------------------------------").unwrap();
        writer.flush().unwrap(); // Ensures all buffered output is written immediately
    }

    #[test]
    fn test_visual_output() {
        // Generate a visual test output for manual verification
        println!("\nVisual test output:");
        println!("{:-^40}", " FILE TYPES ");

        let categories: &[(&str, &[&str])] = &[
            ("Source Files", &["rs", "py", "c", "go", "java", "hs"]),
            ("Documents", &["pdf", "docx", "txt", "md", "tex"]),
            ("Media", &["jpg", "png", "mp3", "mp4", "gif"]),
            ("Archives", &["zip", "tar.gz", "rar", "7z"]),
            ("System", &["so", "dll", "exe", "bin"]),
        ];

        for (category, exts) in categories {
            println!("\n{}:", category);
            for ext in *exts {
                if let Some(seq) = LS_COLOURS_HASHMAP.get(ext.as_bytes()) {
                    print!("\x1b[0m.{}", String::from_utf8_lossy(seq));
                    print!("{:<8}", ext);
                    print!("\x1b[0m");
                } else {
                    print!(".{:<8} ", ext);
                }
            }
            println!();
        }
        println!("\n{:-^40}", " LEGEND ");
        println!("Regular text: No colour mapping");
        println!("Coloured text: Has colour mapping");
    }
}
