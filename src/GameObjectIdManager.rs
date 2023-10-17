pub mod IdManager{
    use rand::Rng;
    use rand::prelude::*;
    use crate::GameGlobalReferences;

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
            println!("Getting new object id");
            let random_i64 = loop{
                let mut candidate : i64 = self.rng.gen();
                println!("random id candidate is {}", candidate);
                if !self.used_ids.contains(&candidate){
                    println!("used_ids doesn't contain {}", candidate);
                    self.used_ids.push(candidate);
                    break candidate;
                }
            };
            println!("Returning {}", random_i64);
            return random_i64;
        }
    }
}