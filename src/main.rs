#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate rocket_contrib;

#[cfg(test)]
mod tests;

use rocket::Request;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

// Routes

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        &TemplateContext {
            title: "Portfolio",
            parent: "layout",
        },
    )
}

#[get("/about")]
fn about() -> Template {
    Template::render(
        "about",
        &TemplateContext {
            title: "About",
            parent: "layout",
        },
    )
}

// Errors

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, about])
        .mount("/", StaticFiles::from("static"))
        .register(catchers![not_found])
        .attach(Template::fairing())
        .launch();
}
