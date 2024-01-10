
use std::{collections::{HashMap, hash_map::DefaultHasher, HashSet}, hash::{Hash, Hasher}, mem::{swap, replace}, cell::RefCell};
use any_vec::AnyVec;
use serde::Serializer;

use self::components::{ComponentTypeId, Component};

pub mod components;
mod systems;
pub const VOID_ARCHETYPE: u64 = u64::MAX;
type EntityId = u32;
pub struct World {
	archetypes: HashMap<u64, RefCell<Archetype>>,
	archetype_sets: HashMap<ComponentTypeId, HashSet<u64>>,
	entities: HashMap<EntityId, EntityPointer>,
	entity_count: u32,
}

impl World {

	

	pub fn init() -> Self {
		let mut world = World {
			entities: HashMap::new(),
			archetypes: HashMap::new(),
			archetype_sets: HashMap::new(),
			entity_count: 0
		};

		world.archetypes.insert(VOID_ARCHETYPE, RefCell::new(Archetype {
					components: HashMap::new(),
					entity_ids: Vec::new(),
				}));
		world
	}

	pub fn new_entity(&mut self) -> EntityId {
		let new_id = self.entity_count;
		self.entity_count += 1;
		let mut void_archetype = self.archetypes.get_mut(&VOID_ARCHETYPE).expect("Void archetype was not initialized!").borrow_mut();
		let new_row = void_archetype.new_row(new_id);
		let void_pointer = EntityPointer {
			archetype_id: VOID_ARCHETYPE,
			index: new_row
		};
		self.entities.insert(new_id, void_pointer);
		new_id
	}
	pub fn clone_component<T: Component + Clone>(&self, id: EntityId) -> Result<Option<T>, String> {
		let pointer = self.archetype_id_from_entity(id).ok_or("entity does not exist")?;
		let archetype = self.archetypes.get(&pointer.archetype_id).ok_or("entity has no archetype")?.borrow();
		let val = archetype.components.get(&ComponentTypeId::of::<T>()).and_then(|v| v.downcast_ref::<T>()?.get(pointer.index)).and_then(|val| Some(val.clone()));

		Ok(val)
	}
	fn archetype_id_from_entity(&self, id: EntityId) -> Option<&EntityPointer> {
		self.entities.get(&id)
	}
	pub fn set_component<T: Component>(&mut self, id: EntityId, component: T) -> Result<Option<T>, String> {
		let name = ComponentTypeId::of::<T>();
		let archetype_id = self.archetype_id_from_entity(id).ok_or("entity does not exist")?.archetype_id;
		let archetype = self.archetypes.get(&archetype_id).ok_or("entity has no archetype")?.borrow();
		let old_hash = archetype_id;
		let has_already = archetype.components.contains_key(&name);
		let new_hash = {
			if has_already {
				old_hash
			} else {
				let mut hasher = DefaultHasher::new();
				name.hash(&mut hasher);
				old_hash ^ hasher.finish()
			}
		};
		drop(archetype);
		let archetype_already_exists = self.archetypes.contains_key(&new_hash);
		let mut archetype = self.archetypes.get(&archetype_id).ok_or("entity has no archetype")?.borrow_mut();
		let mut current_archetype_storage;
		if has_already {
			let ptr = self.entities.get(&id).ok_or("entity does not exist")?;
			current_archetype_storage = archetype;
			current_archetype_storage.set::<T>(ptr.index, component).map(|val| Some(val))
		} else if archetype_already_exists {
			let new_row = self.archetypes.get(&new_hash).unwrap().borrow_mut().new_row(id);
			let old_ptr = self.entities.get(&id).ok_or("entity does not exist")?.to_owned();
			

			let swapped_entity = *archetype.entity_ids.last().unwrap();
			archetype.entity_ids.swap_remove(old_ptr.index);
			self.entities.get_mut(&swapped_entity).unwrap().index = old_ptr.index;

			let column_iter = archetype.components.iter_mut();
			let mut new_archetype = self.archetypes.get(&new_hash).unwrap().borrow_mut();
			for entry in column_iter {
				let old_component_storage = entry.1;
				
				let mut new_component_storage = new_archetype.components.get_mut(entry.0).expect("New component storage was initialized incorrectly");
		


				let value = old_component_storage.swap_remove(old_ptr.index);
				
				new_component_storage.push(value)
			}
			current_archetype_storage = self.archetypes.get(&new_hash).unwrap().borrow_mut();
			current_archetype_storage.entity_ids[new_row] = id;
			current_archetype_storage.push(component).map(|_| None)
		} else {
			let mut new_row;
			current_archetype_storage = {
				let mut new_archetype = Archetype { 
					components: HashMap::new(), 
					entity_ids: Vec::new()
				};
				new_row = new_archetype.new_row(id);
				let old_ptr = self.entities.get(&id).ok_or("entity does not exist")?.clone();
				

				let swapped_entity = *archetype.entity_ids.last().unwrap();
				archetype.entity_ids.swap_remove(old_ptr.index);
				self.entities.get_mut(&swapped_entity).unwrap().index = old_ptr.index;
				
				let column_iter = archetype.components.iter_mut();
				for entry in column_iter {
					let old_component_storage = entry.1;

					let mut new_component_storage = old_component_storage.clone_empty();
	
					let value = old_component_storage.swap_remove(old_ptr.index);

					
					new_component_storage.push(value);
					
					new_archetype.components.insert(entry.0.clone(), new_component_storage);
					self.archetype_sets.get_mut(entry.0).expect("previous archetype was not in correct sets").insert(new_hash);
				}
				new_archetype.components.insert(name, AnyVec::new::<T>());

				new_archetype.push(component);
				let new_component_set = {
					match self.archetype_sets.get_mut(&name) {
						Some(val) => {
							val
						},
						None => {
							self.archetype_sets.insert(name, HashSet::new());
							self.archetype_sets.get_mut(&name).unwrap()
						}
					}
				};
				new_component_set.insert(new_hash);
				drop(archetype);

				self.archetypes.insert(new_hash, RefCell::new(new_archetype));
				
				self.archetypes.get_mut(&new_hash).unwrap().borrow_mut()
				};
				Ok(None)


			

		}
	}
}
#[derive(Clone)]
struct EntityPointer {
	archetype_id: u64,
	index: usize
}
struct Archetype {
	components: HashMap<ComponentTypeId, AnyVec>,
	entity_ids: Vec<EntityId>
}

