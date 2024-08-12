use web_sys::HtmlInputElement;
use web_sys::KeyboardEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SubitemProps {
    pub name: String,
}

#[function_component(Subitem)]
pub fn subitem(props: &SubitemProps) -> Html {
    let subitem_name = use_state(|| props.name.clone());
    let date = use_state(|| "2024-08-11".to_string());
    let area = use_state(|| "Area 2".to_string());
    let people = use_state(|| "Person 1".to_string());
    let notes = use_state(|| "Sample Subitem Note".to_string());
    let files = use_state(|| "0".to_string());
    let budget = use_state(|| "$0".to_string());

    let on_input_change = |setter: UseStateHandle<String>| {
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    setter.set(input.value());
                }
            }
        })
    };

    html! {
           <>
           <input
        class="text-sm border-none focus:ring-0 focus:border-blue-300 border-l-8 border-current-800 rounded-lg"
        type="text"
        value={(*subitem_name).clone()}
        onkeydown={on_input_change(subitem_name.clone())}
    />
    <input
        class="text-sm border-none focus:ring-0 focus:border-blue-300 border-l-8 border-current rounded-lg"
        type="text"
        value={(*date).clone()}
        onkeydown={on_input_change(date.clone())}
    />
    <input
        class="text-sm border-none focus:ring-0 focus:border-blue-300 border-l-8 border-current rounded-lg"
        type="text"
        value={(*area).clone()}
        onkeydown={on_input_change(area.clone())}
    />
    <input
        class="text-sm border-none focus:ring-0 focus:border-blue-300 border-l-8 border-current rounded-lg"
        type="text"
        value={(*people).clone()}
        onkeydown={on_input_change(people.clone())}
    />
    <input
        class="text-sm border-none focus:ring-0 focus:border-blue-300 border-l-8 border-current rounded-lg"
        type="text"
        value={(*notes).clone()}
        onkeydown={on_input_change(notes.clone())}
    />
    <input
        class="text-sm border-none focus:ring-0 focus:border-blue-300 border-l-8 border-current rounded-lg"
        type="text"
        value={(*files).clone()}
        onkeydown={on_input_change(files.clone())}
    />
    <input
        class="text-sm border-none focus:ring-0 focus:border-blue-300 border-l-8 border-current rounded-lg"
        type="text"
        value={(*budget).clone()}
        onkeydown={on_input_change(budget.clone())}
    />

            </>
        }
}
