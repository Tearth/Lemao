use super::list::ComponentList;
use super::list::ComponentListTrait;
use super::Component;
use std::any::TypeId;
use std::collections::hash_map::Values;
use std::collections::hash_map::ValuesMut;
use std::collections::HashMap;

type ComponentTuple2<'a, C1, C2> = (&'a mut ComponentList<C1>, &'a mut ComponentList<C2>);
type ComponentTuple3<'a, C1, C2, C3> = (&'a mut ComponentList<C1>, &'a mut ComponentList<C2>, &'a mut ComponentList<C3>);
type ComponentTuple4<'a, C1, C2, C3, C4> = (&'a mut ComponentList<C1>, &'a mut ComponentList<C2>, &'a mut ComponentList<C3>, &'a mut ComponentList<C4>);

#[derive(Default)]
pub struct ComponentManager {
    data: HashMap<TypeId, Box<dyn ComponentListTrait>>,
}

impl ComponentManager {
    pub fn store<C>(&mut self, item: Box<dyn ComponentListTrait>) -> Result<(), String>
    where
        C: 'static,
    {
        if self.data.contains_key(&TypeId::of::<C>()) {
            return Err("Storage item already exists".to_string());
        }

        self.data.insert(TypeId::of::<C>(), item);
        Ok(())
    }

    pub fn contains<C>(&self) -> bool
    where
        C: 'static,
    {
        self.data.contains_key(&TypeId::of::<C>())
    }

    pub fn get<C>(&self) -> Result<&dyn ComponentListTrait, String>
    where
        C: 'static,
    {
        match self.data.get(&TypeId::of::<C>()) {
            Some(item) => Ok(item.as_ref()),
            _ => Err(format!("Component list {} not found", 0)),
        }
    }

    pub fn get_and_cast<C>(&self) -> Result<&ComponentList<C>, String>
    where
        C: Component + 'static,
    {
        match self.data.get(&TypeId::of::<C>()) {
            Some(item) => Ok(item.as_any().downcast_ref::<ComponentList<C>>().unwrap()),
            _ => Err(format!("Component list {} not found", 0)),
        }
    }

    pub fn get_mut<C>(&mut self) -> Result<&mut Box<dyn ComponentListTrait>, String>
    where
        C: 'static,
    {
        match self.data.get_mut(&TypeId::of::<C>()) {
            Some(item) => Ok(item),
            _ => Err(format!("Component list {} not found", 0)),
        }
    }

    pub fn get_and_cast_mut<C>(&mut self) -> Result<&mut ComponentList<C>, String>
    where
        C: Component + 'static,
    {
        match self.data.get_mut(&TypeId::of::<C>()) {
            Some(item) => Ok(item.as_any_mut().downcast_mut::<ComponentList<C>>().unwrap()),
            _ => Err(format!("Component list {} not found", 0)),
        }
    }

    pub fn get_and_cast_mut_2<C1, C2>(&mut self) -> Result<ComponentTuple2<C1, C2>, String>
    where
        C1: Component + 'static,
        C2: Component + 'static,
    {
        unsafe {
            let a = self.get_mut::<C1>()? as *mut _ as *mut Box<dyn ComponentListTrait>;
            let b = self.get_mut::<C2>()? as *mut _ as *mut Box<dyn ComponentListTrait>;

            Ok(((*a).as_any_mut().downcast_mut::<ComponentList<C1>>().unwrap(), (*b).as_any_mut().downcast_mut::<ComponentList<C2>>().unwrap()))
        }
    }

    pub fn get_and_cast_mut_3<C1, C2, C3>(&mut self) -> Result<ComponentTuple3<C1, C2, C3>, String>
    where
        C1: Component + 'static,
        C2: Component + 'static,
        C3: Component + 'static,
    {
        unsafe {
            let a = self.get_mut::<C1>()? as *mut _ as *mut Box<dyn ComponentListTrait>;
            let b = self.get_mut::<C2>()? as *mut _ as *mut Box<dyn ComponentListTrait>;
            let c = self.get_mut::<C3>()? as *mut _ as *mut Box<dyn ComponentListTrait>;

            Ok((
                (*a).as_any_mut().downcast_mut::<ComponentList<C1>>().unwrap(),
                (*b).as_any_mut().downcast_mut::<ComponentList<C2>>().unwrap(),
                (*c).as_any_mut().downcast_mut::<ComponentList<C3>>().unwrap(),
            ))
        }
    }

    pub fn get_and_cast_mut_4<C1, C2, C3, C4>(&mut self) -> Result<ComponentTuple4<C1, C2, C3, C4>, String>
    where
        C1: Component + 'static,
        C2: Component + 'static,
        C3: Component + 'static,
        C4: Component + 'static,
    {
        unsafe {
            let a = self.get_mut::<C1>()? as *mut _ as *mut Box<dyn ComponentListTrait>;
            let b = self.get_mut::<C2>()? as *mut _ as *mut Box<dyn ComponentListTrait>;
            let c = self.get_mut::<C3>()? as *mut _ as *mut Box<dyn ComponentListTrait>;
            let d = self.get_mut::<C4>()? as *mut _ as *mut Box<dyn ComponentListTrait>;

            Ok((
                (*a).as_any_mut().downcast_mut::<ComponentList<C1>>().unwrap(),
                (*b).as_any_mut().downcast_mut::<ComponentList<C2>>().unwrap(),
                (*c).as_any_mut().downcast_mut::<ComponentList<C3>>().unwrap(),
                (*d).as_any_mut().downcast_mut::<ComponentList<C4>>().unwrap(),
            ))
        }
    }

    pub fn iter(&self) -> Values<TypeId, Box<dyn ComponentListTrait>> {
        self.data.values()
    }

    pub fn iter_mut(&mut self) -> ValuesMut<TypeId, Box<dyn ComponentListTrait>> {
        self.data.values_mut()
    }
}
