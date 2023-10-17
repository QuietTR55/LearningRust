use std::{rc::Rc, sync::{Arc, Mutex}, thread, ptr::null};
use GameObjectManager::GameObjectManager::GameObject;
use GameObjectManager::GameObjectManager::GObjectManager;
use GameObjectIdManager::IdManager::{GObjectIdManager};

mod GameObjectIdManager;
mod GameObjectManager;

fn main() {
    println!("creating id manager");
    let mut id_manager = GObjectIdManager::default();
    println!("creating shared id manager");

    let mut shared_id_manager = Arc::new(Mutex::new(id_manager));
    println!("creating game global references");

    let mut game_global_references = GameGlobalReferences{
        game_object_id_manager: Box::new(GObjectIdManager::default()),
        game_object_manager: Box::new(GObjectManager::default()),
    };
    
    println!("creating shared game global references");

    let shared_global_references = Arc::new(Mutex::new(game_global_references));
    
    println!("creating a new game object");
    let created_game_object = shared_global_references.lock().unwrap().create_game_object();

    println!("created game object with id {}", created_game_object.object_id);

}

pub struct GameGlobalReferences{
    pub game_object_id_manager : Box<GObjectIdManager>,
    pub game_object_manager : Box<GObjectManager>,
}

impl GameGlobalReferences{

    fn create_game_object (&mut self) -> Box<GameObject>{
        return (self.game_object_manager).create_new_game_object(&mut self.game_object_id_manager);
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