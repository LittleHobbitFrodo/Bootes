use std::{io::Read, path::PathBuf};

fn main() {

    //println!("cargo:rustc-link-arg=-dT ./linker.ld");



    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut kernel_path = PathBuf::from(manifest.clone());
    _ = kernel_path.pop();
    kernel_path.push("kernel.elf");

    dbg!(&kernel_path);


    if std::fs::exists(&kernel_path).unwrap() {
        let mut file = match std::fs::File::open(kernel_path) {
            Ok(k) => k,
            Err(e) => panic!("failed to open kernel: {e:?}"),
        };

        let mut kernel = Vec::new();
        if let Err(e) = file.read_to_end(&mut kernel) {
            panic!("faile to load kernel: {e:?}");
        }



        let mut array = format!(r"
#[repr(align(8))]
pub struct BinaryHolder {{
    pub data: [u8; {}],
}}

pub static KERNEL_BINARY: BinaryHolder = BinaryHolder {{
    data: [", kernel.len());

        //let mut array = String::from("pub static KERNEL_BINARY: &'static [u8] = &[");

        let mut iter = kernel.iter();

        {
            let byte = *iter.next().unwrap();
            use std::fmt::Write;
            _ =write!(&mut array, "0x{byte:X}u8, ");

        }

        for byte in iter {
            use std::fmt::Write;
            _ = write!(&mut array, "0x{byte:X}, ");
        }

        array += "]\n};";

        let mut module = PathBuf::from(manifest);
        module.push("src/kernel_binary.rs");
        let mut output = match std::fs::File::create(module) {
            Ok(o) => o,
            Err(e) => panic!("faile to create module: {e:?}"),
        };

        use std::io::Write;
        if let Err(e) = output.write_all(array.as_bytes()) {
            panic!("failed to write output: {e:?}");
        }


    } else {
        panic!("kernel is not present");
    }

}
