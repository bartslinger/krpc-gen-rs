mod original;
mod parser;

pub fn generate_for(path: &str) {

    let input_structure = original::deserialize_from_file(path);

    let output_structure = parser::create_output_structure(&input_structure);
   
    for (getter, procedure) in &output_structure.classes["Vessel"].getters {
        println!("{:?}: {:?}", getter, procedure);
    }
    
    // println!("{:?}", v.space_center.procedures["get_ActiveVessel"]);
    
    // Convert into some more useable structure for code generation
    
}
