include!(concat!(env!("OUT_DIR"), "/ls_colors.rs"));
#[cfg(test)]
mod test;




////// Returns the color code for a given file extension.
/// If the extension is not found in the LS_COLORS_MAP, it returns the provided alternative
pub fn colour_hashmap<'a>(ext:&'a[u8],or_alternative:&'a[u8])->&'a[u8]{
    LS_COLOURS_HASHMAP.get(ext).map(|v| &**v).unwrap_or_else(||or_alternative)
}


// Returns the color code for a given file extension.
pub fn colour_map<'a>(ext:&'a[u8])->Option<&'a[u8]>{
    LS_COLOURS_HASHMAP.get(ext).map(|v| &**v)
}


