use rfd::FileDialog;
use std::fs;

mod create_compressed;
fn main() {

    let file = FileDialog::new()
        .pick_file();

    match file {
        Some(path) => {

            let bytes = fs::read(&path)
                .expect("Error reading file");

            let mut compressed: Vec<String> = Vec::new();

            let mut count:i32 = 1;

            for i in 0..bytes.len(){

                if i == (bytes.len() - 1) {
                    compressed.push(format!("{}", bytes[i]));
                    break;
                }

                if bytes[i] == bytes[i + 1] {
                    count += 1;
                }

                if count > 1 {
                    if bytes[i] != bytes[i + 1] {

                        let value = format!("{}*{}", bytes[i], count);
                        count = 1;
                        compressed.push(value);

                    }
                } else if bytes[i] != bytes[i + 1] {
                    compressed.push(bytes[i].to_string());
                }

            }

            println!("Compressed array: {}", compressed.len());
            println!("Original array: {}", bytes.len());

            create_compressed::create_compressed();

        }
        None => {
            println!("No path chose");
        }
    }

}
