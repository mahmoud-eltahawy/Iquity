use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TooltipProps {
    pub tip: AttrValue,
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub position: TooltipPosition,
}

#[derive(PartialEq, Default, Clone, Copy)]
pub enum TooltipPosition {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipProps) -> Html {
    let mut tooltip_classes = classes!("overflow-visible", "tooltip", "tooltip-info", "block");

    match &props.position {
        TooltipPosition::Top => tooltip_classes.push("tooltip-top"),
        TooltipPosition::Bottom => tooltip_classes.push("tooltip-bottom"),
        TooltipPosition::Left => tooltip_classes.push("tooltip-left"),
        TooltipPosition::Right => tooltip_classes.push("tooltip-right"),
    }

    html! {
        <div data-tip={&props.tip} class={tooltip_classes} onclick={&props.onclick}>
            { for props.children.clone() }
        </div>
    }
}
