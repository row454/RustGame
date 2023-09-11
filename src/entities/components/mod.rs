use std::{any::{Any, type_name}, rc::Rc, fmt::format, collections::HashMap};

use crate::maths::vector::Vector;

pub mod position;


struct Transform {
	pos: Vector,
	rot: f32,
	scale: Vector
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
	fn copy_to(&self, src_row: usize, dst: Box<dyn Storage>, dst_row: usize);
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

	fn copy_to(&self, src_row: usize, dst: Box<dyn Storage>, dst_row: usize) {
        dst.set(dst_row, self.get(&src_row))
    }
}