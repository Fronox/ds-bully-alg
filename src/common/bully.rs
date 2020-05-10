use super::models::*;

use rand::seq::SliceRandom;

fn get_rand_id(keys: Vec<Id>) -> Id {
    *keys.choose(&mut rand::thread_rng()).unwrap()
}

pub fn bully_election(storage: &mut Storage, verbose: bool) -> Id {
    let alive_process_ids: Vec<i32> = storage
        .keys()
        .filter(|id| storage[id].alive)
        .cloned()
        .collect();
    if alive_process_ids.is_empty() {
        return -1;
    }
    let mut current_id = get_rand_id(alive_process_ids);
    let new_coord_id;
    loop {
        let current_process: &Process = storage.get(&current_id).unwrap();
        let mut go_next = false;
        let nodes_ids: &Vec<Id> = current_process.nodes.as_ref().unwrap();
        for id in nodes_ids {
            if *id > current_id && storage[id].alive {
                current_id = *id;
                go_next = true;
                break;
            }
        }
        if !go_next {
            new_coord_id = current_process.id;
            break;
        }
    }
    if verbose {
        println!("New coord: {}", new_coord_id);
    }
    storage
        .iter_mut()
        .for_each(|(_, p)| p.number_of_elections += 1);
    new_coord_id
}
