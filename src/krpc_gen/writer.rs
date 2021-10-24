use std::fs::File;
use handlebars;

use crate::output;

pub fn write_to_file(service_name: &str, path: &std::path::Path, output_structure: &output::OutputStructure) {
    let mut handlebars = handlebars::Handlebars::new();
    
    let template_bytes = std::include_bytes!("../../templates/service.rs.hbs");
    handlebars.register_template_string("template", String::from_utf8_lossy(template_bytes)).unwrap();

    let mut output_file = File::create(path).unwrap();

    let mut data = serde_json::Map::<String, serde_json::Value>::new();
    data.insert("service_name".to_string(), handlebars::to_json(service_name));

    data.insert("service_methods".to_string(), handlebars::to_json(&output_structure.methods));
    data.insert("service_getters_setters".to_string(), handlebars::to_json(&output_structure.getters_setters));
    data.insert("classes".to_string(), handlebars::to_json(&output_structure.classes));
    data.insert("enumerations".to_string(), handlebars::to_json(&output_structure.enumerations));

    handlebars.render_to_write("template", &data, &mut output_file).unwrap();
    
    // let mut output_test_file = File::create("output/generated.rs").unwrap();
    // handlebars.render_to_write("template", &data, &mut output_test_file).unwrap();
}
