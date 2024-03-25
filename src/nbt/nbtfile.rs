use quartz_nbt::{
    io::{self, Flavor, NbtIoError},
    NbtCompound, NbtTag,
};
use std::{fs::File, io::Seek, os::unix::fs::FileExt, sync::Arc};

#[derive(Debug, Clone)]
pub struct NbtFile {
    pub file: Arc<File>,
    pub flavor: Flavor,
    pub roottag: (NbtCompound, String),
    pub is_edited: bool,
}

impl NbtFile {
    pub fn new(file: File) -> Result<Self, NbtIoError> {
        let mut detect_comp_buf: [u8; 2] = [0, 0];
        file.read_exact_at(&mut detect_comp_buf, 0)?;

        match detect_comp_buf {
            [0x1f, 0x8b] => Self::new_with_flavor(file, Flavor::GzCompressed),
            [0x78, ..] => Self::new_with_flavor(file, Flavor::ZlibCompressed),
            _ => Self::new_with_flavor(file, Flavor::Uncompressed),
        }
    }

    pub fn new_with_flavor(mut file: File, flavor: Flavor) -> Result<Self, NbtIoError> {
        let (tag, name) = io::read_nbt(&mut file, flavor)?;

        Ok(NbtFile {
            file: Arc::new(file),
            flavor,
            roottag: (tag, name),
            is_edited: false,
        })
    }

    pub fn set_tag(&mut self, tag_path: String, value: NbtTag) -> Result<(), anyhow::Error> {
        if !self.is_edited {
            self.is_edited = true
        }

        let mut tag_names = tag_path.split('.').rev().collect();
        let tag = super::get_tags_from_compound(&mut tag_names, &mut self.roottag.0)?;
        *tag = value;

        Ok(())
    }

    pub fn write_to_disk(&mut self) -> Result<(), NbtIoError> {
        self.file.rewind()?;
        io::write_nbt(
            &mut self.file,
            Some(self.roottag.1.as_str()),
            &self.roottag.0,
            self.flavor,
        )?;

        log::info!("Saved to {:?}", self.file);
        Ok(())
    }
}
