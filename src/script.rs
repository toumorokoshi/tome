// used to determine if the file is a valid script or not
pub fn is_tome_script(filename: &str) -> bool {
    !filename.starts_with('.')
}
