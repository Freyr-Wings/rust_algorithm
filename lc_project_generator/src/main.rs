use std::{
    env, fs,
    io::{self, Read, Write},
    path::Path,
};

use toml_edit::{Array, Document, Item};

fn main() -> io::Result<()> {
    // Get the new project's name from the CLI arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <new_project_name>", args[0]);
        std::process::exit(1);
    }
    let new_project_name = &args[1];

    // Define paths
    let template_dir = Path::new("./template"); // Adjust if your template folder is elsewhere
    let new_project_dir = Path::new("..").join(new_project_name);

    // 1) Copy the entire template folder into the new folder
    copy_dir_all(template_dir, &new_project_dir)?;

    // 2) Update Cargo.toml with the new project name
    let cargo_toml_path = new_project_dir.join("Cargo.toml");
    update_cargo_toml(&cargo_toml_path, new_project_name)?;

    // 3) Add this new project to the root workspace's Cargo.toml
    //    (assuming the root workspace Cargo.toml is in `../Cargo.toml`)
    let workspace_cargo_toml_path = Path::new("..").join("Cargo.toml");
    // The path to the new crate, relative to the workspace root
    let new_member_path = new_project_name; // e.g. "my_new_project"
    add_project_to_workspace(&workspace_cargo_toml_path, new_member_path)?;

    println!("Successfully generated project: {}", new_project_name);

    Ok(())
}

/// Recursively copy a directory from `src` to `dst`.
fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    // Create the destination directory if it doesn't exist
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry_result in fs::read_dir(src)? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            // Recursively copy directories
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            // Copy files
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

/// Update `Cargo.toml` to set the package name to `new_project_name`.
/// Assumes there's a line like `name = "..."` in the `[package]` section.
fn update_cargo_toml(cargo_toml_path: &Path, new_project_name: &str) -> io::Result<()> {
    let mut contents = String::new();
    {
        let mut file = fs::File::open(cargo_toml_path)?;
        file.read_to_string(&mut contents)?;
    }

    // Replace the line name = "..."
    // This is a simple approach that assumes a single occurrence of name = "<old_name>"
    let updated_contents = contents
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("name =") {
                format!(r#"name = "{}""#, new_project_name)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut file = fs::File::create(cargo_toml_path)?;
    file.write_all(updated_contents.as_bytes())?;

    Ok(())
}

/// Add the newly created project to the `[workspace] members` array in the root Cargo.toml.
///
/// `new_member_path` is typically the relative directory name, e.g. "my_new_project".
fn add_project_to_workspace(
    workspace_cargo_toml_path: &Path,
    new_member_path: &str,
) -> io::Result<()> {
    // Read the root Cargo.toml into a string
    let old_contents = fs::read_to_string(workspace_cargo_toml_path)?;

    // Parse it with toml_edit
    let mut doc = old_contents.parse::<Document>().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("TOML parse error: {e}"))
    })?;

    // Access the [workspace] table
    // If it doesn't exist, create it
    let workspace = &mut doc["workspace"];

    // Access the "members" array under [workspace]
    // If it doesn't exist, create it
    let members = &mut workspace["members"];
    if members.is_none() {
        // Create an empty array
        *members = Item::Value(toml_edit::Value::Array(Default::default()));
    }

    // Convert to an editable array
    let arr: &mut Array = members
        .as_array_mut()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "workspace.members is not an array"))?;

    // Check if it already exists
    let already_exists = arr
        .iter()
        .any(|item| item.as_str() == Some(new_member_path));
    if !already_exists {
        // Insert the new path
        arr.push::<&str>(new_member_path.into());
    }

    // Now write it back. `to_string_in_original_order()` preserves existing
    // formatting/comments as much as possible.
    // Alternatively, `to_string_pretty()` is more "styled" but loses some original layout.
    let new_contents = doc.to_string();

    // Write the updated Cargo.toml
    fs::write(workspace_cargo_toml_path, new_contents)?;
    Ok(())
}
