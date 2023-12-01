#[derive(Debug)]
enum Lang {
  English,
  Spanish,
  Chinese,
  Texan,
  Russian,
  Ukranian,
  Romanian,
}

struct Greeting {
    message: String,
    lang: Lang,
}

fn main() {
  let mut v :Vec<Greeting> = Vec::new();

  let g : Greeting = Greeting { lang: Lang::English, message: String::from("Hello WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Spanish, message: String::from("Hola WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Texan, message: String::from("Howdy WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Chinese, message: String::from("WasmEdge 你好!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Russian, message: String::from("Привет, WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Ukranian, message: String::from("Привiт, WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Romanian, message: String::from("Noroc, WasmEdge!") };
  v.push(g);

  for e in v {
    println!("{:?} {}", e.lang, e.message);
  }
}
