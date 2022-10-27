use actix_web::*;

pub async fn catalogo() -> HttpResponse {
    HttpResponse::Ok()
    .content_type("application/json; charset=utf-8")
    .body(r#"[
        {
            "message": "Olá, mundo!"
        },
        {
            "message": "Primeira API rest em Rust!"
        }
    ]"#)
}