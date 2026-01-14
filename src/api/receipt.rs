use leptos::prelude::*;
use leptos::server_fn::codec::{MultipartData, MultipartFormData};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct ReceiptResult {
    result: bool,
    text: Option<Vec<String>>,
    error: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ReceiptItem {
    name: String,
    tax: String,
    price: f64,
}

#[cfg(feature = "ssr")]
fn ocr_image(image_path: &str) -> Result<Vec<ReceiptItem>, String> {
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

    log!("Ocr cmd: {:?}", output);
    let stdout =
        String::from_utf8(output.stdout).map_err(|_| String::from("Could not parse OCR output"))?;
    let results: ReceiptResult =
        serde_json::from_str(&stdout).map_err(|_| String::from("Could not parse OCR output"))?;

    if !results.result {
        error!("OCR command failed: {:?}", results.error);
        return Err(String::from("Parsing OCR image failed"));
    }

    let text = results.text.unwrap_or_default();

    // Find first instance of percentage, item before is the first item
    let p_index = text
        .iter()
        .position(|t| t.contains("%"))
        .ok_or(String::from("Could not parse OCR output"))?;
    let first_item_index = p_index - 1;

    // Sum x varer is after all items
    let sum_index = text
        .iter()
        .position(|t| t.contains("Sum ") && t.contains(" varer"))
        .ok_or(String::from("Could not parse OCR output"))?;

    if (sum_index - first_item_index) % 3 != 0 {
        return Err(String::from("Could not parse OCR output"));
    }
    let mut parsed_items: Vec<ReceiptItem> = vec![];

    let raw_items = &text[first_item_index..sum_index];

    for item in raw_items.chunks_exact(3) {
        parsed_items.push(ReceiptItem {
            name: item[0].clone(),
            tax: item[1].clone(),
            price: item[2].replace(",", ".").parse().unwrap(),
        })
    }
    Ok(parsed_items)
}

#[server(input = MultipartFormData)]
pub async fn scan_receipt(data: MultipartData) -> Result<Vec<ReceiptItem>, ServerFnError> {
    use leptos::logging::log;
    use tempfile::Builder;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

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

        let result = ocr_image(&final_path).map_err(ServerFnError::new);

        let _ = std::fs::remove_file(&final_path);
        return result;
    }

    Err(ServerFnError::new("No image provided"))
}

#[cfg(feature = "ssr")]
#[cfg(test)]
mod test {
    use super::ReceiptItem;
    use crate::api::receipt::ocr_image;

    #[test]
    pub fn test_ocr() {
        let valid_items = vec![
            ReceiptItem {
                name: String::from("SOLSIKKEOLJE 1L ELDORAD"),
                tax: String::from("15%"),
                price: 37.9,
            },
            ReceiptItem {
                name: String::from("HAVRE KNEKKEBROD"),
                tax: String::from("15%"),
                price: 33.5,
            },
            ReceiptItem {
                name: String::from("BEREPOSE PLAST KIWI"),
                tax: String::from("25%"),
                price: 6.75,
            },
            ReceiptItem {
                name: String::from("AGURKER SKIVEDE NORA"),
                tax: String::from("15%"),
                price: 29.9,
            },
            ReceiptItem {
                name: String::from("PEANUTBUTTER 35OG MILLS"),
                tax: String::from("15%"),
                price: 39.9,
            },
            ReceiptItem {
                name: String::from("OPPHOGDE POTTETER"),
                tax: String::from("15%"),
                price: 47.9,
            },
            ReceiptItem {
                name: String::from("BURGERBROD BRIOCHE"),
                tax: String::from("15%"),
                price: 38.9,
            },
            ReceiptItem {
                name: String::from("TRIPPEL OMEGA-3 144STK"),
                tax: String::from("15%"),
                price: 89.9,
            },
        ];
        let items = ocr_image("ocr/receipt.jpg").unwrap();
        assert_eq!(items, valid_items);
    }
}
