// Use lazy_static for the template engine
lazy_static! {
    pub static ref TEMPLATE_ENGINE: handlebars::Handlebars<'static> = {
        let mut engine = handlebars::Handlebars::new();

        match engine.register_template_file("index.html", "./templates/index.html") {
            Ok(_) => {}
            Err(_) => eprintln!("Unable to register index.html"),
        }

        engine
    };
}
