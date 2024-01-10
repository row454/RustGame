use std::{any::TypeId, hash::Hasher, fmt::{Display, Formatter}};

pub mod position;

pub trait Component: 'static + Sized + Send + Sync {

}

impl<T: 'static + Sized + Send + Sync> Component for T {

}
#[derive(Copy, Clone, Debug, Eq, PartialOrd, Ord)]
pub struct ComponentTypeId {
    pub(crate) type_id: TypeId,
    #[cfg(debug_assertions)]
    name: &'static str,
}

impl ComponentTypeId {
    pub fn of<T: Component>() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            #[cfg(debug_assertions)]
            name: std::any::type_name::<T>(),
        }
    }

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }
}

impl std::hash::Hash for ComponentTypeId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_id.hash(state);
    }
}

impl PartialEq for ComponentTypeId {
    fn eq(&self, other: &Self) -> bool {
        self.type_id.eq(&other.type_id)
    }
}

impl Display for ComponentTypeId {
    #[cfg(debug_assertions)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        write!(f, "{}", self.name)
    }

    #[cfg(not(debug_assertions))]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.type_id)
    }
}
