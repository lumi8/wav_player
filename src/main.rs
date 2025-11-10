use rodio::{Decoder, OutputStream, Sink};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main() {
    // Get the WAV file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_wav_file>", args[0]);
        return;
    }
    let wav_path = Path::new(&args[1]);

    // Open the WAV file
    let file = match File::open(wav_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return;
        }
    };

    // Create a buffer reader for the file
    let reader = BufReader::new(file);

    // Decode the WAV file
    let source = match Decoder::new(reader) {
        Ok(source) => source,
        Err(e) => {
            eprintln!("Failed to decode WAV file: {}", e);
            return;
        }
    };

    // Create an output stream to play the audio
    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Failed to create output stream: {}", e);
            return;
        }
    };

    // Create a sink to control the audio playback
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Append the audio source to the sink
    sink.append(source);

    // Play the audio
    sink.sleep_until_end();
}
