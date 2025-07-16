use std::path::Path;
use std::fs;
use std::fs::{File, OpenOptions};
use serde::{Serialize, Deserialize};
use rand::{Rng};
use serde_json::{Value};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::collections::HashMap;
//use teloxide::types::ChatId;
use teloxide::types::MessageId;

const HIRAGANA_MOJI: &str = include_str!("file/hiragana_moji.json");
const KATAKANA_MOJI: &str = include_str!("file/katakana_moji.json");
const HIRAGANA: &str = include_str!("file/hiragana.json");

#[derive(Serialize, Deserialize)]
struct setting {
    token: String,
    administrator: bool,
}



#[derive(Serialize, Deserialize)]
struct log_user {
    learning: String,
    question: String,
    answer: String,
    indonesia: String,
    english: String,
    message_id: String,
}


#[derive(Serialize, Deserialize)]
struct kotoba {
    japan: String,
    writing: String,
    indonesia: String,
    english: String,
}

#[derive(Serialize, Deserialize)]
struct moji {
    moji: String,
    writing: String,
}

pub fn check_file(filename: &str) -> bool {
    let file_path = filename;
    if !Path::new(&file_path).exists() {
        return false;
    } else {
//        let data = fs::read_to_string(file_path).expect("Gagal membaca file");
        return true;
    }
}

pub fn check_folder(folder_name: &str) {
    let dir = Path::new(folder_name);
    if dir.exists() {
    } else {
        fs::create_dir_all(dir).unwrap();
    }
}


pub fn check_setting(filename: &str) {
    let check = check_file(filename);
    if check {
    } else {
        let setting = setting {
        token: "0000".to_string(),
        administrator: false,
        };
        let json_data = serde_json::to_string_pretty(&setting).unwrap();
        let mut file = File::create(filename).unwrap();
        file.write_all(json_data.as_bytes()).unwrap();
    }
}


pub fn check_user(filename: &str, last_learning: &str, question: &str, answer: &str, indonesia: &str, english: &str) {
        let mut message_id = String::new();
        if let Ok(mut file) = File::open(filename) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            if let Ok(existing_user) = serde_json::from_str::<log_user>(&contents) {
                message_id = existing_user.message_id;
            }
        }
        let user = log_user {
        learning: last_learning.to_string(),
        question: question.to_string(),
        answer: answer.to_string(),
        indonesia: indonesia.to_string(),
        english: english.to_string(),
        message_id,
        };
        let json_data = serde_json::to_string_pretty(&user).unwrap();
        let mut file = File::create(filename).unwrap();
        file.write_all(json_data.as_bytes()).unwrap();
}

pub fn input_string(text: &str) -> String {
    let mut input = String::new();
    input.clear();
    print!("{}", text);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("failed to readline");
    input.trim().to_string()
}


pub fn get_token() -> String {
    let mut file = File::open("setting.json").expect("Gagal membuka file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Gagal membaca isi file");
    let mut setting: setting = serde_json::from_str(&contents).expect("Gagal parse JSON");
    if setting.token != "0000" {
        return setting.token
    } else {
        let new_token = input_string("Input Token Telegram: ");
        setting.token = new_token.to_string();
        let new_json = serde_json::to_string_pretty(&setting).expect("Gagal ubah ke JSON");
        let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("setting.json")
        .expect("Gagal membuka file untuk ditulis");
        file.write_all(new_json.as_bytes()).expect("Gagal menulis ke file");
        println!("Token berhasil diubah menjadi: {}", new_token);
        return new_token;
    }
}


pub fn validate_answer(user_id: u64, answer: &str) -> (bool, String) {
    let mut file = File::open(format!("cache/{}.json", user_id)).expect("Gagal membuka file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Gagal membaca isi file");
    let log_user: log_user = serde_json::from_str(&contents).expect("Gagal parse JSON");
    let parts: Vec<&str> = log_user.answer.split('/').collect();
    let is_valid = parts.contains(&answer);
    (is_valid, log_user.learning)
}

