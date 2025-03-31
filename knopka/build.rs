fn main() {
    nechto_build::build_shaders(
        "../data/engine/shader",
        "../build/engine/shader",
        &["world.slang"],
    );
    nechto_build::build_scripts("../data/engine/script", "../build/engine/script");
}
