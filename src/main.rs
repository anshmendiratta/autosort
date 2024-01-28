use std::{
    fs::{metadata, read_dir, rename, Metadata},
    path::Path,
};

// hello world
fn read_all_file_names() -> Vec<String> {
    // Enter the path of your directory
    let absolute_path = Path::new("");
    let files = read_dir(absolute_path).unwrap();

    files
        .into_iter()
        .filter(|file| !Metadata::is_dir(&metadata(file.as_ref().clone().unwrap().path()).unwrap()))
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<String>>()
}

fn get_file_extension(file: String) -> Option<String> {
    let dot_index = file.find(".");
    match dot_index {
        Some(index) => Some(file[index..].to_owned()),
        None => return None,
    }
}

fn sort_files(files: Vec<String>) {
    let folders = ["ened", "cs", "math", "chem", "cheml", "coop"];
    for file in files {
        let underscore_index = file.find("_");
        let dot_index = file.find('.').unwrap_or_else(|| {
            println!("FAILED AT {:?}", file);
            return 1;
        });
        if let Some(index) = underscore_index {
            if folders.contains(&&file[0..index]) {
                let folder = &file[0..index];
                place_file_in_dir(
                    file.clone()[0..dot_index].to_string(),
                    get_file_extension(file.clone()).unwrap(),
                    folder.into(),
                )
            };
        }
    }
}

fn place_file_in_dir(file: String, extension: String, folder: String) {
    // Before the `{}{}`, enter the path of your parent directory
    let path_string = format!(
        "{}{}",
        file, extension
    );
    let move_string = format!(
    // Before the `{}/{}{}`, enter the path of your parent directory
        "{}/{}{}",
        folder, file, extension,
    );
    rename(&path_string, &move_string).unwrap();
}

fn main() {
    let files = read_all_file_names();
    print!("Sorting `assignments/`... ");
    sort_files(files);
    println!("Finished");
}

// TODO: Add ability to read for sorting folder attribute from files
