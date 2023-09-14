use std::{any::{Any, type_name}, rc::Rc, fmt::format, collections::HashMap, borrow::Borrow};

use crate::maths::vector::Vector;

pub mod position;


struct Transform {
	pos: Vector,
	rot: f32,
	scale: Vector
}
pub fn storage_cast<T: Storage>(storage: &dyn Storage) -> Result<&T, String> {
	storage.as_any().downcast_ref().ok_or(format!("storage_cast was called with an incorrect target type!"))

}
pub fn storage_cast_mut<T: Storage>(storage: &mut dyn Storage) -> Result<&mut T, String> {
	storage.as_any().downcast_mut().ok_or(format!("storage_cast was called with an incorrect target type!"))

}
pub trait Component: Sized + Send + Sync {
	type ComponentStorage: Storage;
	fn as_any(&self) -> &dyn Any;
}
impl<T: Sized + Send + Sync> Component for T {
    type ComponentStorage = HashMap<usize, Self>;
	fn as_any(&self) -> &dyn Any {
		self
	}
}

pub trait Storage {
	fn init() -> Self where Self: Sized;
	fn remove(&mut self, row: usize);
	fn clone_type(&self) -> Box<dyn Storage>;
	fn as_any(&self) -> &dyn Any;
	fn get<T: Component>(&mut self, row_index: usize) -> Result<Option<&T>, String> where Self: Sized;
	fn set<T: Component>(&mut self, row_index: usize, component: T) -> Result<Option<T>, String> where Self: Sized;
	fn move_to<T: Storage, U: Component>(&mut self, src_row: usize, dst: &mut T, dst_row: usize) -> Result<Option<U>, String> where Self: Sized;
}
impl<I: Component> Storage for HashMap<usize, I> {
	fn init() -> Self where Self: Sized {
		HashMap::new()
	}
	fn remove(&mut self, row: usize) {
		self.remove(&row);
	}
	fn clone_type(&self) -> Box<dyn Storage> {
		Box::new(Self::new())
	}
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn get<T: Component>(&mut self, row_index: usize) -> Result<Option<&T>, String> where Self: Sized {
		Ok(Some(self.get::<I>(row_index).as_any().downcast_ref().ok_or(format!("Storage::set was called with an incorrect component type! Expected {} but got {}!", type_name::<I>(), type_name::<T>()))?))
	}
	fn set<T: Component>(&mut self, row_index: usize, component: T) -> Result<Option<T>, String> where Self: Sized {

		let component_as_item: I = *component.as_any().downcast_ref().ok_or(format!("Storage::set was called with an incorrect component type! Expected {} but got {}!", type_name::<I>(), type_name::<T>()))?;
        Ok(self.insert(row_index, component_as_item).map(|val|*val.as_any().downcast_ref().expect("Type was already checked? How did this fail?")))
    }

	fn move_to<T: Storage, U: Component>(&mut self, src_row: usize, dst: &mut T, dst_row: usize) -> Result<Option<U>, String> {
		let val = self.remove(&src_row).ok_or("Storage::copy_to was told to copy an empty row!")?;
        dst.set(dst_row, *val.as_any().downcast_ref().ok_or("Storage::move_to was called with incompatible types!")?)
    }
}