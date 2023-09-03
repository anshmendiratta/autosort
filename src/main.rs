use std::{
    fs::{metadata, read_dir, rename, Metadata},
    path::Path,
};

use notify::{event::CreateKind, EventKind, RecursiveMode, Watcher};

fn read_all_file_names() -> Vec<String> {
    let absolute_path = Path::new("/Users/anshmendiratta/Desktop/assignments/");
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
    let folders = [
        "ened",
        "cs",
        "math",
        "phys",
        "physl",
        "publicspeaking",
        "collegeculture",
    ];
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
    let path_string = format!(
        "/Users/anshmendiratta/Desktop/assignments/{}{}",
        file, extension
    );
    let move_string = format!(
        "/Users/anshmendiratta/Desktop/assignments/{}/{}{}",
        folder, file, extension,
    );
    // println!("MOVING {} TO {}", &path_string, &move_string);
    rename(&path_string, &move_string).unwrap();
}

fn main() {
    let mut watcher = notify::recommended_watcher(|event: Result<notify::Event, notify::Error>| {
        if event.unwrap().kind == EventKind::Create(CreateKind::File) {
            let files = read_all_file_names();
            print!("Sorting... ");
            sort_files(files);
            println!("Finished");
        }
    })
    .unwrap();

    watcher
        .watch(
            Path::new("/Users/anshmendiratta/Desktop/assignments/"),
            RecursiveMode::NonRecursive,
        )
        .unwrap();
}
