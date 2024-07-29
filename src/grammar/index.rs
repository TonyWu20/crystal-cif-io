use std::fmt::Debug;
use std::ops;

use crate::{DataBlock, DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

use super::CifDocument;

pub trait Index<Value>: private::Sealed {
    type Item;
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Self::Item>;
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Self::Item>;
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Self::Item;
}

impl Index<CifDocument> for usize {
    type Item = DataBlock;

    fn index_into<'v>(&self, v: &'v CifDocument) -> Option<&'v Self::Item> {
        v.data_blocks().and_then(|datablocks| datablocks.get(*self))
    }

    fn index_into_mut<'v>(&self, v: &'v mut CifDocument) -> Option<&'v mut Self::Item> {
        v.data_blocks_mut()
            .as_mut()
            .and_then(|datablocks| datablocks.get_mut(*self))
    }

    fn index_or_insert<'v>(&self, v: &'v mut CifDocument) -> &'v mut Self::Item {
        let len = v
            .data_blocks()
            .map(|blocks| blocks.len())
            .unwrap_or_else(|| panic!("the document does not have any data block!"));
        v.data_blocks_mut()
            .as_mut()
            .and_then(|blocks| blocks.get_mut(*self))
            .unwrap_or_else(|| {
                panic!(
                    "cannot access index {} of {} datablocks in the cif document",
                    *self, len
                )
            })
    }
}

impl Index<CifDocument> for str {
    type Item = DataBlock;

    fn index_into<'v>(&self, v: &'v CifDocument) -> Option<&'v Self::Item> {
        v.data_blocks()
            .and_then(|datablocks| datablocks.iter().find(|block| block.heading() == self))
    }

    fn index_into_mut<'v>(&self, v: &'v mut CifDocument) -> Option<&'v mut Self::Item> {
        v.data_blocks_mut()
            .as_mut()
            .and_then(|datablocks| datablocks.iter_mut().find(|block| block.heading() == self))
    }

    fn index_or_insert<'v>(&self, v: &'v mut CifDocument) -> &'v mut Self::Item {
        self.index_into_mut(v).unwrap_or_else(|| {
            panic!(
                "cannot access datablock with name '{}' in the cif document",
                self
            )
        })
    }
}

impl Index<CifDocument> for String {
    type Item = DataBlock;

    fn index_into<'v>(&self, v: &'v CifDocument) -> Option<&'v Self::Item> {
        self[..].index_into(v)
    }

    fn index_into_mut<'v>(&self, v: &'v mut CifDocument) -> Option<&'v mut Self::Item> {
        self[..].index_into_mut(v)
    }

    fn index_or_insert<'v>(&self, v: &'v mut CifDocument) -> &'v mut Self::Item {
        self[..].index_or_insert(v)
    }
}

impl Index<DataBlock> for usize {
    type Item = DataItems;

    fn index_into<'v>(&self, v: &'v DataBlock) -> Option<&'v Self::Item> {
        v.members().get(*self).and_then(|member| {
            match member {
                DataBlockMember::DataItems(item) => Some(item),
                DataBlockMember::SaveFrame(_) => None, // not supported
            }
        })
    }

    fn index_into_mut<'v>(&self, v: &'v mut DataBlock) -> Option<&'v mut Self::Item> {
        v.members_mut()
            .get_mut(*self)
            .and_then(|member| match member {
                DataBlockMember::DataItems(item) => Some(item),
                DataBlockMember::SaveFrame(_) => None,
            })
    }

    fn index_or_insert<'v>(&self, v: &'v mut DataBlock) -> &'v mut Self::Item {
        let len = v.members().len();
        v.members_mut()
            .get_mut(*self)
            .and_then(|member| match member {
                DataBlockMember::DataItems(item) => Some(item),
                DataBlockMember::SaveFrame(_) => None,
            })
            .unwrap_or_else(|| {
                panic!(
                    "cannot access index {} of data block members of length {}",
                    self, len
                )
            })
    }
}

impl Index<DataBlock> for str {
    type Item = DataItems;

    fn index_into<'v>(&self, v: &'v DataBlock) -> Option<&'v Self::Item> {
        v.members().iter().find_map(|member| match member {
            DataBlockMember::DataItems(item) => match item {
                crate::DataItems::SingleValue(v) => {
                    if v.tag().as_str() == self {
                        Some(item)
                    } else {
                        None
                    }
                }
                crate::DataItems::MultiValues(m) => {
                    if m.find_loop_column_by_tag(self).is_some() {
                        Some(item)
                    } else {
                        None
                    }
                }
            },
            DataBlockMember::SaveFrame(_) => None,
        })
    }

    fn index_into_mut<'v>(&self, v: &'v mut DataBlock) -> Option<&'v mut Self::Item> {
        v.members_mut().iter_mut().find_map(|member| match member {
            DataBlockMember::DataItems(item) => match item {
                crate::DataItems::SingleValue(v) => {
                    if v.tag().as_str() == self {
                        Some(item)
                    } else {
                        None
                    }
                }
                crate::DataItems::MultiValues(m) => {
                    if m.find_loop_column_by_tag(self).is_some() {
                        Some(item)
                    } else {
                        None
                    }
                }
            },
            DataBlockMember::SaveFrame(_) => None,
        })
    }

    fn index_or_insert<'v>(&self, v: &'v mut DataBlock) -> &'v mut Self::Item {
        self.index_into_mut(v)
            .unwrap_or_else(|| panic!("cannot access item with tag {} of data block members", self))
    }
}

impl Index<DataBlock> for String {
    type Item = DataItems;

