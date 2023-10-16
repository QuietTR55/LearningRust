use std::{rc::Rc, sync::{Arc, Mutex}, thread, ptr::null};
use GameObjectManager::GameObjectManager::GameObject;
use GameObjectIdManager::IdManager::{GObjectIdManager};
use std::thread::*;

use crate::GameObjectManager::GameObjectManager::GObjectManager;

mod GameObjectIdManager;
mod GameObjectManager;

fn main() {
    let mut id_manager = GObjectIdManager::default();
    let mut shared_id_manager = Arc::new(Mutex::new(id_manager));
    
    let thread = thread::spawn(||{
        let testing_string : String = String::from("This is my testing string");
        println!("Testing string is {}", testing_string);
        let mut loop_index : i32 = 0;
    });
    
    thread.join().unwrap();

    println!("Second testing string");
}

pub struct GameGlobalReferences{
    pub game_object_id_manager : Option<Arc<Mutex<GObjectIdManager>>>,
    pub game_object_manager : Option<Arc<Mutex<GObjectManager>>>
}

impl Default for GameGlobalReferences{
    fn default() -> GameGlobalReferences{
        GameGlobalReferences{
            game_object_id_manager : None,
            game_object_manager : None,
        }
    }
}

trait GameComponentTrait {
    fn update(&mut self);
    fn get_game_object(&self) -> Rc<GameObject>;
    fn set_game_object(&mut self, game_object: Rc<GameObject>);
}

trait ComponentHolderTrait{
    fn add_component(&mut self,component : Rc<Box<dyn GameComponentTrait>>);
    fn get_component<T : GameComponentTrait>(&mut self) -> Rc<Box<dyn GameComponentTrait>>;
}

struct TransformComponent {
    x: f32,
    y: f32,
    z: f32,
    game_object: Rc<GameObject>,
}

impl GameComponentTrait for TransformComponent {
    fn update(&mut self) {
        
    }

    fn get_game_object(&self) -> Rc<GameObject> {
        self.game_object.clone()
    }

    fn set_game_object(&mut self, game_object: Rc<GameObject>) {
        self.game_object = game_object;
    }
}