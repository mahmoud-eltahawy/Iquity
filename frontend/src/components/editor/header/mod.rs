mod add;
mod bold;
mod font_decrease;
mod font_increase;
mod formatting;
mod headings;
mod image;
mod italics;
mod link;
mod quote;
mod redo;
mod table;
mod undo;
use yew::prelude::*;

use crate::components::divider::DividerYAxis;
use crate::components::editor::header::bold::BoldBtn;
use crate::components::editor::header::font_decrease::FontDecreaseBtn;
use crate::components::editor::header::font_increase::FontIncreaseBtn;
use crate::components::editor::header::headings::HeadingsDropdown;
use crate::components::editor::header::image::AddImageBtn;
use crate::components::editor::header::italics::ItalicsBtn;
use crate::components::editor::header::link::AddLinkBtn;
use crate::components::editor::header::quote::QuoteBtn;
use crate::components::editor::header::redo::RedoBtn;
use crate::components::editor::header::table::AddTableBtn;
use crate::components::editor::header::undo::UndoBtn;

pub const DESKTOP_HEADER_BTN_CLASSES: &str = "btn btn-ghost btn-xs 2xl:btn-sm";

#[derive(Debug, PartialEq, Properties)]
pub struct HeaderBtnProps {
    pub btn_classes: &'static str,
}

#[function_component(EditorHeader)]
pub fn input_header() -> Html {
    let btn_classes = DESKTOP_HEADER_BTN_CLASSES;

    html! {
        <div class="flex flex-row flex-wrap justify-end w-full">
            <UndoBtn btn_classes={btn_classes}/>
            <RedoBtn btn_classes={btn_classes}/>
            <DividerYAxis />
            <HeadingsDropdown btn_classes={btn_classes}/>
            <BoldBtn btn_classes={btn_classes}/>
            <ItalicsBtn btn_classes={btn_classes}/>
            <QuoteBtn btn_classes={btn_classes}/>
            <DividerYAxis />
            <AddLinkBtn btn_classes={btn_classes}/>
            <AddImageBtn btn_classes={btn_classes}/>
            <AddTableBtn btn_classes={btn_classes}/>
            <DividerYAxis />
            <FontDecreaseBtn btn_classes={btn_classes}/>
            <FontIncreaseBtn btn_classes={btn_classes}/>
        </div>
    }
}
