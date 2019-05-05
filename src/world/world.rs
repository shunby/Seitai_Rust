use super::super::living::living::*;

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;


pub struct World{
    pub livings: Arc<RwLock<Vec<Arc<Mutex<Living>>>>>,
    pub next_id: Arc<Mutex<usize>>,
    pub to_be_removed: Arc<Mutex<Vec<Arc<Mutex<Living>>>>>,
    pub to_be_added: Arc<Mutex<Vec<Arc<Mutex<Living>>>>>,
    pub chunks : Arc<Mutex<Vec<Vec<Vec<Arc<Mutex<Living>>>>>>>
}

impl World{
    pub fn new() -> World{
        World{
            livings: Arc::new(RwLock::new(Vec::new())),
            next_id: Arc::new(Mutex::new(0)),
            to_be_removed: Arc::new(Mutex::new(Vec::new())),
            to_be_added: Arc::new(Mutex::new(Vec::new())),
            chunks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_living(&self, living: &Arc<Mutex<Living>>){
        let mut add = self.to_be_added.lock().unwrap();
        add.push(living.clone());
    }

    pub fn remove_living(&self, living: &Arc<Mutex<Living>>) {
        let mut rem = self.to_be_removed.lock().unwrap();
        rem.push(living.clone());
    }

    fn _add_living(& self, living: Arc<Mutex<Living>>){
        {
            let mut li = living.lock().unwrap();
            li.set_id(*self.next_id.lock().unwrap());
        }
        let mut livs = self.livings.write().unwrap();
        livs.push(living);
        *self.next_id.lock().unwrap() += 1;
    }


    fn _remove_living(& self, living: Arc<Mutex<Living>>){

        let li = living.lock().unwrap();
        let deleted_id = li.get_id();

        let mut livings = self.livings.write().unwrap();
        let mut next_id = self.next_id.lock().unwrap();

        if deleted_id >= livings.len(){
            eprintln!("remove_living: given index is out of bound!");
            return;
        }else if livings.len() < 2{
            livings.clear();
            *next_id = 0;
            return;
        }else if deleted_id == *next_id - 1{
            livings.pop();
            *next_id -= 1;
            return;
        }
        livings[*next_id - 1].lock().unwrap().set_id(deleted_id);
        
        livings.swap_remove(deleted_id);
        
        *next_id -= 1;

    }

    pub fn flush_modify_livings(&mut self){
        let mut add = self.to_be_added.lock().unwrap();
        let mut rem = self.to_be_removed.lock().unwrap(); 
        for li in &*add{
            self._add_living(li.clone());
        }
        for li in &*rem{
            self._remove_living(li.clone());
        }
        add.clear();
        rem.clear();
    }
}