pub fn generate_hiragana_moji() -> (String, String, String, String, String) {
    let mut rng = rand::rng();
    let rand = rng.random_range(0..=26);
    let mut rng2 = rand::rng();
    let rand2 = rng2.random_range(0..=4);
    let json: Value = serde_json::from_str(HIRAGANA_MOJI).unwrap();
    let hasil = &json["data"]["category_id"][rand]["data"];
    let moji= hasil[rand2]["moji"].as_str().unwrap_or(hasil[0]["moji"].as_str().unwrap());
    let writing= hasil[rand2]["writing"].as_str().unwrap_or(hasil[0]["writing"].as_str().unwrap());
    (moji.to_string(), writing.to_string(), "".to_string(), "".to_string(), "".to_string())
}


pub fn generate_katakana_moji() -> (String, String, String, String, String) {
    let mut rng = rand::rng();
    let rand = rng.random_range(0..=26);
    let mut rng2 = rand::rng();
    let rand2 = rng2.random_range(0..=4);
    let json: Value = serde_json::from_str(KATAKANA_MOJI).unwrap();
    let hasil = &json["data"]["category_id"][rand]["data"];
    let moji= hasil[rand2]["moji"].as_str().unwrap_or(hasil[0]["moji"].as_str().unwrap());
    let writing= hasil[rand2]["writing"].as_str().unwrap_or(hasil[0]["writing"].as_str().unwrap());
    (moji.to_string(), writing.to_string(), "".to_string(), "".to_string(), "".to_string())
}

pub fn generate_hiragana_kotoba() -> (String, String, String, String, String) {
    let content = fs::read_to_string("kotoba.json").unwrap_or_else(|_| "[]".to_string());
    let json: Value = serde_json::from_str(&content).unwrap_or(Value::Null);
    let array = json.as_array().expect("Data JSON harus berupa array");
    let total = array.len();
    if total == 0 {
        panic!("Data kosong di kotoba.json");
    }
    let mut rng = rand::thread_rng();
    let rand_index = rng.gen_range(0..total);
    let item = &array[rand_index];
    let japan = item["japan"].as_str().unwrap_or("").to_string();
    let writing = item["writing"].as_str().unwrap_or("").to_string();
    let indonesia = item["indonesia"].as_str().unwrap_or("").to_string();
    let english = item["english"].as_str().unwrap_or("").to_string();
    (japan, writing, indonesia, english, "".to_string())
}

pub fn generate_kanji_angka() -> (String, String, String, String, String) {
    let content = fs::read_to_string("numbers.json").unwrap_or_else(|_| "[]".to_string());
    let json: Value = serde_json::from_str(&content).unwrap_or(Value::Null);
    let array = json.as_array().expect("Data JSON harus berupa array");
    let total = array.len();
    if total == 0 {
        panic!("Data kosong di numbers.json");
    }
    let mut rng = rand::thread_rng();
    let rand_index = rng.gen_range(0..total);
    let item = &array[rand_index];
    let kanji = item["kanji"].as_str().unwrap_or("").to_string();
    let romaji = item["romaji"].as_str().unwrap_or("").to_string();
    let furigana = item["furigana"].as_str().unwrap_or("").to_string();
    let indonesia = item["indonesia"].as_str().unwrap_or("").to_string();
    let english = item["english"].as_str().unwrap_or("").to_string();
    (kanji, romaji, furigana, indonesia, english)
}

pub fn generate_kanji_waktu() -> (String, String, String, String, String) {
    let content = fs::read_to_string("time.json").unwrap_or_else(|_| "[]".to_string());
    let json: Value = serde_json::from_str(&content).unwrap_or(Value::Null);
    let array = json.as_array().expect("Data JSON harus berupa array");
    let total = array.len();
    if total == 0 {
        panic!("Data kosong di numbers.json");
    }
    let mut rng = rand::thread_rng();
    let rand_index = rng.gen_range(0..total);
    let item = &array[rand_index];
    let kanji = item["kanji"].as_str().unwrap_or("").to_string();
    let romaji = item["romaji"].as_str().unwrap_or("").to_string();
    let furigana = item["furigana"].as_str().unwrap_or("").to_string();
    let indonesia = item["indonesia"].as_str().unwrap_or("").to_string();
    let english = item["english"].as_str().unwrap_or("").to_string();
    (kanji, romaji, furigana, indonesia, english)
}


