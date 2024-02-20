use actix_web::{get, HttpResponse, post, Responder, web};
use actix_web::http::header::ContentType;
use askama::Template;
use regex::Regex;
use crate::{component_injections, SignupForm, templates};
use crate::templates::{ServicesComponent, WelcomeComponent, WorkshopsPage};

#[get("/workshops")]
pub async fn workshops() -> impl Responder {
    let workshops = component_injections::get_workshops().await;
    let component = WorkshopsPage { workshops };
    HttpResponse::Ok().content_type(ContentType::html()).body(component.render().unwrap())
}

#[get("/welcome")]
pub async fn welcome() -> impl Responder {
    let component = WelcomeComponent {};
    HttpResponse::Ok().content_type(ContentType::html()).body(component.render().unwrap())
}

#[get("/services")]
pub async fn services() -> impl Responder {
    let component = ServicesComponent {};
    HttpResponse::Ok().content_type(ContentType::html()).body(component.render().unwrap())
}

#[get("/about")]
pub async fn about() -> impl Responder {
    let component = templates::AboutPage {};
    HttpResponse::Ok().content_type(ContentType::html()).body(component.render().unwrap())
}

#[get("/newsletter")]
pub async fn newsletter() -> impl Responder {
    let component = templates::NewsletterComponent {};
    HttpResponse::Ok().content_type(ContentType::html()).insert_header(("HX-Retarget", "#modal-container")).body(component.render().unwrap())
}

#[post("/newsletter")]
pub async fn post_newsletter(signup_form: web::Form<SignupForm>) -> impl Responder {
    let email_regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    if !email_regex.is_match(&signup_form.email) {
        println!("Received invalid email: {}", signup_form.email);
        let failure_alert = templates::FailureAlertComponent { failure_reason: "Provided Email Was Invalid".to_string() };
        let failure_alert_render = failure_alert.render().unwrap();
        return HttpResponse::BadRequest().content_type(ContentType::html()).body(failure_alert_render);
    }
    println!("Received valid email: {}", signup_form.email);
    let success_alert = templates::SuccessAlertComponent {};
    let success_alert_render = success_alert.render().unwrap();
    return HttpResponse::Ok().content_type(ContentType::html()).body(success_alert_render);
}
