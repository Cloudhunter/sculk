use crate::{
    error::SculkParseError, item::Item, traits::FromCompoundNbt, util::get_t_compound_vec,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ChiseledBookshelf<'a> {
    /// List of books in the bookshelf.  
    ///
    /// The valid slot numbers are 0-5.
    ///
    /// `Items`
    pub items: Vec<Item<'a>>,

    /// Last interacted slot (0–5), or -1 if no slot has been interacted with yet.
    pub last_interacted_slot: i32,
}

impl<'a> FromCompoundNbt for ChiseledBookshelf<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let items = get_t_compound_vec(&nbt, "Items", Item::from_compound_nbt)?;
        let last_interacted_slot = nbt
            .int("last_interacted_slot")
            .ok_or(SculkParseError::MissingField("last_interacted_slot".into()))?;

        Ok(ChiseledBookshelf {
            items,
            last_interacted_slot,
        })
    }
}
