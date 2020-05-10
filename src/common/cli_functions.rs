use super::models::*;

pub fn pretty_print_storage(storage: &Storage, current_coordinator: Id) -> String {
    let mut printed: Vec<String> = Vec::new();
    storage.iter().for_each(|(_, process)| {
        if !process.alive {
            return;
        }
        let line = if process.id == current_coordinator {
            format!("{}{}", process.display(), "*")
        } else {
            process.display()
        };
        printed.push(line);
    });
    printed.join("\n")
}

pub fn deconstruct_line(config_line: String) -> (Id, Name, i32) {
    let parts: Vec<&str> = config_line.split(",").collect();
    let id = parts[0].parse().unwrap();
    let full_name_parts: Vec<&str> = parts[1].trim().split("_").collect();
    let name = String::from(full_name_parts[0]);
    let number_of_elections = full_name_parts[1].parse::<i32>().unwrap();
    (id, name, number_of_elections)
}

pub fn kill_process(storage: &mut Storage, id: Id) {
    match storage.get_mut(&id) {
        Some(process_to_kill) => {
            process_to_kill.alive = false;
        }
        None => {}
    }
}
pub fn read_processes(content: String, storage: &mut Storage) {
    let mut ids: Vec<Id> = Vec::new();
    content.lines().for_each(|line| {
        let (id, name, elections) = deconstruct_line(line.to_string());
        ids.push(id);
        if !storage.contains_key(&id) {
            let process = Process::new(id, name, elections);
            storage.insert(id, process);
        } else {
            let process = storage.get_mut(&id).unwrap();
            if !process.alive {
                process.alive = true;
                process.number_of_elections = 0;
            }
        }
    });

    ids.sort();
    storage
        .iter_mut()
        .for_each(|(_, process)| process.set_ids(ids.clone()));
}
