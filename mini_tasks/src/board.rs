use crate::task::Task;
use std::collections::HashMap;

pub struct Board {
    lists: HashMap<String, Vec<Task>>
}

impl Board {
    pub fn new() -> Board {
        Board { 
            lists: HashMap::new(), 
        }
    }

    pub fn add_task(&mut self, list_name: &str, task: Task) {
        let wektor = self.lists.entry(list_name.to_string()).or_insert(Vec::new());
        wektor.push(task);
    }

    pub fn complete_task(&mut self, list_name: &str, title: &str) -> bool {
        if let Some(tasks) = self.lists.get_mut(list_name){
            for task in tasks {
                if task.title == title {
                    task.complete();
                    return true;
                }
            }
        }
         false
    }

    pub fn print_list(& self, list_name: &str) {
        match self.lists.get(list_name) {
            Some(tasks) => {
                print!("{list_name}:");
                for task in tasks {
                    println!("{}", task.summary());
                }
            },
            None => println!("lista nie istnieje")
        }
    }

    pub fn count_pending(&self) -> usize {
        let mut count = 0;

        for (_, tasks) in &self.lists {
            for task in tasks {
                if !task.done {
                    count += 1;
                }
            }
        }
        count
    }
}
