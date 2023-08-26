include!(concat!(env!("OUT_DIR"), "/templates.rs"));

fn main() {
    let mut buf = Vec::new();
    templates::hello_args_html(&mut buf, "World").unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
}
