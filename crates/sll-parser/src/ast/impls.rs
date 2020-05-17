use super::*;

#[derive(Debug, Clone)]
pub struct Impl<'a> {
    pub tys: BumpVec<'a, BoundedType<'a>>,
    pub items: BumpVec<'a, Item<'a>>,
}
