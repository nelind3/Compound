mod nbtfile;
pub use nbtfile::NbtFile;

pub fn get_tags_from_compound<'a>(
    tag_names: &mut Vec<&str>,
    compound_tag: &'a mut quartz_nbt::NbtCompound,
) -> Result<&'a mut quartz_nbt::NbtTag, anyhow::Error> {
    let opt_tag_name = tag_names.pop();
    if let Some(tag_name) = opt_tag_name {
        let tag = compound_tag.get_mut::<_, &mut quartz_nbt::NbtTag>(tag_name)?;
        if let quartz_nbt::NbtTag::Compound(compound) = tag {
            get_tags_from_compound(tag_names, compound)
        } else {
            Ok(tag)
        }
    } else {
        anyhow::bail!("More tag names than depth of provided compound!")
    }
}
