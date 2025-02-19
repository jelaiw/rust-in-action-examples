// Although src/lib.rs exists within our project, it’s treated the same as any other crate within the src/bin.rs file.
use libactionkv::ActionKV;

// The cfg attribute allows Windows users to see the correct file extension in their help documentation.
#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY
    akv_mem.exe FILE insert KEY VALUE
    akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut store = ActionKV::open(path).expect("unable to open file");
    store.load().expect("unable to load data");

    match action {
        "get" => match store.get(key).unwrap() {
            None => eprintln!("{:?} not found", key),
            // println! needs to use the Debug syntax ({:?}) because [u8] contains arbitrary bytes and doesn’t implement Display.
            Some(value) => println!("{:?}", value),
        },

        "delete" => store.delete(key).unwrap(),

        "insert" => {
            // A future update that can be added for compatibility with Rust’s HashMap, where insert returns the old value if it exists.
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap()
        },

        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap()
        },

        _ => eprintln!("{}", &USAGE),
    }
}
