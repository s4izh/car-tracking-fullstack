use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TaskIdentifier {
    task_global_id: String,
}

#[get("/task/{task_global_id}")]
pub async fn get_data(
    task_identifier: Path<TaskIdentifier>,
    // body: Json<TaskIdentifier>,
) -> Json<String> {
    Json(task_identifier.into_inner().task_global_id)
}

// #[post("/add")]
// pub async fn add (nums: web::Json<Nums>) -> impl Responder {
//     Operation {
//         operation: "add".to_string(),
//         result: nums.first + nums.second,
//     }
// }
