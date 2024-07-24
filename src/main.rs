use std::{fs, path::PathBuf};
use std::env;

mod ffmpeg_batch_encoder;

fn main() {

    let ffmpeg_path = fs::read_to_string("ffmpeg.txt").expect("ERROR READ ffmpeg.txt with PATH to ffmpeg.exe");

    let args: Vec<String> = env::args().collect();
    //args[1] = "Z:\\ffmpeg\\ffmpeg-n6.1-latest-win64-gpl-6.1\\bin".to_string();

    let path_to_encode = PathBuf::from(&args[1]);

    //let path = Path::new("Z:\\ffmpeg\\ffmpeg-n6.1-latest-win64-gpl-6.1\\bin");
    //let ffmpeg_path = Path::new("Z:\\ffmpeg\\ffmpeg-n6.1-latest-win64-gpl-6.1\\bin\\ffmpeg.exe");

    let encoder = ffmpeg_batch_encoder::Encoder::new(path_to_encode, PathBuf::from(ffmpeg_path));
    encoder.process();

}