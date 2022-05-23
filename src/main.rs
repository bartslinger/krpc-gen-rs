use krpc_gen;
fn main() {

    krpc_gen::generate_for(
        std::path::Path::new("/home/bart/.local/share/Steam/steamapps/common/Kerbal Space Program/GameData/kRPC/KRPC.SpaceCenter.json"),
        std::path::Path::new("/home/bart/testdir/ksp/kerbal-computer/src/generated/services/space-center.ts")
    );

    // krpc_gen::generate_for(
    //     std::path::Path::new("/home/bart/.local/share/Steam/steamapps/common/Kerbal Space Program/GameData/kRPC/KRPC.Drawing.json"),
    //     std::path::Path::new("/home/bart/testdir/ksp/kerbal-computer/src/generated/services/drawing.ts")
    // );

    // krpc_gen::generate_for(
    //     std::path::Path::new("/home/bart/.local/share/Steam/steamapps/common/Kerbal Space Program/GameData/kRPC/KRPC.InfernalRobotics.json"),
    //     std::path::Path::new("/home/bart/testdir/ksp/kerbal-computer/src/generated/services/infernal-robotics.ts")
    // );

    // krpc_gen::generate_for(
    //     std::path::Path::new("/home/bart/.local/share/Steam/steamapps/common/Kerbal Space Program/GameData/kRPC/KRPC.KerbalAlarmClock.json"),
    //     std::path::Path::new("/home/bart/testdir/ksp/kerbal-computer/src/generated/services/kerbal-alarm-clock.ts")
    // );

    // krpc_gen::generate_for(
    //     std::path::Path::new("/home/bart/.local/share/Steam/steamapps/common/Kerbal Space Program/GameData/kRPC/KRPC.RemoteTech.json"),
    //     std::path::Path::new("/home/bart/testdir/ksp/kerbal-computer/src/generated/services/remote-tech.ts")
    // );

    // krpc_gen::generate_for(
    //     std::path::Path::new("/home/bart/.local/share/Steam/steamapps/common/Kerbal Space Program/GameData/kRPC/KRPC.UI.json"),
    //     std::path::Path::new("/home/bart/testdir/ksp/kerbal-computer/src/generated/services/ui.ts")
    // );
}
