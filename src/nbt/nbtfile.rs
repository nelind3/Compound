use quartz_nbt::{
    io::{self, Flavor, NbtIoError},
    NbtCompound, NbtTag,
};
use std::{collections::VecDeque, fs::File, path::PathBuf};

#[derive(Debug, Clone)]
pub struct NbtFile {
    pub path: PathBuf,
    pub flavor: Flavor,
    pub roottag: NbtCompound,
    pub is_edited: bool,
}

impl NbtFile {
    pub async fn new(path: PathBuf, flavor: Flavor) -> Result<Self, NbtIoError> {
        let (tag, _name) = io::read_nbt(&mut File::open(&path)?, flavor)?;

        Ok(NbtFile {
            path,
            flavor,
            roottag: tag,
            is_edited: false,
        })
    }

    pub fn set_tag(&mut self, tag_path: String, value: NbtTag) -> Result<(), anyhow::Error> {
        if !self.is_edited {
            self.is_edited = true
        }

        let mut tag_names: VecDeque<_> = tag_path.splitn(usize::MAX, '.').collect();
        let tag = super::get_tags_from_compound(&mut tag_names, &mut self.roottag)?;
        *tag = value;

        Ok(())
    }

    pub fn write_to_disk(&self) -> Result<(), NbtIoError> {
        io::write_nbt(
            &mut File::create(&self.path)?,
            None,
            &self.roottag,
            self.flavor,
        )
    }
}
