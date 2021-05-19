use std::{
    fs::{create_dir_all, remove_dir_all},
    io::Error,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct TemporaryDirectory(pub PathBuf);

impl TemporaryDirectory {
    pub fn create() -> Result<Self, Error> {
        let path = PathBuf::from(format!(
            "/tmp/inc-{}",
            COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        create_dir_all(&path)?;
        Ok(Self(path))
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = remove_dir_all(&self.0);
    }
}
