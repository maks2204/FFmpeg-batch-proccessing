use std::fs;
use std::path::PathBuf;
use std::path::Path;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::thread;
use regex::Regex;
use ffmpeg_sidecar::{command::FfmpegCommand, event::FfmpegEvent};


//static MAXTHREAD: i32 = 5;

pub struct Encoder {
    ffmpeg_path: PathBuf,
    path: PathBuf,
    files: Vec<PathBuf>,
    videos_count: i32,
    m: MultiProgress,
    sty: ProgressStyle,
    rx: Receiver<i32>,
    tx: Sender<i32>,
    command_str: String,
    max_threads: i32
}

impl Encoder {
    pub fn new(path: PathBuf, ffmpeg_path: PathBuf, ffmpeg_commands: String, max_threads: i32) -> Self {
        let files = Encoder::scan_folder(&path);
        let videos_count = i32::try_from(files.len()).unwrap();
        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        Self { 
            ffmpeg_path,
            path,
            files,
            videos_count,
            m: MultiProgress::new(),
            sty: ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",).unwrap().progress_chars("##-"),
            rx,
            tx,
            command_str: ffmpeg_commands,
            max_threads: max_threads
        }
    }


    pub fn process(&self) {
        // for i in &self.files {
        //     println!("{:?}", i);
        // }
        // print!("{:?}", self.videos_count);
        self.create_folder_finish();

        let mut curr_ind;
        if self.max_threads >= self.videos_count {
            curr_ind = self.videos_count
        } else {
            curr_ind = self.max_threads;
        }

        self.initial_spawn_threads();
        for _i in 0..self.videos_count {
            self.rx.recv().expect("ERROR RECIEVE");
            //println!("Thread Finish {}", self.rx.recv().expect("ERROR RECIEVE"));
            //println!("curr ind:{}", curr_ind);
            if curr_ind < self.videos_count {
                self.spawn(curr_ind);
                curr_ind += 1;
            }
        }
    }

    // fn stdout_process(&self) -> i32 {

    // }

    fn create_folder_finish(&self) {
        let mut folder_finish = self.path.clone();
        folder_finish.push("finish");
        if !folder_finish.exists(){
            fs::create_dir(folder_finish).unwrap();
        }
    }

    fn initial_spawn_threads(&self) {
        let initial_threads: i32;
        if self.max_threads > self.videos_count {
            initial_threads = self.videos_count;
        } else {
            initial_threads = self.max_threads;
        }
        for id in 0..initial_threads {
            self.spawn(id);
        }
    }

    fn spawn(&self, id: i32) {
        let thread_tx = self.tx.clone();
        let command = self.command_str.clone();
        let m_clone = self.m.clone();
        let pb = m_clone.add(ProgressBar::hidden());
        pb.set_style(self.sty.clone());

        let curr_file = self.files[id as usize].clone();
        let mut output_file = self.path.clone();
        output_file.push("finish");
        output_file.push(curr_file.file_name().unwrap());
        //let ffmpeg_path=self.ffmpeg_path.clone();
        let ffmpeg_path = self.ffmpeg_path.clone();
        

        thread::spawn(move || {
            let mut find = false;

            let temp = curr_file.clone();
            let filename_progress = temp.file_name().unwrap();



            FfmpegCommand::new_with_path(ffmpeg_path)
            .arg("-i")
            .arg(curr_file)
            .args(command.split(" "))
            .arg(output_file)
            .spawn()
            .unwrap()
            .iter()
            .unwrap()
            .for_each(|event: FfmpegEvent| {
                match event {
                    FfmpegEvent::Progress(progress) => {
                    pb.set_position(progress.frame as u64);
                    pb.set_message(format!("Thread ID:{} FPS:{} File Name: {}", id, progress.fps, filename_progress.to_str().unwrap()))
                    }

                    FfmpegEvent::Log(_level, msg) => {
                        if !find {
                            if let Ok(res_regex) = Encoder::process_regex(&msg){
                                    find = true;
                                    pb.set_length(res_regex.parse::<u64>().unwrap());
                            }
                        }
                    //eprintln!("[ffmpeg] {}", msg); // <- granular log message from stderr
                    }
                    _ => {}
                }
                });
    
                pb.finish_with_message(format!("End THREAD ID:{}", id));

                thread_tx.send(id).unwrap();
    
            });
    }

    fn process_regex(_str: &String) -> Result<String, &str> {
        let re = Regex::new(r"      NUMBER_OF_FRAMES(?:-eng)*: (\d+)$").unwrap();
        let Some(caps) = re.captures(&_str) else { return Err("Error") };
        //println!("{:?}", &caps[1]);
        Ok(caps[1].to_string())
    }

    fn scan_folder(path: &Path) -> Vec<PathBuf> {
        //let mut vec: Vec<PathBuf> = Vec::new();
        let mut list_files: Vec<PathBuf> = Vec::new();
        for entry in path.read_dir().expect("Read Dir Failed") {
            if let Ok(entry) = entry {
                if let Some(e) = entry.path().extension() {
                    if e == "mkv" {
                        list_files.push(entry.path());
                    }
                }
            }
        }
        list_files
    }
}