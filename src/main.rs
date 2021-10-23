use krpc_gen;
fn main() {

    krpc_gen::generate_for(
        std::path::Path::new("/home/bart/.local/share/Steam/steamapps/common/Kerbal Space Program/GameData/kRPC/KRPC.SpaceCenter.json"),
        std::path::Path::new("output/space_center.rs")
    );
}
