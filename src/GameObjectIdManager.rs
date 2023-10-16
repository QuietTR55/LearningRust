pub mod IdManager{
    use rand::Rng;
    use lazy_static::lazy_static;
    use rand::rngs::OsRng;
    use rand::rngs::adapter::ReseedingRng;
    use rand_chacha::ChaChaRng;
    use rand::prelude::*;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::sync::Mutex;


    pub struct GObjectIdManager {
        used_ids: Vec<i64>,
        rng: rand::rngs::ThreadRng,
    }

    impl Default for GObjectIdManager{
        fn default() -> GObjectIdManager{
            GObjectIdManager{
                used_ids: Vec::new(),
                rng: thread_rng()
            }
        }
    }

    impl GObjectIdManager {

        pub fn get_new_object_id(&mut self) -> i64 
        {
            let random_i64 = loop{
                let mut candidate : i64 = self.rng.gen();
                
                if !self.used_ids.contains(&candidate){
                    self.used_ids.push(candidate);
                    return candidate;
                }
            };
        }
    }
}