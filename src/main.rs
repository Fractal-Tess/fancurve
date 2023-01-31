use regex::Regex;
use std::{
    process::Command,
    str::{from_utf8, FromStr},
    thread,
    time::Duration,
};

fn main() {
    loop {
        let child = Command::new("nvidia-smi")
            .arg("-q")
            .arg("-d")
            .arg("temperature")
            .output()
            .expect("Failed to run nvidia-smi command");

        let out = child.stdout;

        let string = from_utf8(&out).unwrap();

        let gpu_temp = string
            .lines()
            .filter(|line| line.contains("GPU Current Temp"))
            .next()
            .unwrap();

        let regex = Regex::new(r"\D").unwrap();
        let temp_parse = regex.replace_all(gpu_temp, "");

        let current_temp =
            u32::from_str(temp_parse.as_ref()).expect("Cannot convert GPU temp to u8 int");

        let fan_speed = ((current_temp - 25).pow(3) / 2000 + 30).clamp(40, 100);

        Command::new("nvidia-settings")
            .arg("-a")
            .arg("[gpu:0]/GPUFanControlState=1")
            .arg("-a")
            .arg(format!("fan:0]/GPUTargetFanSpeed={}", fan_speed))
            .output()
            .expect("Failed to run nvidia-settings command");

        println!("Set gpu fan speed to {}%", fan_speed);

        thread::sleep(Duration::from_secs(1))
    }
}
