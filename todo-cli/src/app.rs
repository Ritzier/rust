use inquire::{error::InquireResult, Confirm, Select, Text};

pub struct App {
    tasks: Vec<Task>,
}

struct Task {
    description: String,
    completed: bool,
}

impl App {
    pub fn new(file: String) -> Self {
        Self { tasks: vec![] }
    }

    pub fn run(&mut self) -> InquireResult<()> {
        loop {
            let options = vec![
                "Add Task",
                "View Tasks",
                "Toggle Complete",
                "Remove Task",
                "Quit",
            ];
            let action = Select::new("Choose an option:", options).prompt()?;

            match action {
                "Add Task" => self.add_task()?,
                "View Tasks" => self.view_tasks()?,
                "Toggle Complete" => self.toggle_complete()?,
                "Remove Task" => self.remove_task()?,
                "Quit" => break,
                _ => continue,
            }
        }

        Ok(())
    }

    fn add_task(&mut self) -> InquireResult<()> {
        let task_description = Text::new("Enter the task description:").prompt()?;

        self.tasks.push(Task {
            description: task_description,
            completed: false,
        });

        Ok(())
    }

    fn view_tasks(&self) -> InquireResult<()> {
        if self.tasks.is_empty() {
            println!("No tasks available");
        } else {
            for (i, task) in self.tasks.iter().enumerate() {
                let status = if task.completed { "✓" } else { "☐" };
                println!("{}. {} {}", i + 1, task.description, status);
            }
        }
        Ok(())
    }

    fn toggle_complete(&mut self) -> InquireResult<()> {
        if self.tasks.is_empty() {
            println!("No tasks avaiable to complete.");
            return Ok(());
        }

        let task_description: Vec<String> = self
            .tasks
            .iter()
            .enumerate()
            .map(|(i, task)| format!("{}. {}", i + 1, task.description))
            .collect();

        let task_index = Select::new("Choose a task to mark as complete:", task_description)
            .prompt()?
            .split('.')
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap()
            - 1;

        self.tasks[task_index].completed = !self.tasks[task_index].completed;

        Ok(())
    }

    fn remove_task(&mut self) -> InquireResult<()> {
        if self.tasks.is_empty() {
            println!("No tasks available to remove.");
            return Ok(());
        }

        let task_descriptions: Vec<String> = self
            .tasks
            .iter()
            .enumerate()
            .map(|(i, task)| format!("{}. {}", i + 1, task.description))
            .collect();

        let task_index = Select::new("Choose a task to remove:", task_descriptions)
            .prompt()?
            .split('.')
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap()
            - 1;

        let confirm = Confirm::new(&format!(
            "Are you sure you want to remove the task: '{}'",
            self.tasks[task_index].description
        ))
        .prompt()?;

        if confirm {
            self.tasks.remove(task_index);
            println!("Task removed successfully!");
        } else {
            println!("Task removal canceled.");
        }

        Ok(())
    }
}
