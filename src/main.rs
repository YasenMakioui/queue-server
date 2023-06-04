use std::collections::HashMap;

struct Message {
    id: String,
    content: String,
    creation_date: String
}


struct Queue {
    name: String,
    creation_date: String,
    queue: Vec<Message>
}

struct Registry {
    id: String,
    queues: HashMap<String , Queue>
}

impl Message {
    fn new(id: String, content: String, creation_date: String) -> Self {
        Self { id, content, creation_date }
    }

    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_content(&self) -> &str {
        &self.content
    }

    fn get_creation_date(&self) -> &str {
        &self.creation_date
    }
}

impl Queue {
    fn new(name: String, creation_date: String) -> Self {
        let queue = Vec<Message>
        Self { name, creation_date, queue }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_creation_date(&self) -> &str {
        &self.creation_date
    }

    fn get_queue(&self) -> &Vec<Message> {
        &self.queue
    }
}

impl Registry {
    fn new() -> Self {
        Self {id: String::from("some_id"), queues: HashMap::new()}
    }

    fn create_queue(&self) -> &str {
        let id = String::from("random");
        let queue = Queue::new(
            name: String::from("my_first_queue"),
            creation_date: String::from("random_date"),
        );
        self.queues.insert(
            id,
            queue
        );
        &id
    }
}


fn main() {
    let id = String::from("some_id");
    let content = String::from("some_content");
    let creation_date = String::from("creation_date");
    let message = Message::new(id, content, creation_date);

    println!("{}",message.get_id())
}
