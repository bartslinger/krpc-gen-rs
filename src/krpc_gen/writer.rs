use std::fs::File;
use handlebars;

use crate::output;

pub fn write_to_file(service_name: &str, path: &str, output_structure: &output::OutputStructure) {
    let mut handlebars = handlebars::Handlebars::new();
    handlebars.register_template_file("template", "templates/service.rs.hbs").unwrap();

    let mut output_file = File::create(path).unwrap();

    let mut data = serde_json::Map::<String, serde_json::Value>::new();
    data.insert("service_name".to_string(), handlebars::to_json(service_name));

    data.insert("service_methods".to_string(), handlebars::to_json(&output_structure.methods));
    data.insert("service_getters".to_string(), handlebars::to_json(&output_structure.getters));
    data.insert("service_setters".to_string(), handlebars::to_json(&output_structure.setters));
    // data.insert("classes".to_string(), handlebars::to_json(&output_structure.classes));

    handlebars.render_to_write("template", &data, &mut output_file).unwrap();
}
