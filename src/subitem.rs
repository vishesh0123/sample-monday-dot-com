use web_sys::HtmlInputElement;
use web_sys::KeyboardEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SubitemProps {
    pub name: String,
    pub color: String,
    pub on_update_name: Callback<String>,
    pub on_update_date: Callback<String>,
    pub on_update_area: Callback<String>,
    pub on_update_people: Callback<String>,
    pub on_update_notes: Callback<String>,
    pub on_update_files: Callback<String>,
    pub on_update_budget: Callback<String>,
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

    {
        let subitem_name = subitem_name.clone();
        use_effect_with(props.name.clone(), move |name| {
            subitem_name.set(name.clone());
            || ()
        });
    }

    let on_input_change = |setter: UseStateHandle<String>, on_update: Callback<String>| {
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    let value = input.value();
                    setter.set(value.clone());
                    on_update.emit(value);
                }
            }
        })
    };

    html! {
        <li class={format!("ml-3 p-2 border rounded-lg shadow-sm my-1 border-l-8 border-{}-500", props.color)}>
            <div class="grid grid-cols-7 gap-4">
                <input
                    class="text-sm border-none focus:ring-0 focus:border-blue-300 rounded-lg w-full"
                    type="text"
                    value={(*subitem_name).clone()}
                    onkeydown={on_input_change(subitem_name.clone(), props.on_update_name.clone())}
                />
                <input
                    class="text-sm border-none focus:ring-0 focus:border-blue-300 rounded-lg w-full"
                    type="text"
                    value={(*date).clone()}
                    onkeydown={on_input_change(date.clone(), props.on_update_date.clone())}
                />
                <input
                    class="text-sm border-none focus:ring-0 focus:border-blue-300 rounded-lg w-full"
                    type="text"
                    value={(*area).clone()}
                    onkeydown={on_input_change(area.clone(), props.on_update_area.clone())}
                />
                <input
                    class="text-sm border-none focus:ring-0 focus:border-blue-300 rounded-lg w-full"
                    type="text"
                    value={(*people).clone()}
                    onkeydown={on_input_change(people.clone(), props.on_update_people.clone())}
                />
                <input
                    class="text-sm border-none focus:ring-0 focus:border-blue-300 rounded-lg w-full"
                    type="text"
                    value={(*notes).clone()}
                    onkeydown={on_input_change(notes.clone(), props.on_update_notes.clone())}
                />
                <input
                    class="text-sm border-none focus:ring-0 focus:border-blue-300 rounded-lg w-full"
                    type="text"
                    value={(*files).clone()}
                    onkeydown={on_input_change(files.clone(), props.on_update_files.clone())}
                />
                <input
                    class="text-sm border-none focus:ring-0 focus:border-blue-300 rounded-lg w-full"
                    type="text"
                    value={(*budget).clone()}
                    onkeydown={on_input_change(budget.clone(), props.on_update_budget.clone())}
                />
            </div>
        </li>
    }
}
