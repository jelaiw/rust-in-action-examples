fn main() {
    // Escapes the trailing newline character.
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        // Skips header row and lines with only whitespace.
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record // Starts with a line of text.
            .split(',') // Splits record into fields.
            .map(|field| field.trim()) // Trims whitespace of each field.
            .collect(); // Builds a collection of fields.
        
        // cfg! checks configuration at compile time.
        if cfg!(debug_assertions) {
            // eprintln! prints to standard error (stderr).
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // Attempts to parse field as a floating-point number.
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}
