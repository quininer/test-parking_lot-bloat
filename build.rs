use std::{ fs, env };
use std::io::{ self, Write };
use std::path::Path;


const COUNT: usize = 256;

fn main() -> io::Result<()> {
    let outdir = env::var("OUT_DIR")
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    let mut bar = fs::File::create(Path::new(&outdir).join("bar.rs"))?;

    for i in 0..COUNT {
        write!(&mut bar,
            "\
            pub struct Bar{index}(usize);\n\
            #[inline(never)]\
            pub fn bar{index}(bar: &parking_lot::RwLock<Bar{index}>) -> (std::any::TypeId, usize) {{\n\
                {{ bar.write().0 += 1; }};\n\
                (std::any::TypeId::of::<Bar{index}>(), bar.read().0)\n\
            }}\n\
            ",
            index = i
        )?;
    }

    writeln!(&mut bar, "pub fn run() -> usize {{ let mut count = 0; let mut types = std::collections::HashSet::new();")?;

    for i in 0..COUNT {
        writeln!(&mut bar, "let b{index} = parking_lot::RwLock::new(Bar{index}({index}));", index = i)?;
    }

    for i in 0..COUNT {
        writeln!(&mut bar, "let (id, i) = bar{index}(&b{index}); types.insert(id); count += i;", index = i)?;
    }

    writeln!(&mut bar, "assert_eq!(types.len(), {});", COUNT)?;
    writeln!(&mut bar, "count }}")?;

    Ok(())
}
