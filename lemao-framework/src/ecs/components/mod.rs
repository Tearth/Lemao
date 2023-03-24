use std::{any::TypeId, collections::HashMap};

use self::list::{ComponentList, ComponentListTrait};

pub mod list;
pub mod manager;

pub trait Component {
    fn get_entity(&self) -> usize;
    fn set_entity_id(&mut self, entity_id: usize);
}

pub trait ComponentManagerHashMap {
    fn get_component_managers_1<C1>(&mut self) -> &mut ComponentList<C1>
    where
        C1: Component + 'static;

    fn get_component_managers_2<C1, C2>(&mut self) -> (&mut ComponentList<C1>, &mut ComponentList<C2>)
    where
        C1: Component + 'static,
        C2: Component + 'static;

    fn get_component_managers_3<C1, C2, C3>(&mut self) -> (&mut ComponentList<C1>, &mut ComponentList<C2>, &mut ComponentList<C3>)
    where
        C1: Component + 'static,
        C2: Component + 'static,
        C3: Component + 'static;

    fn get_component_managers_4<C1, C2, C3, C4>(&mut self) -> (&mut ComponentList<C1>, &mut ComponentList<C2>, &mut ComponentList<C3>, &mut ComponentList<C4>)
    where
        C1: Component + 'static,
        C2: Component + 'static,
        C3: Component + 'static,
        C4: Component + 'static;
}

impl ComponentManagerHashMap for HashMap<TypeId, Box<dyn ComponentListTrait>> {
    fn get_component_managers_1<C1>(&mut self) -> &mut ComponentList<C1>
    where
        C1: Component + 'static,
    {
        let a = self.get_mut(&TypeId::of::<C1>()).unwrap();
        a.as_any_mut().downcast_mut::<ComponentList<C1>>().unwrap()
    }

    fn get_component_managers_2<C1, C2>(&mut self) -> (&mut ComponentList<C1>, &mut ComponentList<C2>)
    where
        C1: Component + 'static,
        C2: Component + 'static,
    {
        let [a, b] = self.get_many_mut([&TypeId::of::<C1>(), &TypeId::of::<C2>()]).unwrap();
        (a.as_any_mut().downcast_mut::<ComponentList<C1>>().unwrap(), b.as_any_mut().downcast_mut::<ComponentList<C2>>().unwrap())
    }

    fn get_component_managers_3<C1, C2, C3>(&mut self) -> (&mut ComponentList<C1>, &mut ComponentList<C2>, &mut ComponentList<C3>)
    where
        C1: Component + 'static,
        C2: Component + 'static,
        C3: Component + 'static,
    {
        let [a, b, c] = self.get_many_mut([&TypeId::of::<C1>(), &TypeId::of::<C2>(), &TypeId::of::<C3>()]).unwrap();
        (
            a.as_any_mut().downcast_mut::<ComponentList<C1>>().unwrap(),
            b.as_any_mut().downcast_mut::<ComponentList<C2>>().unwrap(),
            c.as_any_mut().downcast_mut::<ComponentList<C3>>().unwrap(),
        )
    }

    fn get_component_managers_4<C1, C2, C3, C4>(&mut self) -> (&mut ComponentList<C1>, &mut ComponentList<C2>, &mut ComponentList<C3>, &mut ComponentList<C4>)
    where
        C1: Component + 'static,
        C2: Component + 'static,
        C3: Component + 'static,
        C4: Component + 'static,
    {
        let [a, b, c, d] = self.get_many_mut([&TypeId::of::<C1>(), &TypeId::of::<C2>(), &TypeId::of::<C3>(), &TypeId::of::<C4>()]).unwrap();
        (
            a.as_any_mut().downcast_mut::<ComponentList<C1>>().unwrap(),
            b.as_any_mut().downcast_mut::<ComponentList<C2>>().unwrap(),
            c.as_any_mut().downcast_mut::<ComponentList<C3>>().unwrap(),
            d.as_any_mut().downcast_mut::<ComponentList<C4>>().unwrap(),
        )
    }
}
