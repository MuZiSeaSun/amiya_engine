use std::{any::{Any, TypeId}, collections::HashMap};

use super::{component::{Component, Components}, entity::Entity, sparse_set::SparseSet};

pub struct World{
    entity_map : SparseSet<Entity>,
    // components_map : HashMap<TypeId, Box<dyn Storage<dyn Component>>>
    components_map : HashMap<TypeId, Box<dyn Any>>
}

impl World {
    pub fn new()->Self{
        World{
            entity_map : SparseSet::new(),
            components_map : HashMap::new()
        }
    }

    pub fn spaw_entity(&mut self)->usize{
        let id = self.entity_map.get_min_none();
        self.entity_map.inster(id, Box::new(Entity::new(id)));
        // Rc::new(self.entity_map.get(id).unwrap())
        id
    }

    pub fn add_component<Com : Component>(&mut self, entity_id : usize){
        let type_id = TypeId::of::<Com>();
        match self.components_map.contains_key(&type_id) {
            true=>{
                let storage = self.components_map.get_mut(&type_id).unwrap().downcast_mut::<Components<Com>>().unwrap();
                storage.add_component(entity_id);
            },
            false=>{
                let mut storage = Components::<Com>::new();
                storage.add_component(entity_id);
                self.components_map.insert(type_id, Box::new(storage));
            }
        }
    }

    pub fn add_component_with<Com : Component>(&mut self, entity_id : usize, com : Com){
        let type_id = TypeId::of::<Com>();
        match self.components_map.contains_key(&type_id) {
            true=>{
                let storage = self.components_map.get_mut(&type_id).unwrap().downcast_mut::<Components<Com>>().unwrap();
                storage.add_component_with(entity_id, Box::new(com));
            },
            false=>{
                let mut storage = Components::<Com>::new();
                storage.add_component_with(entity_id, Box::new(com));
                self.components_map.insert(type_id, Box::new(storage));
            }
        }
    }

    pub fn get_components<Com : Component>(&self)->Option<&[Box<Com>]>{
        let type_id = TypeId::of::<Com>();
        match self.components_map.contains_key(&type_id){
            true=>{
                let storage = self.components_map.get(&type_id).unwrap().downcast_ref::<Components<Com>>().unwrap();
                Some(storage.get_all())
            },
            false=>{
                None
            }
        }
    }

    pub fn get_component<Com : Component>(&self, entity_id : usize)->Option<&Com>{
        let type_id = TypeId::of::<Com>();
        match self.components_map.contains_key(&type_id){
            true=>{
                let storage = self.components_map.get(&type_id).unwrap().downcast_ref::<Components<Com>>().unwrap();
                storage.get(entity_id)
            },
            false=>{
                None
            }
        }
    }

    pub fn get_components_mut<Com : Component>(&mut self)->Option<&mut [Box<Com>]>{
        let type_id = TypeId::of::<Com>();
        match self.components_map.contains_key(&type_id){
            true=>{
                let storage = self.components_map.get_mut(&type_id).unwrap().downcast_mut::<Components<Com>>().unwrap();
                Some(storage.get_all_mut())
            },
            false=>{
                None
            }
        }
    }

    pub fn get_component_mut<Com : Component>(&mut self, entity_id : usize)->Option<&mut Com>{
        let type_id = TypeId::of::<Com>();
        match self.components_map.contains_key(&type_id){
            true=>{
                let storage = self.components_map.get_mut(&type_id).unwrap().downcast_mut::<Components<Com>>().unwrap();
                storage.get_mut(entity_id)
            },
            false=>{
                None
            }
        }
    }

    pub fn remove_component<Com : Component>(&mut self, entity_id : usize){
        let type_id = TypeId::of::<Com>();
        match self.components_map.contains_key(&type_id){
            true=>{
                let storage = self.components_map.get_mut(&type_id).unwrap().downcast_mut::<Components<Com>>().unwrap();
                storage.remove_component(entity_id)
            },
            false=>()
        }
    }

    pub fn remove_components<Com : Component>(&mut self, entity_ids : &[usize]){
        let type_id = TypeId::of::<Com>();
        match self.components_map.contains_key(&type_id){
            true=>{
                let storage = self.components_map.get_mut(&type_id).unwrap().downcast_mut::<Components<Com>>().unwrap();
                for entity_id in entity_ids{
                    storage.remove_component(*entity_id)
                }
            },
            false=>()
        }
    }
}

mod test{
    
    pub struct Name{
        pub name : String
    }
    
    impl Default for Name {
        fn default() -> Self {
            Self { name: "none".to_string() }
        }
    }
    
    pub struct Age{
        pub age : u32
    }
    
    impl Default for Age {
        fn default() -> Self {
            Self { age : 233 }
        }
    }
    
    #[test]
    fn test_all(){
        use crate::world::World;
        
        let mut world = World::new();

        let e1 = world.spaw_entity();
        let e2 = world.spaw_entity();
        let e3 = world.spaw_entity();
        let e4 = world.spaw_entity();
        world.add_component_with::<Name>(e1, Name{name : "1".to_string()});
        world.add_component_with::<Name>(e2, Name{name : "2".to_string()});
        world.add_component_with::<Name>(e3, Name{name : "3".to_string()});
        world.add_component_with::<Name>(e4, Name{name : "4".to_string()});

        for c in world.get_components::<Name>().unwrap(){
            println!("name:{}", c.name)
        }

        println!("==========================");
        
        for c in world.get_components_mut::<Name>().unwrap(){
            c.name += "a";
        }
        
        for c in world.get_components::<Name>().unwrap(){
            println!("name:{}", c.name)
        }
        
        println!("==========================");
        
        world.remove_component::<Name>(e1);
        for c in world.get_components::<Name>().unwrap(){
            println!("name:{}", c.name)
        }

        println!("==========================");
        
        world.add_component::<Name>(e1);
        for c in world.get_components::<Name>().unwrap(){
            println!("name:{}", c.name)
        }
        
        println!("==========================");
    }
}
