use std::fs;

pub fn get_audio_files(path: &str) -> Vec<String> {
    let entries = fs::read_dir(path).unwrap();

    let mut audio_files = vec![];

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "mp3" || extension == "wav" {
                    audio_files.push(entry.file_name().into_string().unwrap());
                }
            }
        }
    }

    audio_files
}
