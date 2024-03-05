use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone)]
struct Application {
    #[allow(dead_code)]
    id: Uuid,
}

impl Application {
    fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

struct Storage {
    applications: Mutex<Vec<Application>>,
}

impl Storage {
    fn new() -> Self {
        Self {
            applications: Mutex::new(vec![]),
        }
    }

    fn add_application(&self, app: Application) {
        let mut applications = self.applications.lock().unwrap();

        applications.push(app);
    }

    fn application_count(&self) -> usize {
        let applications = self.applications.lock().unwrap();

        applications.len()
    }
}

async fn create_application(storage: Data<Storage>) -> impl Responder {
    let app = Application::new();

    storage.add_application(app.clone());

    println!(
        "Application created: {:?}, count: {}",
        app,
        storage.application_count()
    );

    HttpResponse::NoContent()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1";
    let port = 8090;

    println!(
        "Webhooks server is listening for requests on {}:{}",
        ip, port
    );

    let storage = Data::new(Storage::new());

    HttpServer::new(move || {
        App::new()
            .app_data(storage.clone())
            .route("/api/v1/application", web::post().to(create_application))
    })
    .bind((ip, port))?
    .run()
    .await
}
