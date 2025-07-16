



pub fn message_start(username: &str, user_id: u64) -> String {
    let text = format!("<b>Hallo @{}</b>🇯🇵🇯🇵
    
Welcome to Telebot_Hirakata <b>
• Belajar Hiragana
• Belajar Katakana
• Kotoba
• Kanji </b>

Tekan /help untuk fitur yang tersedia ({})
", username, user_id);
    return text
}


pub fn message_help() -> String {
    let text = format!("<b>Berikut Adalah List Fitur yang Tersedia🔥🔥

/start => Start Bot
/help => Bantuan

/hiragana => Membaca Hiragana
/katakana => Membaca Katakana
/kanji => Belajar Kanji

/add (admin)
/delete (admin)</b>

Bot ini dibuat oleh @Irfan4ami, Dibuat untuk belajar, Dev tidak bertanggungjawab atas apapun pelanggaran yang disebabkan oleh user

");
    return text
}


pub fn message_admin_only() -> String {
    let text = format!("<b>Administrator Only</b>\n");
    return text
}

pub fn message_invalid_input() -> String {
    let text = format!("<b>Invalid Input</b>\n");
    return text
}

pub fn message_duplicate_data() -> String {
    let text = format!("<b>Data Sudah Ada</b>\n");
    return text
}

pub fn message_request_generate_moji(quest: &str) -> String {
    let text = format!("<b>Tolong Tulis ke Romaji</b>
    \n
🇯🇵: {}
🇮🇩: ?
", quest);
    return text
}

pub fn message_request_generate_kotoba(quest: &str, indonesia: &str, english: &str) -> String {
    let text = format!("<b>Tolong Tulis ke Romaji
    
• 🇯🇵 {}
• ⌨️ ????
• 🇮🇩 {}
• 🇬🇧 {}
</b>", quest, indonesia, english);
    return text
}

pub fn message_correct() -> String {
    let text = format!("<b>✅✅✅CORRECT✅✅✅</b>\n");
    return text
}

pub fn message_wrong() -> String {
    let text = format!("<b>❌❌❌WRONG❌❌❌</b>\n");
    return text
}



pub fn message_add(japan: &str, writing: &str, indonesia: &str, english: &str) -> String {
    let text = format!("<b>Data Berhasil Ditambahkan</b>
 <b>
• 🇯🇵 {}
• ⌨️ {}
• 🇮🇩 {}
• 🇬🇧 {}
</b>
Tekan /help untuk fitur yang tersedia /add", japan, writing, indonesia, english);
    return text
}

pub fn message_kanji() -> String {
    let text = format!("<b>Berikut Adalah List Fitur yang Tersedia🔥🔥

/start_kanji_angka
/start_kanji_waktu
</b>
Tekan /help untuk list fitur yang tersedia");
    return text
}


pub fn message_hiragana() -> String {
    let text = format!("<b>Berikut Adalah List Fitur yang Tersedia🔥🔥

/start_hiragana_moji
/start_hiragana_kotoba
</b>
Tekan /help untuk list fitur yang tersedia");
    return text
}


pub fn message_katakana() -> String {
    let text = format!("<b>Berikut Adalah List Fitur yang Tersedia🔥🔥

/start_katakana_moji
/start_katakana_kotoba
</b>
Tekan /help untuk list fitur yang tersedia");
    return text
}

pub fn message_request_generate_kanji(kanji: &str, quest: &str) -> String {
    let text = format!("<b>Tulis Terjemahan atau Romajinya

• 🇯🇵 {}
• 🇯🇵 {}
• ⌨️ ????
• 🇮🇩 ????
• 🇬🇧 ????
</b>", kanji, quest);
    return text
}