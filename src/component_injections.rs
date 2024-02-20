use crate::templates::{NavItem, SubMenuItem};

pub async fn get_nav_items() -> Vec<NavItem> {
    let nav_items = vec![
        NavItem {
            title: "Home".to_string(),
            href: "/hey".to_string(),
            submenu: vec![],
            has_submenu: false,
            is_home: true,
        },
        NavItem {
            title: "About".to_string(),
            href: "/about".to_string(),
            submenu: vec![],
            has_submenu: false,
            is_home: false,
        },
        NavItem {
            title: "Services".to_string(),
            href: "/services".to_string(),
            has_submenu: true,
            submenu: vec![
                SubMenuItem {
                    title: "AI Playground".to_string(),
                    href: "/ai".to_string(),
                },
                SubMenuItem {
                    title: "Pact".to_string(),
                    href: "/pact".to_string(),
                },
            ],
            is_home: false,
        },
        NavItem {
            title: "Workshops".to_string(),
            href: "/workshops".to_string(),
            submenu: vec![
                SubMenuItem {
                    title: "HTMX, Rust, Actix".to_string(),
                    href: "/rahstack".to_string(),
                },
            ],
            has_submenu: true,
            is_home: false,
        },
        NavItem {
            title: "Signup".to_string(),
            href: "/newsletter".to_string(),
            submenu: vec![],
            has_submenu: false,
            is_home: false,
        },
        NavItem {
            title: "Contact".to_string(),
            href: "/app/contact".to_string(),
            submenu: vec![],
            has_submenu: false,
            is_home: false,
        },
    ];
    nav_items
}

pub async fn get_workshops() -> Vec<Workshop> {
    let workshops = vec![
        Workshop {
            title: "Introduction to Rust".to_string(),
            description: "Learn the basics of Rust".to_string(),
            image_url: "/static/rust.png".to_string(),
            signup_url: "/signup".to_string(),
        },
        Workshop {
            title: "Introduction to HTMX".to_string(),
            description: "Learn the basics of HTMX".to_string(),
            image_url: "/static/htmx.png".to_string(),
            signup_url: "/signup".to_string(),
        },
        Workshop {
            title: "Introduction to Actix-Web".to_string(),
            description: "Learn the basics of Actix-Web".to_string(),
            image_url: "/static/actix-web.png".to_string(),
            signup_url: "/signup".to_string(),
        },
        Workshop {
            title: "HTMX, Actix, Askama and Rust".to_string(),
            description: "Create a small app using the RAAH stack".to_string(),
            image_url: "/static/RAAHstack.png".to_string(),
            signup_url: "/signup".to_string(),
        },
    ];
    workshops
}

pub async fn get_services() -> Vec<String> {
    let services = vec![
        "For entrepreneurs".to_string(),
        "For students".to_string(),
        "For hobbyists".to_string(),
    ];
    services
}

#[derive(Debug, Clone)]
pub struct Workshop {
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub signup_url: String,
}
