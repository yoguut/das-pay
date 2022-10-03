use std::path::PathBuf;

#[derive(Debug)]
pub struct Opt {
    pub file_path: PathBuf,
}

impl Opt {
    /// Returns a instance of Opt if path exist,
    /// else returns None.
    pub fn new(path: String) -> Option<Opt> {
        let p = PathBuf::from(path);
        if p.exists() {
            let opt = Opt { file_path: p };
            Some(opt)
        } else {
            None
        }
    }
}
