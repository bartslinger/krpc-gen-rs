mod original;
mod parser;
mod writer;
mod output;

pub fn generate_for(path: &str) {

    let input_structure = original::deserialize_from_file(path);

    for (service_name, content) in input_structure {
        let output_structure = parser::create_output_structure(&content);
        writer::write_to_file(service_name.as_str(), "output/space_center.rs", &output_structure);
    }

   
    // for (getter, procedure) in &output_structure.getters {
    //     println!("{:?}: {:?}", getter, procedure);
    // }
    // println!("{:?}", &output_structure.getters);
    
    
    // println!("{:?}", v.space_center.procedures["get_ActiveVessel"]);
    
    // Convert into some more useable structure for code generation
    
}
