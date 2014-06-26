pub struct FileHandler {
    bump: int
}

impl FileHandler {
    pub fn new() -> FileHandler {
        FileHandler {
            bump: 0,
        }
    }
}
