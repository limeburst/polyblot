extern crate actix_web;
extern crate listenfd;
extern crate fluent_locale;
extern crate accept_language;

use listenfd::ListenFd;
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
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            .resource("/", |r| r.f(print_chosen_language))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run();
}
