use quartz_nbt::{
    io::{self, Flavor, NbtIoError},
    NbtCompound, NbtTag,
};
use std::{collections::VecDeque, fs::File, os::unix::fs::FileExt, path::PathBuf};

#[derive(Debug, Clone)]
pub struct NbtFile {
    pub path: PathBuf,
    pub flavor: Flavor,
    pub roottag: (NbtCompound, String),
    pub is_edited: bool,
}

impl NbtFile {
    pub async fn new(path: PathBuf) -> Result<Self, NbtIoError> {
        let mut detect_comp_buf: [u8; 2] = [0, 0];
        {
            let nbt_file_handle = File::open(&path)?;
            nbt_file_handle.read_exact_at(&mut detect_comp_buf, 0)?;
        }
        match detect_comp_buf {
            [0x1f, 0x8b] => Self::new_with_flavor(path, Flavor::GzCompressed),
            [0x78, ..] => Self::new_with_flavor(path, Flavor::ZlibCompressed),
            _ => Self::new_with_flavor(path, Flavor::Uncompressed),
        }
    }

    pub fn new_with_flavor(path: PathBuf, flavor: Flavor) -> Result<Self, NbtIoError> {
        let (tag, name) = io::read_nbt(&mut File::open(&path)?, flavor)?;

        Ok(NbtFile {
            path,
            flavor,
            roottag: (tag, name),
            is_edited: false,
        })
    }

    pub fn set_tag(&mut self, tag_path: String, value: NbtTag) -> Result<(), anyhow::Error> {
        if !self.is_edited {
            self.is_edited = true
        }

        let mut tag_names: VecDeque<_> = tag_path.splitn(usize::MAX, '.').collect();
        let tag = super::get_tags_from_compound(&mut tag_names, &mut self.roottag.0)?;
        *tag = value;

        Ok(())
    }

    pub fn write_to_disk(&self) -> Result<(), NbtIoError> {
        io::write_nbt(
            &mut File::create(&self.path)?,
            Some(self.roottag.1.as_str()),
            &self.roottag.0,
            self.flavor,
        )
    }
}