    fn index_into<'v>(&self, v: &'v DataBlock) -> Option<&'v Self::Item> {
        self[..].index_into(v)
    }

    fn index_into_mut<'v>(&self, v: &'v mut DataBlock) -> Option<&'v mut Self::Item> {
        self[..].index_into_mut(v)
    }

    fn index_or_insert<'v>(&self, v: &'v mut DataBlock) -> &'v mut Self::Item {
        self[..].index_or_insert(v)
    }
}

impl Index<LoopColumns> for usize {
    type Item = LoopColumn;

    fn index_into<'v>(&self, v: &'v LoopColumns) -> Option<&'v Self::Item> {
        v.columns().get(*self)
    }

    fn index_into_mut<'v>(&self, v: &'v mut LoopColumns) -> Option<&'v mut Self::Item> {
        v.columns_mut().get_mut(*self)
    }

    fn index_or_insert<'v>(&self, v: &'v mut LoopColumns) -> &'v mut Self::Item {
        let len = v.columns().len();
        self.index_into_mut(v)
            .unwrap_or_else(|| panic!("cannot access index {} of {} loop columns", self, len))
    }
}

impl Index<LoopColumns> for str {
    type Item = LoopColumn;

    fn index_into<'v>(&self, v: &'v LoopColumns) -> Option<&'v Self::Item> {
        v.find_loop_column_by_tag(self)
    }

    fn index_into_mut<'v>(&self, v: &'v mut LoopColumns) -> Option<&'v mut Self::Item> {
        v.find_loop_column_mut_by_tag(self)
    }

    fn index_or_insert<'v>(&self, v: &'v mut LoopColumns) -> &'v mut Self::Item {
        self.index_into_mut(v)
            .unwrap_or_else(|| panic!("cannot access column with tag {} in the loop", self))
    }
}

impl Index<LoopColumns> for String {
    type Item = LoopColumn;

    fn index_into<'v>(&self, v: &'v LoopColumns) -> Option<&'v Self::Item> {
        self[..].index_into(v)
    }

    fn index_into_mut<'v>(&self, v: &'v mut LoopColumns) -> Option<&'v mut Self::Item> {
        self[..].index_into_mut(v)
    }

    fn index_or_insert<'v>(&self, v: &'v mut LoopColumns) -> &'v mut Self::Item {
        self[..].index_or_insert(v)
    }
}

impl<T> Index<CifDocument> for &T
where
    T: ?Sized + Index<CifDocument, Item = DataBlock>,
{
    type Item = DataBlock;

    fn index_into<'v>(&self, v: &'v CifDocument) -> Option<&'v Self::Item> {
        (**self).index_into(v)
    }

    fn index_into_mut<'v>(&self, v: &'v mut CifDocument) -> Option<&'v mut Self::Item> {
        (**self).index_into_mut(v)
    }

    fn index_or_insert<'v>(&self, v: &'v mut CifDocument) -> &'v mut Self::Item {
        (**self).index_or_insert(v)
    }
}

impl<T> Index<DataBlock> for &T
where
    T: ?Sized + Index<DataBlock, Item = DataItems>,
{
    type Item = DataItems;

    fn index_into<'v>(&self, v: &'v DataBlock) -> Option<&'v Self::Item> {
        (**self).index_into(v)
    }

    fn index_into_mut<'v>(&self, v: &'v mut DataBlock) -> Option<&'v mut Self::Item> {
        (**self).index_into_mut(v)
    }

    fn index_or_insert<'v>(&self, v: &'v mut DataBlock) -> &'v mut Self::Item {
        (**self).index_or_insert(v)
    }
}

impl<T> Index<LoopColumns> for &T
where
    T: ?Sized + Index<LoopColumns, Item = LoopColumn>,
{
    type Item = LoopColumn;

    fn index_into<'v>(&self, v: &'v LoopColumns) -> Option<&'v Self::Item> {
        (**self).index_into(v)
    }

    fn index_into_mut<'v>(&self, v: &'v mut LoopColumns) -> Option<&'v mut Self::Item> {
        (**self).index_into_mut(v)
    }

    fn index_or_insert<'v>(&self, v: &'v mut LoopColumns) -> &'v mut Self::Item {
        (**self).index_or_insert(v)
    }
}

impl<I> ops::Index<I> for DataBlock
where
    I: Index<DataBlock, Item = DataItems>,
{
    type Output = DataItems;

    fn index(&self, index: I) -> &Self::Output {
        static NULL: DataItems = DataItems::SingleValue(SingleLineData::null());
        index.index_into(self).unwrap_or(&NULL)
    }
}

impl<I> ops::IndexMut<I> for DataBlock
where
    I: Index<DataBlock, Item = DataItems>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        index.index_or_insert(self)
    }
}

impl<I> ops::Index<I> for CifDocument
where
    I: Index<CifDocument, Item = DataBlock> + Debug,
{
    type Output = DataBlock;

    fn index(&self, index: I) -> &Self::Output {
        index
            .index_into(self)
            .unwrap_or_else(|| panic!("cannot access the datablock by index {:?}", index))
    }
}

impl<I> ops::IndexMut<I> for CifDocument
where
    I: Index<CifDocument, Item = DataBlock> + Debug,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        index.index_or_insert(self)
    }
}

impl<I> ops::Index<I> for LoopColumns
where
    I: Index<LoopColumns, Item = LoopColumn> + Debug,
{
    type Output = LoopColumn;

    fn index(&self, index: I) -> &Self::Output {
        index.index_into(self).unwrap_or_else(|| {
            panic!(
                "cannot access column by index {:?} in loop columns {:?}",
                index, self
            )
        })
    }
}

impl<I> ops::IndexMut<I> for LoopColumns
where
    I: Index<LoopColumns, Item = LoopColumn> + Debug,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        index.index_or_insert(self)
    }
}

mod private {

    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for String {}
    impl<'a, T> Sealed for &'a T where T: ?Sized + Sealed {}
}
