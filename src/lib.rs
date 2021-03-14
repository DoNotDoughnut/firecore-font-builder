use std::io::Write;

use firecore_font_lib::FontSheet;
use firecore_font_lib::FontSheetFile;


pub fn build_font(font_folder: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {

    let mut fonts = Vec::new();

    for entry in std::fs::read_dir(font_folder)? {
        if let Ok(entry) = entry {
            let file = entry.path();
            if file.is_file() {
                match std::fs::read_to_string(&file) {
                    Ok(content) => {
                        let result: Result<FontSheetFile, ron::Error> = ron::from_str(&content);
                        match result {
                            Ok(font_sheet_file) => {
                                match std::fs::read(&font_sheet_file.file) {
                                    Ok(image) => {
                                        fonts.push(FontSheet {
                                            image,
                                            data: font_sheet_file.data,
                                        });
                                    }
                                    Err(err) => {
                                        eprintln!("Could not read image file at {} specified by font sheet id {} with error {}", font_sheet_file.file, font_sheet_file.data.id, err);
                                    }
                                }
                            }
                            Err(err) => {
                                eprintln!("Could not deserialize font sheet info at {:?} with error {}", file, err);
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Could not read font sheet info at {:?} to string with error {}", file, err);
                    }
                }
            }
        }
    }    

    println!("Creating file...");
    let mut file = std::fs::File::create(output_file)?;

    println!("Serializing fonts...");
    let bytes = bincode::serialize(&firecore_font_lib::SerializedFonts {
        fonts
    })?;

    println!("Writing fonts to file...");
    let bytes = file.write(&bytes)?;
    println!("Wrote {} bytes to font file!", bytes);

    Ok(())
}