pub mod add_image;
pub mod add_link;
pub mod create_file;
pub mod file_name;
pub mod table;
pub mod utils;

use yew::prelude::*;
use {
    add_image::AddImageModal, add_link::AddLinkModal, create_file::CreateFileModal,
    file_name::SelectNameModal, table::TableModal,
};

#[function_component(Modals)]
pub fn modals() -> Html {
    html! {
        <>
            <CreateFileModal />
            <SelectNameModal />
            <AddLinkModal />
            <AddImageModal />
            <TableModal />
        </>
    }
}
