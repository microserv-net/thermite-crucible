use argrust::{ArgumentDescription, Arguments, Display, Fetch, Parse};

fn main() {
    let mut args = Arguments::new(vec!["-".to_string(), "--".to_string()]);
    args.add("--install", ArgumentDescription::new().short_argument("-inst"));
    args.print_defined();
    args.parse();
    match args.raise_errors() {
        Ok(()) => {},
        Err(e) => eprintln!("Failed: {:?}", e)
    }

    if match args.is_arg("-inst") {
        Ok(b) => b,
        Err(e) => {eprint!("Failed: {:?}", e); false},
    } {
        println!("present: {}", args.fetch_single("--install").unwrap());

    }
}