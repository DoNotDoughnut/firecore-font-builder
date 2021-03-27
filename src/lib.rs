use std::io::Write;
use std::path::Path;

use error::FontError;
use firecore_font_lib::FontSheet;
use firecore_font_lib::FontSheetFile;

pub mod error;


pub fn compile<P: AsRef<Path>>(font_folder: P, output_file: P) -> Result<(), FontError> {
    let font_folder = font_folder.as_ref();
    let output_file = output_file.as_ref();

    let mut fonts = Vec::new();

    for entry in std::fs::read_dir(font_folder)? {
        let file = entry?.path();
        if file.is_file() {
            let content = std::fs::read_to_string(&file)?;
            let font_sheet_file: FontSheetFile = ron::from_str(&content).map_err(|err| FontError::ParseError(file.to_string_lossy().to_string(), err))?;
            let image = std::fs::read(&font_sheet_file.file)?;
            fonts.push(FontSheet {
                image,
                data: font_sheet_file.data,
            });
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