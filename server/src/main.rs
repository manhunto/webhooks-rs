use actix_web::web::{Data, Json};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::{debug, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone)]
struct Application {
    id: Uuid,
    name: String,
}

impl Application {
    fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
        }
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

#[derive(Deserialize)]
struct CreateAppRequest {
    name: String,
}

#[derive(Serialize)]
struct CreateAppResponse {
    id: String,
    name: String,
}

impl From<Application> for CreateAppResponse {
    fn from(value: Application) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name,
        }
    }
}

async fn create_application(
    storage: Data<Storage>,
    request: Json<CreateAppRequest>,
) -> impl Responder {
    let app = Application::new(request.name.to_string());

    storage.add_application(app.clone());

    debug!(
        "Application created: {:?}, count: {}",
        app,
        storage.application_count()
    );

    let response = CreateAppResponse::from(app);

    HttpResponse::Created().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_log();

    let ip = "127.0.0.1";
    let port = 8090;

    debug!(
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

fn init_log() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%+)(utc)} [{f}:{L}] {h({l})} {M}:{m}{n}",
        )))
        .build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
        .unwrap();

    log4rs::init_config(config).unwrap();
}
