/*
手順
urlを受け取る. 形式 : "https://aws/?"..."img=base64データ"...
"&"で分割
"img="を検索
ヒットした部分を取り出す
"img=" を消去.
"-"を"+" に、 "_"を"/" に変換.
(4 - 文字数を4で割ったあまり)個の"="を末尾に追加.
文字列を返す.
*/

pub fn get_bace64_from_url(url: &str) -> std::string::String {
  let mut url2: String = url.replace("https://aws/?", "");

  //＆で分割してimg=が含まれる部分を割り出す
  let v: Vec<String> = url2.split('&').fold(Vec::new(), |mut s, i| {
    s.push(i.to_string());
    s
  });
  let mut j: usize = 0;
  let mut flag: usize = 0;
  while flag != 0 {
    if v[j].find("img=") != None {
      flag = j;
    }
  }

  url2 = v.get(flag).unwrap().to_string();

  //各種置き換え
  url2 = url2.replace("img=", "");
  url2 = url2.replace("-", "+");
  url2 = url2.replace("_", "/");
  //文字数が4の倍数になるよう"="を追加
  for i in 0..4-url2.chars().count() {
      url2.push('=');
  }

	return url2;
}