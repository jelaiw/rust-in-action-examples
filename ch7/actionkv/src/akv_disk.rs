// Although src/lib.rs exists within our project, it’s treated the same as any other crate within the src/bin.rs file.
use libactionkv::ActionKV;
use std::collections::HashMap;

// The cfg attribute allows Windows users to see the correct file extension in their help documentation.
#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    akv_disk.exe FILE get KEY
    akv_disk.exe FILE delete KEY
    akv_disk.exe FILE insert KEY VALUE
    akv_disk.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    akv_disk FILE get KEY
    akv_disk FILE delete KEY
    akv_disk FILE insert KEY VALUE
    akv_disk FILE update KEY VALUE
";

type ByteStr = [u8];
type ByteString = Vec<u8>;

fn store_index_on_disk(a: &mut ActionKV, index_key: &ByteStr) {
    a.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&a.index).unwrap();
    a.index = std::collections::HashMap::new();
    a.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
    const INDEX_KEY: &ByteStr = b"+index";

    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut store = ActionKV::open(path).expect("unable to open file");
    store.load().expect("unable to load data");

    match action {
        "get" => {
            // INDEX_KEY is an internal hidden name of the index within the database.
            // Two unwrap() calls are required because a.index is a HashMap that returns Option, and
            // values themselves are stored within an Option to facilitate possible future deletes.
            let index_as_bytes = store.get(&INDEX_KEY).unwrap().unwrap();
            let index_decoded = bincode::deserialize(&index_as_bytes);
            let index: HashMap<ByteString, u64> = index_decoded.unwrap();

            // Retrieving a value now involves fetching the index first, then identifying the correct location on disk.
            match index.get(key) {
                None => eprintln!("{:?} not found", key),
                Some(&i) => {
                    let kv = store.get_at(i).unwrap();
                    // To print values, we need to use Debug as an [u8] value contains arbitrary bytes.
                    // NOTE: String::from_utf8_lossy seems more friendly than Debug.
                    println!("{:?}", String::from_utf8_lossy(&kv.value))
                }
            }
        },

        "delete" => store.delete(key).unwrap(),

        "insert" => {
            // A future update that can be added for compatibility with Rust’s HashMap, where insert returns the old value if it exists.
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap();
            // The index must also be updated whenever the data changes.
            store_index_on_disk(&mut store, INDEX_KEY);
        },

        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap();
            // The index must also be updated whenever the data changes.
            store_index_on_disk(&mut store, INDEX_KEY);
        },

        _ => eprintln!("{}", &USAGE),
    }
}
