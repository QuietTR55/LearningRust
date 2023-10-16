use crate::GameObjectIdManager;
pub mod GameObjectManager{
    use std::clone;
    use std::sync::Mutex;
    use std::sync::Arc;

    use crate::GameGlobalReferences;


    pub struct GObjectManager{
        pub game_objects : Vec<Arc<Mutex<GameObject>>>,
        pub global_references : Option<Arc<Mutex<GameGlobalReferences>>>,
    }

    impl GObjectManager {
        pub fn create_new_game_object(&mut self) -> Option<Arc<Mutex<GameObject>>>{

            match self.global_references.clone(){
                Some(arc)=>
                {
                    let id_manager = arc.lock().unwrap().game_object_id_manager.clone();

                    match id_manager{
                        Some(id_manager_arc) => 
                        {
                            let object_id = id_manager_arc.lock().unwrap().get_new_object_id();
                            let game_object = GameObject { object_id:  object_id};
                            let shared_game_object = Arc::new(Mutex::new(game_object));
                            self.game_objects.push(shared_game_object.clone());
                            Some(shared_game_object)
                        }
                        None => None
                    }

                }
                None =>
                {
                    println!("global_references is none in GObjectManager");
                    None
                }
            }

            
        }
    }

    pub struct GameObject {
        pub object_id : i64
    }
}