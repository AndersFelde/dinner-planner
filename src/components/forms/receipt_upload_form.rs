use crate::api::receipt::scan_receipt;
use leptos::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

#[component]
pub fn ReceiptUpload() -> impl IntoView {
    let upload_action = Action::new_local(|data: &FormData| {
        // `MultipartData` implements `From<FormData>`
        let data = data.clone();
        async move {scan_receipt(data.into()).await}
    });
    let upload_submitted = upload_action.input();
    let pending = upload_action.pending();
    let upload = upload_action.value();

    view! {
        <h3>File Upload</h3>
        <p>Uploading files is fairly easy using multipart form data.</p>
        <form on:submit=move |ev: SubmitEvent| {
            ev.prevent_default();
            let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
            let form_data = FormData::new_with_form(&target).unwrap();
            upload_action.dispatch_local(form_data);
        }>
            <input type="file" name="file_to_upload" />
            <input type="submit" />
        </form>
        <p>
            {move || {
                if upload_submitted.read().is_none() && upload.read().is_none()
                {
                    "Upload a file.".to_string()
                } else if pending.get() {
                    "Uploading...".to_string()
                } else if let Some(Ok(value)) = upload.read().as_ref() {
                    format!("{:?}", value)
                } else {
                    format!("{:?}", upload.read().as_ref())
                }
            }}

        </p>
    }
}
