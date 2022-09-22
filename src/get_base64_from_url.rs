use std::str::{self, FromStr};
// 手順
// 1.urlを受け取る. 形式 : "https://aws/?img=base64データ"
// 2."https://aws/?img=" を消去.
// 3."-"を"+" に、 "_"を"/" に変換.
// 4.(4 - 文字数を4で割ったあまり)個の"="を末尾に追加.
// 5.文字列を返す.

pub fn get_bace64_from_specified_url(url: &str) -> std::string::String {
    let mut url2: String = url.replace("https://aws/?img=", "");
    let mut url3: String = url2.replace("-", "+");
    url2 = url3.replace("_", "/");

    let mut long;
    long = url2.chars().count();
    long = 4 - long;
    let mut i = 0;
    for i in 0..long {
        url2.push('=');
    }
    return url2;
}

pub fn get_base64_from_url(url: &str) -> String {
    let query_start = url.find("?").unwrap();
    let data_start = url.find("img=").unwrap();
    let data_end = url.find("&").unwrap();

    if query_start < data_start {
        return slice_str(url, data_start, data_end);
    }
    return String::new();
}

fn slice_str(s: &str, start: usize, end: usize) -> String {
    let mut ret = String::new();

    for (i, c) in s.chars().enumerate() {
        if i < start {
            continue;
        } else if end < i {
            break;
        } else {
            ret.push(c);
        }
    }
    ret
}
