// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn s(arg: &str) -> () {
    string_slice(arg);
}

fn t(arg: String) -> () {
    string(arg);
}

fn main() {
    s("blue");
    t("red".to_string());
    t(String::from("hi"));
    t("rust is fun!".to_owned());
    t("nice weather".into());
    t(format!("Interpolation {}", "Station"));
    s(&String::from("abc")[0..1]);
    s("  hello there ".trim());
    t("Happy Monday!".to_string().replace("Mon", "Tues"));
    t("mY sHiFt KeY iS sTiCkY".to_lowercase());
    // vscode-config: font-family: 'HarmonyOS Sans SC' - 所以，凡是经过处理的都会变成`String`，trim除外
}
