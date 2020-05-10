use std::collections::BTreeMap;

pub type Id = i32;
pub type Storage = BTreeMap<Id, Process>;
pub type Name = String;
pub struct Process {
    pub id: Id,
    pub name: Name,
    pub number_of_elections: i32,
    pub nodes: Option<Vec<Id>>,
    pub alive: bool,
}

impl Process {
    pub fn new(id: Id, name: Name, number_of_elections: i32) -> Process {
        Process {
            id,
            name,
            number_of_elections,
            nodes: None,
            alive: true,
        }
    }

    pub fn set_ids(&mut self, nodes: Vec<Id>) {
        self.nodes = Some(nodes)
    }

    pub fn display(&self) -> String {
        format!("{}, {}_{}", self.id, self.name, self.number_of_elections)
    }
}
