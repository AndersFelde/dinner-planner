use leptos::logging::error;
use leptos::prelude::*;
use leptos::server_fn::codec::{MultipartData, MultipartFormData};
use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::models::receipt::{ReceiptForm, ReceiptItem, ReceiptItemForm, ReceiptWithItems};

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct ReceiptResult {
    result: bool,
    lines: Option<Vec<Vec<String>>>,
    error: Option<Vec<String>>,
}

#[cfg(feature = "ssr")]
fn ocr_image(image_path: &str, db: &mut super::ssr::DbConn) -> Result<Vec<Vec<String>>, String> {
    use leptos::logging::{error, log};
    use serde_json;

    let output = match Command::new("uv")
        .arg("run")
        .arg("--project")
        .arg("ocr/")
        .arg("ocr/main.py")
        .arg(image_path)
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            error!("OCR command failed: {}", e);
            return Err(String::from("Parsing OCR image failed"));
        }
    };

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let stdout = String::from_utf8(output.stdout).map_err(|e| {
        error!("Got error while parsing string from utf8 {e}");
        String::from("Could not parse OCR output")
    })?;
    let results: ReceiptResult = serde_json::from_str(&stdout).map_err(|e| {
        error!("Got error while deserializing {e}");
        String::from("Could not parse OCR output")
    })?;

    if !results.result {
        error!("OCR command failed: {:?}", results.error);
        return Err(String::from("Parsing OCR image failed"));
    }

    Ok(results.lines.unwrap_or_default())
}

#[server(input = MultipartFormData)]
pub async fn scan_receipt(
    data: MultipartData,
) -> Result<crate::models::receipt::ReceiptWithItems, ServerFnError> {
    use crate::api::ssr::*;
    use leptos::logging::log;
    use tempfile::Builder;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

    const STORE_NAMES: [&str; 3] = ["Rema", "Coop", "Kiwi"];
    let db = &mut get_db()?;
    // Safe to unwrap
    let mut data = data.into_inner().unwrap();

    while let Ok(Some(mut field)) = data.next_field().await {
        let extension = match field.content_type().unwrap().essence_str() {
            "image/jpg" => "jpg",
            "image/png" => "png",
            "image/jpeg" => "jpeg",
            "image/bmp" => "bmp",
            "image/pdf" => "pdf",
            t => return Err(ServerFnError::new(&format!("Unsupported file type {}", t))),
        };
        log!("Got filetype {}", extension);

        // 1. Create a temporary file
        // NamedTempFile::new() creates it in the default temp dir
        let final_path: String;
        let temp_file = Builder::new()
            .suffix(&format!(".{extension}"))
            .tempfile()
            .map_err(|_| ServerFnError::new("Could not create temporary file"))?;
        let path = temp_file.path().to_owned();
        log!("created tempfile at {:?}", path);

        let (std_file, _path) = temp_file.keep().unwrap();
        let mut file = File::from_std(std_file);

        while let Ok(Some(chunk)) = field.chunk().await {
            file.write_all(&chunk).await?;
        }

        // Flush before OCR
        file.flush().await?;

        // Return the path so you can use it elsewhere
        final_path = path.to_string_lossy().into_owned();

        let lines = ocr_image(&final_path, db).map_err(ServerFnError::new)?;

        log!("Lines: {lines:?}");

        let store = lines
            .iter()
            .flat_map(|row| row.iter()) // flatten Vec<Vec<String>> → iterator of &String
            .find_map(|r| {
                // stops at first match, returns Some
                STORE_NAMES
                    .iter()
                    .find(|&&s| r.to_lowercase().contains(&s.to_lowercase())) // find keyword in STORE_NAMES
                    .copied() // turn &&str → &str
            })
            .unwrap_or("Unknown");

        let mut items = vec![];
        for words in lines {
            if let Some(price) = words.last() {
                let price = price.replace(",", ".").replace(" ", "");
                if !price.contains("."){
                    continue
                }
                if let Ok(price) = price.parse::<f32>() {
                    let words = &words[..words.len() - 1];
                    let name = words
                        .iter()
                        .filter(|word| !(word.chars().last() == Some('%') && word.len() <= 3))
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(" ");
                    if name.contains("Totalt (") || name.contains("Sum ") {
                        break;
                    }
                    if name.len() > 2 {
                        items.push((name, price));
                    }
                }
            }
        }

        let total = items.iter().map(|i| i.1).sum();

        let receipt = ReceiptForm {
            store: store.to_owned(),
            total,
            datetime: chrono::Local::now().naive_local(),
        }
        .insert(db)?;

        let receipt_id = receipt.id;

        let mut receipt_items = vec![];
        for (name, price) in items {
            receipt_items.push(
                ReceiptItemForm {
                    receipt_id,
                    name,
                    price,
                }
                .insert(db)?,
            );
        }
        // let _ = std::fs::remove_file(&final_path);
        return Ok(ReceiptWithItems {
            receipt,
            items: receipt_items,
        });
    }

    Err(ServerFnError::new("No image provided"))
}

#[cfg(feature = "ssr")]
#[cfg(test)]
mod test {
    use super::ReceiptItem;
    use crate::api::receipt::ocr_image;

    #[test]
    // TODO: create test
    pub fn test_ocr() {}
}
