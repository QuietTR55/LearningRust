pub mod GameObjectManager{
    use std::rc::Rc;
    use std::sync::Mutex;
    use std::sync::Arc;

    use crate::GameGlobalReferences;
    use crate::GameObjectIdManager::IdManager::GObjectIdManager;


    pub struct GObjectManager{
        pub game_objects : Vec<Box<GameObject>>,
    }

    impl Default for GObjectManager{
        fn default() -> Self {
            GObjectManager { game_objects: vec![] }
        }
    }  

    impl GObjectManager {
        pub fn create_new_game_object(&mut self, id_manager : &mut GObjectIdManager) -> Box<GameObject>{
            println!("Got id manager");
            let new_game_object = GameObject{
                object_id : (*id_manager).get_new_object_id(),
            };

            return Box::new(new_game_object);
        }
    }

    pub struct GameObject {
        pub object_id : i64
    }
}