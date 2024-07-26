use serde::{Deserialize, Serialize};
use std::{fs::{self, File}, io::{ErrorKind, Write, self}, process::{self}};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub ffmpeg_path: String,
    pub ffmpeg_command: String,
    pub threads: i32
}

impl Config{
    pub fn new() -> Self {
        let json_file_result = fs::read_to_string("config.json");
        let json_file = match json_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("config.json") {
                    Ok(mut fc) => {
                        let init = Config {
                            ffmpeg_path: "\\path\\to\\ffmpeg".to_string(),
                            ffmpeg_command: "-c:a copy -c:v hevc_nvenc -preset p5 -profile:v main10 -bf 4 -b_ref_mode 1 -nonref_p 1 -rc vbr -cq 23 -qmin 1 -qmax 99 -pix_fmt p010le -spatial-aq 1 -aq-strength 8 -temporal-aq 1 -maxrate 20M".to_string(),
                            threads: 4
                        };
                        fc.write(serde_json::to_string(&init).unwrap().as_bytes()).unwrap();
                        println!("Config created press any key and edit it");
                        io::stdin().read_line(&mut String::new()).unwrap();
                        process::exit(0);
                    },
                    Err(e) => panic!("Problem creating the file: {e:?}"),
                },
                other_error => panic!("Problem opening the file: {other_error:?}")
            }
        };

        

        let config: Config = serde_json::from_str(json_file.as_str()).expect("Error PARSE");
        config
    }
}