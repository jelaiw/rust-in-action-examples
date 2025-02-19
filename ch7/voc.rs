use std::collections::BTreeMap;

fn main() {
    let mut voc = BTreeMap::new();

    voc.insert(1_300_405, "Middelburg");
    voc.insert(3_697_915, "Amsterdam");
    voc.insert(  173_000, "Rotterdam");
    voc.insert(  469_400, "Delft");
    voc.insert(  266_868, "Hoorn");
    voc.insert(  540_000, "Enkhuizen");

    for (guilders, kamer) in &voc {
        println!("{} invested {}", kamer, guilders);
    }

    print!("smaller chambers: ");
    // BTreeMap lets you select a portion of the keys that are iterated through with the range syntax.
    for (_guilders, kamer) in voc.range(0..500_000) {
        print!("{} ", kamer)
    }
    println!("");
}
