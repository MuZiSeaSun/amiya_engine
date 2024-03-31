use std::{any::Any, default::Default};

use super::sparse_set::SparseSet;

pub trait Component : Any + 'static {
    fn new()->Box<Self> where Self:Sized;
    fn init(&mut self){}
}

pub trait Init {
    
}

pub trait End {
    
}

impl<T : Default + 'static> Component for T {
    fn new()->Box<Self> where Self:Sized {
        Box::new(T::default())
    }
}

pub trait Pool<Item> {
    fn pool(self : &mut Self) -> &mut Vec<Box<Item>>;

    fn spaw(self : &mut Self)->Box<Item>{
        match self.pool().len()>0 {
            true =>{
                self.pool().pop().unwrap()
            },
            false =>{
                Self::new_com()
            }
        }
    }

    fn recycle(self : &mut Self, com : Box<Item>){
        self.pool().push(com)
    }

    fn new_com()->Box<Item>;
}

pub trait Storage<Com : Component> : Any + 'static {
    fn get(&self)-> &[Box<Com>];
    fn get_mut(&mut self)-> &mut [Box<Com>];
}

pub struct ComponentPool<Com : Component + ?Sized>{
    pool : Vec<Box<Com>>
}

impl<Com : Component> ComponentPool<Com> {
    pub fn new()->Self{
        ComponentPool{
            pool : Vec::new()
        }
    }
}

impl<Com : Component> Pool<Com> for ComponentPool<Com> {
    fn pool(self : &mut Self) -> &mut Vec<Box<Com>> {
        &mut self.pool
    }
    
    fn new_com()->Box<Com> {
        Com::new()
    }
}



pub struct Components<Com : Component + ?Sized>{
    // id : TypeId,
    components : SparseSet<Com>,
    pool : ComponentPool<Com>
}

impl<Com : Component + Sized + 'static> Components<Com> {
    #[inline]
    pub fn new()->Self{
        Components{
            // id : TypeId::of::<Com>(),
            components : SparseSet::new(),
            pool : ComponentPool::new()
        }
    }

    pub fn add_component(self : &mut Self, entity_id : usize){
        match self.components.inster(entity_id, self.pool.spaw()){
            None=>(),
            Some(value)=>{
                self.pool.recycle(value)
            }
        }
    }

    pub fn add_component_with(self : &mut Self, entity_id : usize, com : Box<Com>){
        match self.components.inster(entity_id, com){
            None=>(),
            Some(value)=>{
                self.pool.recycle(value)
            }
        }
    }

    pub fn remove_component(self : &mut Self, entity_id : usize){
        match self.components.remove(entity_id) {
            Some(value)=>{
                self.pool.recycle(value);
            },
            None=>()
        }
        
    }

    #[inline]
    pub fn have_component(&self, entity_id : usize)->bool{
        self.components.contine(entity_id)
    }

    #[inline]
    pub fn get(&self, entity_id : usize)->Option<&Com>{
        self.components.get(entity_id)
    }

    #[inline]
    pub fn get_mut(&mut self, entity_id : usize)->Option<&mut Com>{
        self.components.get_mut(entity_id)
    }

    #[inline]
    pub fn get_all(&self)-> &[Box<Com>]{
        self.components.get_all()
    }

    #[inline]
    pub fn get_all_mut(&mut self)-> &mut [Box<Com>]{
        self.components.get_all_mut()
    }
}

impl<Com : Component> Storage<Com> for Components<Com> {
    fn get(&self)-> &[Box<Com>] {
        self.get_all()
    }

    fn get_mut(&mut self)-> &mut [Box<Com>] {
        self.get_all_mut()
    }
}



mod test{

    
    pub struct A{
        pub name : String
    }
    
    impl Default for A {
        fn default() -> Self {
            Self { name: "none".to_string() }
        }
    }
                
    #[test]
    fn test_all(){
        use crate::component::Components;
                    
        let mut components = Components::<A>::new();

        components.add_component(0);
        components.add_component(1);
        components.add_component(2);

        assert_eq!(components.have_component(0), true);
        assert_eq!(components.have_component(1), true);
        assert_eq!(components.have_component(2), true);
        assert_eq!(components.have_component(3), false);
        assert_eq!(components.have_component(4), false);

        components.remove_component(1);

        assert_eq!(components.have_component(0), true);
        assert_eq!(components.have_component(1), false);
        assert_eq!(components.have_component(2), true);
        assert_eq!(components.have_component(3), false);
        assert_eq!(components.have_component(4), false);

        assert_eq!(components.get(0).unwrap().name, "none");
        println!("{}", components.get(0).unwrap().name);
        
        components.get_mut(0).unwrap().name = "lzx".to_string();
        
        assert_eq!(components.get(0).unwrap().name, "lzx");
        println!("{}", components.get(0).unwrap().name);

        let coms = components.get_all();
        for c in coms {
            println!("{}", c.name);
        }

        let coms_mut = components.get_all_mut();
        let mut i = 0;
        for c in coms_mut {
            c.name = i.to_string();
            i  += 1;
        }

        let coms = components.get_all();
        for c in coms {
            println!("{}", c.name);
        }
    }
}