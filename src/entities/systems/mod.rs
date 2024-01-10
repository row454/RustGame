use super::{components::{Component, ComponentTypeId}, World};


pub struct Query<'a> {
    world: &'a World,
    archetypes: Vec<u64>,
    components: Vec<ComponentTypeId>,
}

impl Query<'_> {
}

pub trait IntoQuery {
    fn query(world: &World) -> Query;
}



macro_rules! tuple_impls {
    ($first:ident $( $name:ident )+ ) => {
        impl<$first: Component,$($name: Component),+> IntoQuery for ($first,$($name,)+)
        {
            fn query(world: &World) -> Query {
                if let Some(archetype_set) = world.archetype_sets.get(&ComponentTypeId::of::<$first>()) {
                    Query {
                        world,
                        archetypes: archetype_set.iter().copied().filter(|k| $(world.archetype_sets.get(&ComponentTypeId::of::<$name>()).and_then(|set| Some(set.contains(k))).unwrap_or(false))&&+).collect(),
                        components: vec![$(ComponentTypeId::of::<$name>()),+]
                    }
                } else {
                    Query {
                        world,
                        archetypes: Vec::new(),
                        components: vec![$(ComponentTypeId::of::<$name>()),+]
                    }
                }
            }
        }
    };
}




impl<T: Component> IntoQuery for (T,) {
    fn query(world: &World) -> Query {
        if let Some(archetype_set) = world.archetype_sets.get(&ComponentTypeId::of::<T>()) {
            Query {
                world,
                archetypes: archetype_set.iter().copied().collect(),
                components: vec![ComponentTypeId::of::<T>()]
            } 
        }
        else {
            Query {
                world,
                archetypes: Vec::new(),
                components: vec![ComponentTypeId::of::<T>()]
            }
        }
    }
}


tuple_impls! { A B }
tuple_impls! { A B C }
tuple_impls! { A B C D }
tuple_impls! { A B C D E }
tuple_impls! { A B C D E F }
tuple_impls! { A B C D E F G }
tuple_impls! { A B C D E F G H }
tuple_impls! { A B C D E F G H I }
tuple_impls! { A B C D E F G H I J }
tuple_impls! { A B C D E F G H I J K }
tuple_impls! { A B C D E F G H I J K L }


