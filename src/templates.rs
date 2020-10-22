use std::collections::HashMap;

// Use lazy_static for the template engine
lazy_static! {
    pub static ref TEMPLATE_ENGINE: handlebars::Handlebars<'static> = {
        let mut engine = handlebars::Handlebars::new();

        // Include the html in the binary for easy deployment
        // (this is not much html...)
        let templates: HashMap<&str, &'static str> = [
            ("index.html",    include_str!("../templates/index.html")),
            ("error.html",    include_str!("../templates/error.html")),
            ("finished.html", include_str!("../templates/finished.html")),
            ("filelist.html", include_str!("../templates/filelist.html")),
        ].iter().cloned().collect();

        for (filename, file_contents) in &templates {
            match engine.register_template_string(filename, file_contents) {
                Ok(_) => {}
                Err(_) => eprintln!("Unable to register {}", filename),
            }
        }

        engine
    };
}
