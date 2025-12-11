#[allow(unused)]
pub fn generate_hashtag2(s: &str) -> Option<String> {
    let res = format!(
        "#{}",
        s.split_whitespace().map(capitalize).collect::<String>()
    );
    if res.len() < 141 && res.len() > 1 {
        Some(res)
    } else {
        None
    }
}

#[allow(unused)]
fn capitalize(s: &str) -> String {
    let mut s = s.to_ascii_lowercase();
    s[0..1].make_ascii_uppercase();
    s
}

pub fn generate_hashtag(s: &str) -> Option<String> {
    let mut s = s.to_string();
    if s.len() > 141 || s.is_empty() {
        // поправка: в задаче РЕЗУЛЬТАТ должен быть меньше 140 символов, а не исходный текст
        return None;
    }

    s = s
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            chars
                .next()
                .map(|first| {
                    let capital = first.to_uppercase().collect::<String>();
                    let rest = chars.as_str().to_lowercase();
                    capital + &rest
                })
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join("");

    if !s.starts_with('#') {
        Some(format!("#{}", s))
    } else {
        Some(s)
    }
}

fn main() {
    println!("{:?}", "fBbeFXy degtGVcTAq bjPGdEypH yga ANQmKbNycZHBPNOGfx ARdwhNkndzalzdeynfs xKsxB AFmPUlkDVqv DRuAvynaWlKHbo Qml ReSsHBrLZL HuofKp AI LAmpChxD yBBJPTKzUvpGhD".len());
    println!(
        "{:?}",
        generate_hashtag2(
            "fBbeFXy degtGVcTAq bjPGdEypH yga ANQmKbNycZHBPNOGfx ARdwhNkndzalzdeynfs xKsxB AFmPUlkDVqv DRuAvynaWlKHbo Qml ReSsHBrLZL HuofKp AI LAmpChxD yBBJPTKzUvpGhD"
        )
    );
}
