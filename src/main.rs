

use std::{collections::HashMap, fs::*, io::{self, Write}, env, path::PathBuf, str::FromStr};
use once_cell::sync::Lazy;
use hash40::Hash40;

static mut ARC_HASHES: Lazy<HashMap<u64, String>> = Lazy::new(|| HashMap::new());
static mut PARAM_HASHES: Lazy<HashMap<u64, String>> = Lazy::new(|| HashMap::new());
static mut NUS3AUDIO_HASHES: Lazy<HashMap<u64, String>> = Lazy::new(|| HashMap::new());
static mut CUSTOM_HASHES: Lazy<HashMap<u64, String>> = Lazy::new(|| HashMap::new());

fn inner_main() -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    Ok(dir)
}

fn parse_text_file(file_path: &PathBuf, hashmap: &mut HashMap<u64, String>){
    if !file_path.exists() {
        return;
    }

    let text_file_string = read_to_string(file_path).unwrap();
    for line in text_file_string.split("\n") {
        let hash = Hash40::from_str(line).unwrap().0;
        if hashmap.contains_key(&hash){
            continue;
        }
        hashmap.insert(hash, line.to_string());
    }
}

fn parse_label_file(file_path: &PathBuf, hashmap: &mut HashMap<u64, String>){
    if !file_path.exists() {
        return;
    }

    let text_file_string = read_to_string(file_path).unwrap();
    for line in text_file_string.split("\n") {
        let (hash, value) = line.split_at(line.find(',').unwrap());
        let hash = u64::from_str_radix(hash.trim_start_matches("0x"), 16).unwrap();
        let value = &value[1..];
        if hashmap.contains_key(&hash){
            continue;
        }
        hashmap.insert(hash, value.to_string());
    }
}

unsafe fn load_hashes(){
    let current_exe_path = inner_main().unwrap();

    let mut hashes_all_path = current_exe_path.clone();
    hashes_all_path.push(PathBuf::from("./Hashes/Hashes_all.txt"));
    parse_text_file(&hashes_all_path, &mut *ARC_HASHES);
    
    let mut param_labels_path = current_exe_path.clone();
    param_labels_path.push(PathBuf::from("./Hashes/ParamLabels.csv"));
    parse_label_file(&param_labels_path, &mut *PARAM_HASHES);
    
    let mut nus3audio_hashes_path = current_exe_path.clone();
    nus3audio_hashes_path.push(PathBuf::from("./Hashes/tone_names.txt"));
    parse_text_file(&nus3audio_hashes_path, &mut *NUS3AUDIO_HASHES);
    
    let mut custom_hashes_path = current_exe_path.clone();
    custom_hashes_path.push(PathBuf::from("./Hashes/custom_hashes.txt"));
    parse_label_file(&custom_hashes_path, &mut *CUSTOM_HASHES);
}

fn is_numeric(string: &String) -> bool {
    string.parse::<u64>().is_ok()
}

fn find_hash(hash: &u64){
    unsafe {
        let mut found = false;
        if ARC_HASHES.contains_key(&hash){
            found = true;
            println!("ARC Hash: {}", ARC_HASHES.get(&hash).unwrap());
        }
    
        if PARAM_HASHES.contains_key(&hash){
            found = true;
            println!("Param Hash: {}", PARAM_HASHES.get(&hash).unwrap());
        }
        
        if NUS3AUDIO_HASHES.contains_key(&hash){
            found = true;
            println!("N3A Tone Name Hash: {}", NUS3AUDIO_HASHES.get(&hash).unwrap());
        }
        
        if CUSTOM_HASHES.contains_key(&hash){
            found = true;
            println!("Custom Hash: {}", CUSTOM_HASHES.get(&hash).unwrap());
        }

        if found == false {
            println!("Hash could not be found!");
        }
    }
}

fn hash_stored(hash: &u64) -> bool {
    unsafe {
        return ARC_HASHES.contains_key(hash) ||
                PARAM_HASHES.contains_key(hash) ||
                NUS3AUDIO_HASHES.contains_key(hash) ||
                CUSTOM_HASHES.contains_key(hash)
    }
}

fn add_custom_hash(hash: u64, value: String) -> bool {
    if value.contains("\n") {
        return false;
    }

    unsafe {
        CUSTOM_HASHES.insert(hash, value);
        let mut output = String::new();
        for (k, v) in CUSTOM_HASHES.iter() {
            output.push_str(&format!("{:#x},{}\n", k, v));
        }

        let mut custom_hashes_path = inner_main().unwrap();
        custom_hashes_path.push(PathBuf::from("./Hashes/custom_hashes.txt"));

        write(custom_hashes_path, output.trim()).unwrap();

        return true;
    }
}

fn run_main(){
    let mut line;
    loop {
        print!("Lookup/Convert: ");
        io::stdout().flush().unwrap();
        line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.starts_with("0x") || is_numeric(&line){
            let hash;
            if line.starts_with("0x") {
                hash = u64::from_str_radix(line.trim_start_matches("0x"), 16).unwrap();
            } else {
                hash = u64::from_str(&line).unwrap();
            }
            find_hash(&hash);
        } else {
            let hash = Hash40::from_str(&line).unwrap().0;
            println!("Hash of {}: {:#x}", line, hash);
            if !hash_stored(&hash) {
                add_custom_hash(hash, line);
            }
        }
        println!("");
    }
}

fn main() {
    unsafe {
        load_hashes();
        run_main();
    }
}