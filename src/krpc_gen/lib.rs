mod original;
mod parser;
mod writer;
mod conversion;
mod output;

pub fn generate_for(path: &str) {

    let input_structure = original::deserialize_from_file(path);

    let output_structure = parser::create_output_structure(&input_structure["SpaceCenter"]);
   
    // for (getter, procedure) in &output_structure.getters {
    //     println!("{:?}: {:?}", getter, procedure);
    // }
    println!("{:?}", &output_structure.getters["ActiveVessel"]);
    
    writer::write_to_file("SpaceCenter", "output/space_center.rs", &output_structure);
    
    // println!("{:?}", v.space_center.procedures["get_ActiveVessel"]);
    
    // Convert into some more useable structure for code generation
    
}
