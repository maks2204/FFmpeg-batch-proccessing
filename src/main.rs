use std::path::PathBuf;
use std::env;

use json_structs::Config;

mod ffmpeg_batch_encoder;
mod json_structs;

fn main() {

    let config: Config = Config::new();
    println!("Config {:?}", config.ffmpeg_path);


    //let ffmpeg_path = fs::read_to_string("ffmpeg.txt").expect("ERROR READ ffmpeg.txt with PATH to ffmpeg.exe");

    let args: Vec<String> = env::args().collect();
    //args[1] = "Z:\\ffmpeg\\ffmpeg-n6.1-latest-win64-gpl-6.1\\bin".to_string();

    let path_to_encode = PathBuf::from(&args[1]);

    //let path = Path::new("Z:\\ffmpeg\\ffmpeg-n6.1-latest-win64-gpl-6.1\\bin");
    //let ffmpeg_path = Path::new("Z:\\ffmpeg\\ffmpeg-n6.1-latest-win64-gpl-6.1\\bin\\ffmpeg.exe");

    //let ffmpeg_command = "-c:a copy -c:v hevc_nvenc -preset p5 -profile:v main10 -bf 4 -b_ref_mode 1 -nonref_p 1 -rc vbr -cq 23 -qmin 1 -qmax 99 -pix_fmt p010le -spatial-aq 1 -aq-strength 8 -temporal-aq 1 -maxrate 20M".to_string();

    let encoder = ffmpeg_batch_encoder::Encoder::new(path_to_encode, PathBuf::from(config.ffmpeg_path), config.ffmpeg_command, config.threads);
    encoder.process();

}