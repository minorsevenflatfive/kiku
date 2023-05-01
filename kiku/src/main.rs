mod utils;
use utils::file_utils;

use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;

use rodio::{Decoder, OutputStream, source::Source};

fn main() {
    let dir = "/Users/nararya/workspace/kiku/kiku-rust";
    let files = file_utils::get_audio_files(dir);
    println!("Found {} audio files in the directory.", files.len());
    for file in &files {
        println!("{}", file);
    }

    // Open the audio file
    let file = File::open(dir.to_owned() + "/" + &files[0]).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    // Initialize the sink (audio player)
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(5));
}
