#[derive(Debug)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub priority: Priority,
    pub done: bool,
}

impl Task {
    pub fn new(title: &str, priority: Priority) -> Task {
        Task { 
            title: String::from(title), 
            priority, 
            done: false, 
        }
    }

    pub fn complete(&mut self){
        self.done = true
    }

    pub fn summary(&self) -> String {
        let prio = match &self.priority {
            Priority::Low => "Niski",
            Priority::Medium => "Sredni",
            Priority::High => "Wysoki",
        };

        let status = if self.done {"[x]"} else {"[]"};

        format!("{status} {} {prio}", self.title)
    }
}