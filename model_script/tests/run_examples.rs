use model_script::DSLCAD;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

#[test]
fn test_can_run_examples() {
    let mut examples = Vec::new();
    visit_dirs(Path::new("../examples"), &mut examples).expect("cant read examples");

    assert_ne!(0, examples.len());
    for example in examples {
        let mut cad = DSLCAD::default();
        cad.render_file(example.path().to_str().unwrap())
            .expect(&format!("cant render {:?}", example));
    }
}

fn visit_dirs(dir: &Path, vec: &mut Vec<DirEntry>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, vec)?;
            } else {
                vec.push(entry);
            }
        }
    }
    Ok(())
}
