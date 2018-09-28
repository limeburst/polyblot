extern crate actix_web;
extern crate fluent_locale;
extern crate accept_language;

use actix_web::{server, App, HttpRequest, Responder};
use fluent_locale::Locale;

fn choose_language(req: &HttpRequest) -> String
{
    let accept_language = req.headers().get("accept-language").unwrap().to_str().unwrap();
    let parsed = accept_language::parse(accept_language);
    let locale = Locale::from(parsed[0].clone());  // FIXME

    locale.get_language().to_string()
}

fn print_chosen_language(req: &HttpRequest) -> impl Responder {
    let chosen_language = choose_language(req);

    format!("{}", chosen_language)
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(print_chosen_language))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
