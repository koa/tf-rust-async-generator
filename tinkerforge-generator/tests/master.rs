use std::{
    env::current_dir,
    fs,
};

use prettyplease::unparse;
use similar::{ChangeTag, TextDiff};
use syn::File;

use tinkerforge_generator::{
    generator::generate_code,
    json_model::JsonContent,
};

#[test]
fn test_build_master() {
    let master = serde_json::from_slice::<'_, JsonContent>(include_bytes!("data/brick_master.json")).expect("Cannot parse brick_master.json");
    let lcd = serde_json::from_slice::<'_, JsonContent>(include_bytes!("data/bricklet_lcd_128x64.json")).expect("Cannot parse bricklet_lcd_128x64.json");
    let file: File = generate_code([master, lcd].into_iter());
    let dest_path = current_dir().unwrap().parent().unwrap().join("target").join("output.rs");
    let generated_code = unparse(&file);
    fs::write(dest_path.clone(), generated_code.as_str()).expect("Cannot write source file");

    let diff = TextDiff::from_lines(
        include_str!("data/output.rs"), generated_code.as_str());
    let mut unified_diff = diff.unified_diff();
    print!("{}", unified_diff.header("template", "generated"));
    let found_diff = diff.iter_all_changes().any(|change| !matches!(change.tag(), ChangeTag::Equal));
    assert!(!found_diff);
}