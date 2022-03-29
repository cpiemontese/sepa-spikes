use actix_web::{post, HttpRequest, HttpResponse};

#[post("/webhook")]
pub async fn webhook(req: HttpRequest) -> HttpResponse {
    dbg!(req);
    HttpResponse::Ok().finish()
}
