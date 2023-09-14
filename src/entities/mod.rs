use core::arch;
use std::{collections::{HashMap, hash_map::DefaultHasher}, sync::Arc, hash::{Hash, Hasher}, f32::consts::E, any::{Any, type_name}};

use self::components::{Storage, Component, storage_cast_mut};

mod archetype;
mod components;
type EntityId = u32;
pub const VOID_ARCHETYPE: u64 = u64::MAX;


struct World {
	entity_count: u32,
	entities: HashMap<EntityId, Pointer>,
	archetypes: HashMap<u64, ArchetypeStorage>
}
#[derive(Clone)]
struct Pointer {
	archetype_id: u64,
	row_index: usize
}

impl World {

	

	pub fn init() -> Self {
		let mut world = World {
			entity_count: 0,
			entities: HashMap::new(),
			archetypes: HashMap::new()
		};

		world.archetypes.insert(VOID_ARCHETYPE, ArchetypeStorage {
			hash: VOID_ARCHETYPE,
			components: HashMap::new(),
    		entity_ids: Vec::new(),
		});
		world
	}

	pub fn new_entity(&mut self) -> EntityId {
		let new_id = self.entity_count;
		self.entity_count += 1;
		let void_archetype = self.archetypes.get_mut(&VOID_ARCHETYPE).expect("Void archetype was not initialized!");
		let new_row = void_archetype.new_row(new_id);
		let void_pointer = Pointer {
			archetype_id: VOID_ARCHETYPE,
			row_index: new_row
		};
		self.entities.insert(new_id, void_pointer);
		new_id
	}

	fn archetype_id_from_entity(&self, id: EntityId) -> Option<&Pointer> {
		self.entities.get(&id)
	}
	pub fn set_component<T: Component>(&mut self, id: EntityId, name: &str, component: T) -> Result<Option<T>, String> {
		let archetype_id = &self.archetype_id_from_entity(id).ok_or("entity does not exist")?.archetype_id;
		let archetype = self.archetypes.get(archetype_id).ok_or("entity has no archetype")?;
		let old_hash = archetype.hash;
		let has_already = archetype.components.contains_key(name);
		let new_hash = {
			if has_already {
				old_hash
			} else {
				let mut hasher = DefaultHasher::new();
				name.hash(&mut hasher);
				old_hash ^ hasher.finish()
			}
		};
		let mut current_archetype_storage;
		if has_already {
			let ptr = self.entities.get(&id).ok_or("entity does not exist")?;
			current_archetype_storage = self.archetypes.get_mut(&new_hash).unwrap();
			current_archetype_storage.set(ptr.row_index, name, component)
		} else if self.archetypes.contains_key(&new_hash) {
			let new_row = self.archetypes.get_mut(&new_hash).unwrap().new_row(id);
			let old_ptr = self.entities.get(&id).ok_or("entity does not exist")?.clone();
			let column_iter = archetype.components.iter();
			for entry in column_iter {
				let old_component_storage = entry.1;
				let mut new_component_storage = self.archetypes.get_mut(&new_hash).unwrap().components.get_mut(entry.0).expect("New component storage was initialized incorrectly");
				let old_component_storage = storage_cast_mut::<T::ComponentStorage>(old_component_storage.as_mut()).unwrap();
				old_component_storage.move_to::<T::ComponentStorage, T>(
					old_ptr.row_index,
					storage_cast_mut::<T::ComponentStorage>(new_component_storage.as_mut()).unwrap(),
					new_row);
			}
			current_archetype_storage = self.archetypes.get_mut(&new_hash).unwrap();
			current_archetype_storage.entity_ids[new_row] = id;
			current_archetype_storage.set(new_row, name, component)
		} else {
			let mut new_row;
			current_archetype_storage = {
				let mut new_archetype = ArchetypeStorage { 
					hash: new_hash, 
					components: HashMap::new(), 
					entity_ids: Vec::new()
				};
				new_row = new_archetype.new_row(id);
				let old_ptr = self.entities.get(&id).ok_or("entity does not exist")?.clone();
				let column_iter = archetype.components.iter();
				for entry in column_iter {
					new_archetype.components.insert(entry.0.to_owned(), entry.1.clone_type());
					let old_component_storage = entry.1;
					let mut new_component_storage = new_archetype.components.get_mut(entry.0).expect("New component storage was initialized incorrectly");
					let old_component_storage = storage_cast_mut::<T::ComponentStorage>(old_component_storage.as_mut()).unwrap();
					old_component_storage.move_to::<T::ComponentStorage, T>(
						old_ptr.row_index,
						storage_cast_mut::<T::ComponentStorage>(new_component_storage.as_mut()).unwrap(),
						new_row);
				}
				let new_component_storage = T::ComponentStorage::init();
				self.archetypes.insert(new_hash, new_archetype);
				self.archetypes.get_mut(&new_hash).unwrap()
				};
			
			current_archetype_storage.set(new_row, name, component)
		}
	}
}
struct ArchetypeStorage {
	hash: u64,
	components: HashMap<String, Box<dyn Storage>>,
	entity_ids: Vec<EntityId>
}

impl ArchetypeStorage {
	pub fn new_row(&mut self, entity_id: EntityId) -> usize {
		let new_row_index = self.entity_ids.len();
		self.entity_ids.push(entity_id);
		new_row_index
	}
	pub fn remove(&mut self, row_index: usize) {
		self.entity_ids.swap_remove(row_index);
		for storage in self.components.values_mut() {
			storage.remove(row_index)
		}
	}
	pub fn set<T: Component>(&mut self, row_index: usize, name: &str, component: T) -> Result<Option<T>, String> {
		let mut component_storage_erased = self.components.get_mut(name).ok_or(String::from("Invalid component name given: ")+ name)?;
		let mut component_storage: &mut T::ComponentStorage = component_storage_erased.as_any().downcast_mut().ok_or(format!("{name} is not of type {}", type_name::<T>()))?;
		component_storage.set(row_index, component)
	}
}