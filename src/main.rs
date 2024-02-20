mod templates;
mod component_injections;
mod controller;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::http::header::ContentType;
use askama::Template;
use actix_files as fs;
use crate::controller::{about, newsletter, post_newsletter, services, welcome, workshops};
use crate::templates::DashboardComponent;

async fn index() -> impl Responder {
    let nav_items = component_injections::get_nav_items().await;
    let component = DashboardComponent { nav_items };
    HttpResponse::Ok().content_type(ContentType::html()).body(component.render().unwrap())
}


#[derive(serde::Deserialize)]
pub struct SignupForm {
    email: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "static").show_files_listing())
            .service(fs::Files::new("/static/3p", "static").show_files_listing())
            .service(workshops)
            .service(services)
            .service(welcome)
            .service(about)
            .service(newsletter)
            .service(post_newsletter)
            .route("/", web::get().to(index))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
