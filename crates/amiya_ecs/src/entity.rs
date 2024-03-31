use std::{any::TypeId, collections::HashSet};

use super::component::Component;

pub struct Entity{
    id : usize,
    component_map : HashSet<TypeId>,
}

impl Entity{
    pub fn new(id : usize)->Self{
        Entity{
            id,
            component_map : HashSet::new()
        }
    }

    pub fn id(self : &Self)->usize{
        self.id
    }

    pub fn have<Com : Component>(self : &Self)->bool{
        self.component_map.contains(&TypeId::of::<Com>())
    }

    // pub(crate) fn add_component<Com : Component>(self : &mut Self){
    //     self.component_map.insert(TypeId::of::<Com>());
    // }

    // pub(crate) fn remove_component<Com : Component>(self : &mut Self){
    //     self.component_map.remove(&TypeId::of::<Com>());
    // }
}

mod test{
    // use super::{super::component::Component, Entity};

    #[derive(Default)]
    struct A;
    // impl Component for A {
    //     fn new()->Box<Self> where Self:Sized {
    //         Box::new(A::default())
    //     }
    // }

    #[derive(Default)]
    struct B;
    // impl Component for B {}

    #[derive(Default)]
    struct C;
    // impl Component for C {}

    #[derive(Default)]
    struct D;
    // impl Component for D {}

    #[test]
    fn test(){
        // let mut entity_0 = Entity::new(0);

        // assert_eq!(entity_0.have::<A>(), false);
        // assert_eq!(entity_0.have::<B>(), false);
        // assert_eq!(entity_0.have::<C>(), false);
        // assert_eq!(entity_0.have::<D>(), false);

        // entity_0.add_component::<A>();
        // entity_0.add_component::<B>();
        // entity_0.add_component::<C>();

        // assert_eq!(entity_0.have::<A>(), true);
        // assert_eq!(entity_0.have::<B>(), true);
        // assert_eq!(entity_0.have::<C>(), true);
        // assert_eq!(entity_0.have::<D>(), false);

        // entity_0.remove_component::<B>();

        // assert_eq!(entity_0.have::<A>(), true);
        // assert_eq!(entity_0.have::<B>(), false);
        // assert_eq!(entity_0.have::<C>(), true);
        // assert_eq!(entity_0.have::<D>(), false);
    }
}