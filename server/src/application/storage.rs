use crate::application::domain::Application;
use std::sync::Mutex;

pub trait ApplicationStorage {
    fn save(&self, app: Application);

    fn count(&self) -> usize;
}

pub struct InMemoryApplicationStorage {
    applications: Mutex<Vec<Application>>,
}

impl InMemoryApplicationStorage {
    pub fn new() -> Self {
        Self {
            applications: Mutex::new(vec![]),
        }
    }
}

impl ApplicationStorage for InMemoryApplicationStorage {
    fn save(&self, app: Application) {
        let mut applications = self.applications.lock().unwrap();

        applications.push(app);
    }

    fn count(&self) -> usize {
        let applications = self.applications.lock().unwrap();

        applications.len()
    }
}
