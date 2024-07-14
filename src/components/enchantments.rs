use crate::{kv::KVPair, traits::FromCompoundNbt};

#[derive(Debug, Clone, PartialEq)]
pub struct Enchantments<'a> {
    /// Contains key-value pairs of levels of enchantments on this item that affect the way the item works.  
    pub enchantments: KVPair<'a, i32>,

    /// Show or hide enchantments on this item's tooltip. Defaults to true.
    pub show_in_tooltip: bool,
}

impl<'a> FromCompoundNbt for Enchantments<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        if let Some(levels) = nbt.compound("levels") {
            // Field compound

            let enchantments = KVPair::<'a, i32>::from_compound_nbt(&levels)?;
            let show_in_tooltip = nbt.byte("show_in_tooltip").map(|b| b != 0).unwrap_or(true);

            return Ok(Enchantments {
                enchantments,
                show_in_tooltip,
            });
        } else {
            // key value only
            let enchantments = KVPair::<'a, i32>::from_compound_nbt(&nbt)?;

            // this could be an enum but eh, show_in_tooltip is default true anyway
            return Ok(Enchantments {
                enchantments,
                show_in_tooltip: true,
            });
        };
    }
}