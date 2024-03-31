pub struct SparseSet<V : ?Sized>{
    indices : Vec<usize>,
    sparse : Vec<Option<usize>>,
    dense : Vec<Box<V>>
}

impl<V> SparseSet<V> {
    pub fn new()-> Self{
        SparseSet{
            indices : Vec::new(),
            sparse : Vec::new(),
            dense : Vec::new()
        }
    }

    pub fn contine(self : &Self, id : usize) -> bool{
        if id >= self.sparse.len(){
            return false
        }

        match self.sparse[id] {
            Some(_) => true,
            None => false,
        }
    }

    pub fn inster(self : &mut Self, id : usize, value : Box<V>) -> Option<Box<V>>{
        if id >= self.sparse.len() {
            self.sparse.resize_with(id + 1, || None);
        }

        match self.contine(id) {
            true => {
                //此id已拥有一个同类型值，该操作会覆盖原有值
                let old = self.remove(id).unwrap();
                self.inster(id, value);
                Some(old)
            },
            false => {
                self.sparse[id] = Some(self.indices.len());
                self.indices.insert(self.indices.len(), id);
                self.dense.insert(self.dense.len(), value);
                None
            },
        }
    }

    pub fn remove(self : &mut Self, id : usize) -> Option<Box<V>>{
        match self.contine(id) {
            true => {
                let idx = self.sparse[id].unwrap();
                let last_id = self.indices.last().unwrap().clone();
                self.sparse[last_id] = Some(idx);
                self.sparse[id] = None;

                self.indices.swap_remove(idx);
                Some(self.dense.swap_remove(idx))
            },
            false => None,
        }
    }

    pub fn get(self : &Self, id : usize)->Option<&V>{
        match self.contine(id) {
            true => Some(&self.dense[self.sparse[id].unwrap()]),
            false => None,
        }
    }

    pub fn get_mut(self : &mut Self, id : usize)->Option<&mut V>{
        match self.contine(id) {
            true => Some(&mut self.dense[self.sparse[id].unwrap()]),
            false => None,
        }
    }

    pub fn get_all(&self) -> &[Box<V>]{
        &self.dense
    }

    pub fn get_all_mut(&mut self) -> &mut [Box<V>]{
        &mut self.dense
    }

    pub fn get_min_none(&self)->usize{
        for id in 0..self.sparse.len() {
            match self.sparse[id] {
                Some(_)=>{},
                None=>return id
            }
        }
        self.sparse.len()
    }
}