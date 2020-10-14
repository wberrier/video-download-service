// Use lazy_static for the template engine
lazy_static! {
    pub static ref TEMPLATE_ENGINE: handlebars::Handlebars<'static> = {
        let mut engine = handlebars::Handlebars::new();

        let templates = ["index", "error", "finished"];

        for template in &templates {
            let base_dir = "./templates".to_string();
            let filename = template.to_string() + ".html";
            let full_filename =
                base_dir + std::path::MAIN_SEPARATOR.to_string().as_str() + filename.as_str();

            match engine.register_template_file(filename.as_str(), full_filename) {
                Ok(_) => {}
                Err(_) => eprintln!("Unable to register {}", filename),
            }
        }

        engine
    };
}
