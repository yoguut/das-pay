use std::path::PathBuf;

#[derive(Debug)]
pub struct Opt {
    pub file_path: PathBuf,
}

impl Opt {
    pub fn new(path: String) -> Option<Opt> {
        // validate input path
        let p = PathBuf::from(path);
        if p.exists() {
            let opt = Opt { file_path: p };
            Some(opt)
        } else {
            None
        }
    }
}
