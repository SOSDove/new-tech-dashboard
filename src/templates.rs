use askama::Template;
use crate::component_injections::Workshop;

#[derive(Template)]
#[template(path = "dashboard-component.html")]
pub struct DashboardComponent {
    pub(crate) nav_items: Vec<NavItem>,
}

#[derive(Template)]
#[template(path = "welcome.html")]
pub struct WelcomeComponent {
}

#[derive(Template)]
#[template(path = "services.html")]
pub struct ServicesComponent {
}

pub struct NavItem {
    pub(crate) title: String,
    pub(crate) href: String,
    pub(crate) submenu: Vec<SubMenuItem>,
    pub(crate) has_submenu: bool,
    pub(crate) is_home: bool,
}

pub struct SubMenuItem {
    pub(crate) title: String,
    pub(crate) href: String,
}

#[derive(Debug, Clone, Template)]
#[template(path = "workshop.html")] // This path will depend on your project's structure.
pub struct WorkshopsPage {
    pub workshops: Vec<Workshop>,
}

#[derive(Debug, Clone, Template)]
#[template(path = "success_alert.html")] // This path will depend on your project's structure.
pub struct SuccessAlertComponent {
}

#[derive(Debug, Clone, Template)]
#[template(path = "failure_alert.html")] // This path will depend on your project's structure.
pub struct FailureAlertComponent {
    pub(crate) failure_reason: String,
}

#[derive(Debug, Clone, Template)]
#[template(path = "about.html")] // This path will depend on your project's structure.
pub struct AboutPage {
}

#[derive(Debug, Clone, Template)]
#[template(path = "newsletter-form.html")] // This path will depend on your project's structure.
pub struct NewsletterComponent {}