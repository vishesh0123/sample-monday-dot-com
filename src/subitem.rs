use web_sys::HtmlInputElement;
use web_sys::KeyboardEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SubitemProps {
    pub name: String,
}

#[function_component(Subitem)]
pub fn subitem(props: &SubitemProps) -> Html {
    let date = use_state(|| "2024-08-11".to_string());
    let area = use_state(|| "Area 2".to_string());
    let people = use_state(|| "Person 1".to_string());
    let notes = use_state(|| "Sample Subitem Note".to_string());
    let files = use_state(|| "0".to_string());
    let saved = use_state(|| false);

    let on_input_change = |setter: UseStateHandle<String>| {
        let saved = saved.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    setter.set(input.value());
                    saved.set(true); // Automatically save on Enter key press
                }
            }
        })
    };

    html! {
        <li class="ml-4 p-2 border rounded-lg shadow-sm my-1">
            <h4 class="text-md font-semibold">{ &props.name }</h4>
            <div class="grid grid-cols-5 gap-4 mt-2">
                <input class="border rounded p-1" type="text" value={(*date).clone()} onkeydown={on_input_change(date.clone())} />
                <input class="border rounded p-1" type="text" value={(*area).clone()} onkeydown={on_input_change(area.clone())} />
                <input class="border rounded p-1" type="text" value={(*people).clone()} onkeydown={on_input_change(people.clone())} />
                <input class="border rounded p-1" type="text" value={(*notes).clone()} onkeydown={on_input_change(notes.clone())} />
                <input class="border rounded p-1" type="text" value={(*files).clone()} onkeydown={on_input_change(files.clone())} />
            </div>
            <span class="ml-4">{ if *saved { "Saved!" } else { "" } }</span>
        </li>
    }
}
