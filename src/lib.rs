use serde::Deserialize;
use std::path::Path;

pub type FontId = u8;
pub(crate) type SizeInt = u8;

#[derive(Debug, Deserialize)]
pub struct FontSheetData {
    pub id: FontId,
    pub width: SizeInt,
    pub height: SizeInt,
    pub chars: String,
    pub custom: Vec<CustomChar>,
}

#[derive(Debug, Deserialize)]
pub struct CustomChar {
    pub id: char,
    pub width: SizeInt,
    pub height: Option<SizeInt>,
}

#[derive(Debug, Deserialize)]
pub struct FontSheet<S> {
    pub sheet: S,
    pub data: FontSheetData,
}

pub fn compile(font_folder: impl AsRef<Path>) -> Vec<FontSheet<Vec<u8>>> {
    let font_folder = font_folder.as_ref();

    std::fs::read_dir(font_folder)
        .unwrap_or_else(|err| panic!("Could not read font folder with error {}", err))
        .flatten()
        .map(|entry| entry.path())
        .flat_map(|file| {
            if file.is_file() {
                let content = std::fs::read_to_string(&file).unwrap_or_else(|err| {
                    panic!(
                        "Could not read file at {:?} to string with error {}",
                        file, err
                    )
                });
                let font_sheet_file: FontSheet<String> =
                    ron::from_str(&content).unwrap_or_else(|err| {
                        panic!("Could not parse file at {:?} with error {}", file, err)
                    });
                let sheet =
                    std::fs::read(font_folder.join(&font_sheet_file.sheet)).unwrap_or_else(|err| {
                        panic!(
                            "Could not read image file at {} for sheet #{} with error {}",
                            font_sheet_file.sheet, font_sheet_file.data.id, err
                        )
                    });
                Some(FontSheet {
                    sheet,
                    data: font_sheet_file.data,
                })
            } else {
                None
            }
        })
        .collect()
}
