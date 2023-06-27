use anchor_interface_syn::{Generator, GeneratorOptions};

pub fn main() {
    const IDL: &str = "jupiter.json";
    const OUT: &str = "src/_gen_.rs";
    let generator = {
        let opts = GeneratorOptions::builder().idl(IDL).build();
        Generator::from(&opts)
    };
    generator.gen_program_file(OUT);
    println!("cargo:rerun-if-changed={}", IDL);
    println!("cargo:rerun-if-changed={}", OUT);
}
