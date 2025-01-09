fn main() {
    let hello = String::from("/tmp/hello.txt");
    let tmp_dir = hello.split("/").nth(0);
    println!("{:?}", tmp_dir);
}