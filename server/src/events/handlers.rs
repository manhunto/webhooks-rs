use crate::events::models::CreateMessageRequest;
use crate::storage::Storage;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, Responder};

pub async fn create_message_handler(
    _storage: Data<Storage>,
    request: Json<CreateMessageRequest>,
    path: Path<String>,
) -> impl Responder {
    let app_id = path.into_inner();
    // todo check if app exists

    println!("{}", app_id);
    println!("{:?}", request.payload);

    HttpResponse::Created()
}