pub fn add_list_kotoba(japan: &str, indonesia: &str, english: &str) -> String{
    let path = "kotoba.json";
    let mut data_list = read_data(path);
    let writing = validate_kotoba_hiragana(&japan);
    let new_kotoba = kotoba {
        japan: japan.to_string(),
        writing: writing.to_string(),
        indonesia: indonesia.to_string(),
        english: english.to_string()
    };
    data_list.push(new_kotoba);
    write_data(path, &data_list);
    return writing
}

fn read_data(path: &str) -> Vec<kotoba> {
    if !Path::new(path).exists() {
        return vec![];
    }

    let file = File::open(path).expect("Gagal membuka file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|_| vec![])
}

fn write_data(path: &str, data: &[kotoba]) {
    let file = File::create(path).expect("Gagal menulis file");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data).expect("Gagal menulis JSON");
}


pub fn validate_kotoba_hiragana(japan: &str) -> String {
    let gana_list: Vec<moji> = serde_json::from_str(HIRAGANA).expect("Gagal parse JSON");
    let gana_map: HashMap<_, _> = gana_list
        .into_iter()
        .map(|k| (k.moji.clone(), k.writing.clone()))
        .collect();

    let chars: Vec<char> = japan.chars().collect();
    let mut result = String::new();
    let mut i = 0;

    while i < chars.len() {
        let current = chars[i];
        if i + 1 < chars.len() {
            let next = chars[i + 1];
            if next == 'ゃ' || next == 'ゅ' || next == 'ょ' {
                // Gabungkan current + next (misal: き + ゅ = きゅ)
                let combined = format!("{}{}", current, next);
                if let Some(writing) = gana_map.get(&combined) {
                    result.push_str(writing);
                    i += 2;
                    continue;
                }
            }
        }
        let current_str = current.to_string();
        match gana_map.get(&current_str) {
            Some(writing) => result.push_str(writing),
            None => {
                println!("Karakter '{}' tidak ditemukan dalam data", current_str);
                result.push_str(&current_str);
            }
        }
        i += 1;
    }

    result
}


pub fn get_super_admin() -> bool {
    let file_content = fs::read_to_string("setting.json").unwrap();
    let json: Value = serde_json::from_str(&file_content).unwrap();
    json["administrator"].as_bool().unwrap()
}


pub fn check_duplicate_data(japan_input: &str, filename: &str) -> bool {
    let content = fs::read_to_string(filename).unwrap_or("[]".to_string());
    let parsed: Value = serde_json::from_str(&content).unwrap_or(Value::Null);
    let binding = vec![];
    let array = parsed.as_array().unwrap_or(&binding);
    for item in array {
        if let Some(japan_value) = item.get("japan") {
            if japan_value == japan_input {
                return false;
            }
        }
    }

    true
}


pub fn update_message_id(user_id: u64, new_message_id: &str) {
    let path = format!("cache/{}.json", user_id);
    let mut file = File::open(&path).expect("Gagal membuka file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Gagal membaca file");
    let mut log_user: log_user = serde_json::from_str(&contents).expect("Gagal parse JSON");
    log_user.message_id = new_message_id.to_string();
    let updated_json = serde_json::to_string_pretty(&log_user).expect("Gagal serialize JSON");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .expect("Gagal membuka file untuk menulis");
    file.write_all(updated_json.as_bytes()).expect("Gagal menulis file");
}

pub fn get_log_message_id(user_id: u64) -> MessageId {
    let mut file = File::open(format!("cache/{}.json", user_id)).expect("Gagal membuka file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Gagal membaca isi file");
    let mut log: log_user = serde_json::from_str(&contents).expect("Gagal parse JSON");
    let message_id_num: i32 = log.message_id.parse().unwrap_or(1);
    let message_id = MessageId(message_id_num);
    message_id
}