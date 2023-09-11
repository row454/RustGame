use core::arch;
use std::{collections::{HashMap, hash_map::DefaultHasher}, sync::Arc, hash::{Hash, Hasher}, f32::consts::E, any::{Any, type_name}};

use self::components::{Storage, Component};

mod archetype;
mod components;
type EntityId = u32;
pub const VOID_ARCHETYPE: u64 = u64::MAX;


struct World {
	entity_count: u32,
	entities: HashMap<EntityId, Pointer>,
	archetypes: HashMap<u64, ArchetypeStorage>
}
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

	fn archetype_from_entity(&mut self, id: EntityId) -> Option<&mut ArchetypeStorage>{
		self.entities.get(&id).map(|ptr| self.archetypes.get_mut(&ptr.archetype_id))?
	}
	pub fn set_component<T: Component>(&mut self, id: EntityId, name: &str, value: T) -> Result<(), String> {
		let mut archetype = self.archetype_from_entity(id).ok_or("entity does not exist")?;
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
		let mut current_archetype_storage = self.archetypes.entry(new_hash).or_insert({
			let mut new_archetype = ArchetypeStorage { 
				hash: new_hash, 
				components: HashMap::new(), 
				entity_ids: Vec::new()
			};
			let mut column_iter = archetype.components.iter();
			for entry in column_iter {
				new_archetype.components.insert(entry.0.to_owned(), entry.1.clone_type());
			}
			let new_component_storage = T::ComponentStorage::init();
			new_archetype
		});
		
		if has_already {
			let ptr = self.entities.get(&id).ok_or("entity does not exist")?;
			current_archetype_storage.set(ptr.row_index, name, value)

		} else {
			let new_row = current_archetype_storage.new_row(id);
			let old_ptr = self.entities.get(id);
			let mut column_iter = archetype.components.iter();
			for entry in column_iter {
				let old_component_storage = entry.1;
				let mut new_component_storage = current_archetype_storage.components.get_mut(entry.0);
				new_component_storage.
			}
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
	pub fn set<T: Component>(&mut self, row_index: usize, name: &str, component: T) -> Result<(), String> {
		let mut component_storage_erased = self.components.get_mut(name).ok_or(String::from("Invalid component name given: ")+ name)?;
		let mut component_storage: &mut T::ComponentStorage = component_storage_erased.as_any().downcast_mut().ok_or(format!("{name} is not of type {}", type_name::<T>()))?;
		component_storage.set(row_index, component)
	}
}