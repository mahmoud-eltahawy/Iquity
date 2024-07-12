use crate::components::toasts::ToastProps;
use crate::components::tooltip::Tooltip;
use crate::contexts::{
    config::use_config,
    markdown::{use_markdown, Markdown},
    toasts::{err_modal, use_toaster},
};
use crate::icons::{AddFileIcon, RESPONSIVE_ICON_LG};
use crate::tauri::read_markdown_from_fs;
use error::UbiquityError;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(AddFileDropdown)]
pub fn add_file_dropdown() -> Html {
    let markdown_ctx = use_markdown();
    let toaster = use_toaster();

    let mut recent_files_html: Vec<Html> = Vec::new();
    let recent_files = Markdown::read_all_markdown_keys();
    recent_files.iter().for_each(|recent_file| {
        let file_name = recent_file.clone();
        let markdown_ctx = markdown_ctx.clone();
        let toaster = toaster.clone();

        let read_file = Callback::from(move |_| {
            let toaster = toaster.clone();
            let markdown_ctx = markdown_ctx.clone();
            let toaster = toaster.clone();
            let file_name = file_name.clone();
            spawn_local(async move {
                let key = file_name.clone();
                let read_file: Result<String, UbiquityError> =
                    read_markdown_from_fs(key.clone()).await;

                match read_file {
                    Ok(file) => {
                        let text = AttrValue::from(file);
                        let key = Some(key);
                        let md = Markdown::from(text, key);
                        markdown_ctx
                            .add_markdown(md.clone())
                            .unwrap_or_else(|err| err_modal(err, toaster.clone()));
                        markdown_ctx
                            .set_markdown(md)
                            .unwrap_or_else(|err| err_modal(err, toaster.clone()));
                    }
                    Err(error) => {
                        let toast = ToastProps::from(error);
                        toaster.add_toast(toast);
                    }
                }
            });
        });

        let file_name = recent_file.clone();
        let html = html! {
            <li>
                <a>
                    <div onclick={read_file}>
                    {file_name}
                    </div>
                </a>
            </li>
        };
        recent_files_html.push(html);
    });

    let mut dropdown_classes = classes!("dropdown");
    if use_config().is_mobile_ui() {
        dropdown_classes.push("dropdown-end");
    }

    html! {
        <div class={dropdown_classes}>
            <Tooltip tip={"Add File"}>
                <label id="add_file_dropdown" tabindex="0" class="btn btn-ghost">
                    <AddFileIcon classes={RESPONSIVE_ICON_LG} />
                </label>
            </Tooltip>
            <div class="dropdown-content z-[1] menu p-2 shadow bg-base-200 rounded-box w-52 lg:w-max">
                <ul tabindex="0">
                    <CreateFileBtn />
                    <AddFileBtn />
                </ul>
                if !recent_files.is_empty() {
                    <ul tabindex="0">
                        <li class="menu-title">{"Recent Files"}</li>
                        {recent_files_html}
                    </ul>
                }
            </div>
        </div>
    }
}

#[function_component(CreateFileBtn)]
pub fn create_file_btn() -> Html {
    use crate::tauri::create_new_markdown_file;

    let markdown_ctx = use_markdown();
    let toaster = use_toaster();

    let create = Callback::from(move |_| {
        let markdown_ctx = markdown_ctx.clone();
        let toaster = toaster.clone();
        spawn_local(async move {
            let save: Result<String, UbiquityError> = create_new_markdown_file().await;
            match save {
                Ok(key) => {
                    let md = Markdown::from(AttrValue::from(""), Some(AttrValue::from(key)));
                    markdown_ctx
                        .add_markdown(md.clone())
                        .unwrap_or_else(|err| err_modal(err, toaster.clone()));
                    markdown_ctx
                        .set_markdown(md.clone())
                        .unwrap_or_else(|err| err_modal(err, toaster.clone()));
                }
                Err(error) => {
                    if error != UbiquityError::no_save_path_selected() {
                        let toast = ToastProps::from(error);
                        toaster.add_toast(toast);
                    }
                }
            }
        });
    });

    html! {
        <li>
            <div onclick={create}>
                {"Create File"}
            </div>
        </li>
    }
}

#[function_component(AddFileBtn)]
pub fn add_file_btn() -> Html {
    use crate::tauri::import_markdown_file;
    use urlencoding::encode;

    let markdown = use_markdown().state();
    let encoded_md = encode(&markdown.text).to_string();

    let mut text_dl = String::from("data:attachment/text,");
    text_dl.push_str(&encoded_md);

    let markdown_ctx = use_markdown();
    let toaster = use_toaster();
    let read_from_fs = Callback::from(move |_| {
        let markdown_ctx = markdown_ctx.clone();
        let toaster = toaster.clone();
        spawn_local(async move {
            let create_file: Result<Markdown, UbiquityError> = import_markdown_file().await;
            match create_file {
                Ok(markdown) => {
                    markdown_ctx
                        .add_markdown(markdown)
                        .unwrap_or_else(|err| err_modal(err, toaster));
                }
                Err(error) => {
                    if error != UbiquityError::no_file_selected() {
                        toaster.add_toast(ToastProps::from(error));
                    }
                }
            }
        });
    });

    html! {
        <li>
            <div onclick={read_from_fs}>
                {"Import File"}
            </div>
        </li>
    }
}
