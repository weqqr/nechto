fn main() {
    nechto_build::build_shaders("../data/shader", "../build/shader", &["world.slang"]);
    nechto_build::build_scripts("../data/script", "../build/script");
}