impl Archetype {
	pub fn new_row(&mut self, entity_id: EntityId) -> usize {
		let new_row_index = self.entity_ids.len();
		self.entity_ids.push(entity_id);
		new_row_index
	}

	/* Caller must fix entity pointers. Returned id is the entity id of the swapped entity.
	 * New row for the swapped entity is the row_index
	 */
	pub fn swap_remove(&mut self, row_index: usize) -> EntityId {

		let swapped = *self.entity_ids.last().unwrap();
		self.entity_ids.swap_remove(row_index);
		for storage in self.components.values_mut() {
			storage.swap_remove(row_index);
		}
		swapped
	}
	pub fn set<T: Component>(&mut self, row_index: usize, component: T) -> Result<T, String> {
		let target_pointer: &mut T = self.components.get_mut(&ComponentTypeId::of::<T>()).ok_or("Set called with wrong component type for this archetype")?
		.get_mut(row_index).ok_or("Row index out of bounds")?.downcast_mut().ok_or(format!("AnyVec in wrong row, should be of type {:?}", ComponentTypeId::of::<T>()))?;
		
		let old_val = replace(target_pointer, component);

		Ok(old_val)

	}

	pub fn push<T: Component>(&mut self, component: T) -> Result<(), String> {
		self.components.get_mut(&ComponentTypeId::of::<T>()).ok_or("Set called with wrong component type for this archetype")?
		.downcast_mut().ok_or(format!("AnyVec in wrong row, should be of type {:?}", ComponentTypeId::of::<T>()))?.push(component);

		Ok(())
		


	}
}