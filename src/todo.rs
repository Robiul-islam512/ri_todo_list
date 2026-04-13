

pub mod todo{
    use uuid::Uuid;
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize,PartialEq)]
    pub enum PriorityLevel{
        High,
        Medium,
        Low
    }
    #[derive(Debug, Serialize, Deserialize,PartialEq)]
    pub enum TaskStatus{
        Completed,
        Pending,
        NotGoingToDo(String),
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Task{
        pub _id: u32,
        pub task_name:String,
        pub priority_level: PriorityLevel,
        pub task_status: TaskStatus,
        pub task_description: String,
        pub remainder: String,
        pub created_at: String,
        pub updated_at: String,
    }

    impl Task {
        pub fn new(&self,_id:u32,task_name:String,priority_level: PriorityLevel,task_status: TaskStatus,task_description: String,remainder: String,created_at: String,updated_at: String) -> Self {
            Task{
                _id:self._id,
                task_name:self.task_name.trim().to_lowercase(),
                priority_level,
                task_status,
                task_description:self.task_description.trim().to_uppercase(),
                remainder,
                created_at,
                updated_at,
                
            }
        }
    }
}