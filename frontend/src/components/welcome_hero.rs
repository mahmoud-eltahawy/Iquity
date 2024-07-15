use yew::prelude::*;

#[function_component(WelcomeHero)]
pub fn welcome_hero() -> Html {
    let classes = classes!(
        "w-screen",
        "flex",
        "flex-col",
        "justify-center",
        "justify-center",
        "w-screen",
        "min-h-screen"
    );

    html! {
        <div class={classes}>
            <div class="flex flex-col items-center space-y-14">
                <h1
                    class="text-6xl md:text-7xl lg:text-8xl xl:text-8xl 2xl:text-8xl 3xl:text-8xl font-bold font-display tracking-wide">
                    {"Ubiquity"}</h1>
                <p class="text-2xl md:text-2xl lg:text-2xl xl:text-4xl 2xl:text-4xl 3xl:text-4xl font-mono">{"A markdown
                    editor."}
                </p>
            </div>
        </div>


        //     <div class="alert alert-info flex flex-col items-start">
        //         <span class="font-bold text-xl">{"Save Error"}</span>
        //         <span>{"There was no save path selected."}</span>
        //         <div tabindex="0" class="collapse collapse-plus">
        //             <div class="collapse-title text-xl font-medium h-8">
        //                 {"View full error"}
        //             </div>
        //             <div class="collapse-content">
        //                 <p>{"eeeeeeeeeeeeeeeeeeeeeeeeeeeee"}</p>
        //             </div>
        //         </div>
        //     </div>
        // </div>
    }
}
