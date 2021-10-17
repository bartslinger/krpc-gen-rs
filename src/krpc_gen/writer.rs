use std::fs::File;
use handlebars;
use handlebars::JsonRender;

use crate::output;

fn return_type_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> Result<(), handlebars::RenderError> {
    let param = h
        .param(0)
        .ok_or(handlebars::RenderError::new(
            "Param 0 with u64 type is required for rank helper.",
        ))?;
    match param.value()["type"].as_str() {
        Some("Class") => {
            out.write(&param.value()["name"].render())?;
        },
        _ => {
            out.write("()")?;
        },
    }
    Ok(())
}

pub fn write_to_file(service_name: &str, path: &str, output_structure: &output::OutputStructure) {
    let mut handlebars = handlebars::Handlebars::new();
    handlebars.register_template_file("template", "templates/service.rs.hbs").unwrap();
    handlebars.register_helper("return_type", Box::new(return_type_helper));

    let mut output_file = File::create(path).unwrap();

    let mut data = serde_json::Map::<String, serde_json::Value>::new();
    data.insert("service_name".to_string(), handlebars::to_json(service_name));

    let service_getters: Vec<_> = output_structure.getters.iter().map(|(name, func)| func).collect();
    
    data.insert("service_getters".to_string(), handlebars::to_json(service_getters));

    handlebars.render_to_write("template", &data, &mut output_file).unwrap();
}
