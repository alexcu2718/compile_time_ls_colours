#[cfg(test)]
mod tests {
    use crate::LS_COLOURS_HASHMAP;
   

    #[test]
    fn test_common_extensions() {
        let test_extensions = [
            "rs", "py", "txt", "jpg", "mp3", "zip", 
            "gz", "html", "css", "js", "json", "md",
            "png", "svg", "mov", "pdf", "docx", "xlsx"
        ];
        
        println!("Testing common extensions:");
        for ext in test_extensions {
            if let Some(escape_seq) = LS_COLOURS_HASHMAP.get(ext.as_bytes()) {
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
    fn test_edge_cases() {
        let edge_cases = [
            "",          // empty string
            " ",         // space
            "tar.gz",   // compound extension
            ".hidden",   // dotfile
            "UPPER",    // uppercase
            "MiXeD",    // mixed case
            "with space",// contains space
            "with.dot",  // contains dot
            "verylongextensionnametotestbufferlimits", // very long
        ];
        
        println!("Testing edge cases:");
        for case in edge_cases {
            match LS_COLOURS_HASHMAP.get(case.as_bytes()) {
                Some(seq) => {
                    println!("Case '{}' has color sequence: {:?}", case, seq);
                    print!("\x1b[0mExample '{}'", case);
                    print!("{}", String::from_utf8_lossy(seq));
                    println!("\x1b[0m");
                },
                None => {
                    println!("Case '{}' has no color mapping", case);
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
                "LS_COLOURS_HASHMAP should contain extension '{}'", ext
            );
        }
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
        println!("Regular text: No color mapping");
        println!("Colored text: Has color mapping");
    }

   

 
}