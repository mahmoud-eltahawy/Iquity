use crate::components::tooltip::Tooltip;
use crate::contexts::markdown::Markdown;
use crate::contexts::{
    markdown::use_markdown,
    toasts::{err_modal, use_toaster},
};
use crate::icons::SaveIcon;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(SaveBtn)]
pub fn save_btn() -> Html {
    use crate::{
        components::toasts::ToastProps, icons::RESPONSIVE_ICON_LG, tauri::save_markdown_to_fs,
    };
    use error::UbiquityError;
    use gloo::utils::window;

    let md_ctx = use_markdown();
    let toaster = use_toaster();
    let save_fs: Callback<MouseEvent> = Callback::from(move |_| {
        let clone = md_ctx.clone();
        let markdown = clone.state();
        let toaster = toaster.clone();
        let key = clone.state().key;
        spawn_local(async move {
            let save_as_markdown = Markdown::from(markdown.text.clone(), key);
            let path: Result<String, UbiquityError> = save_markdown_to_fs(save_as_markdown).await;
            match path {
                Ok(path) => {
                    let key = AttrValue::from(path);
                    clone.update_key(key.clone());
                }
                Err(err) => {
                    if err != UbiquityError::no_save_path_selected() {
                        let toast = ToastProps::from(err);
                        toaster.add_toast(toast);
                    }
                }
            }
        });
    });

    let md_ctx = use_markdown();
    let toaster = use_toaster();
    let save_as_fs: Callback<MouseEvent> = Callback::from(move |_| {
        let clone = md_ctx.clone();
        let markdown = clone.state();
        let toaster = toaster.clone();
        spawn_local(async move {
            let save_as_markdown = Markdown::from(markdown.text.clone(), None);
            let path: Result<String, UbiquityError> = save_markdown_to_fs(save_as_markdown).await;
            match path {
                Ok(path) => {
                    let key = Some(AttrValue::from(path));
                    let new_md = Markdown::from(markdown.text, key);
                    clone
                        .add_markdown(new_md.clone())
                        .unwrap_or_else(|err| err_modal(err, toaster.clone()));
                    clone
                        .set_markdown(new_md)
                        .unwrap_or_else(|err| err_modal(err, toaster.clone()));
                }
                Err(err) => {
                    if err != UbiquityError::no_save_path_selected() {
                        let toast = ToastProps::from(err);
                        toaster.add_toast(toast);
                    }
                }
            }
        });
    });

    let export_pdf: Callback<MouseEvent> = Callback::from(move |_| {
        window().print().unwrap();
    });

    let dropdown_classes = classes!("dropdown");

    html! {
        <div class={dropdown_classes}>
            <Tooltip tip={"Save"}>
                <label tabindex="0" class="btn btn-ghost">
                    <SaveIcon classes={RESPONSIVE_ICON_LG} />
                </label>
            </Tooltip>
            <div class="dropdown-content z-[1] menu p-2 shadow bg-base-200 rounded-box w-52 lg:w-max">
                <ul tabindex="0">
                    <li>
                        <div onclick={save_fs}>
                            {"Save"}
                        </div>
                    </li>
                    <li>
                        <div onclick={save_as_fs}>
                            {"Save As"}
                        </div>
                    </li>
                    <li>
                        <div onclick={export_pdf}>
                            {"Export as PDF"}
                        </div>
                    </li>
                </ul>
            </div>
        </div>
    }
}
