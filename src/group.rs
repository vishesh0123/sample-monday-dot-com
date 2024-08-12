use crate::subitem::Subitem;
use web_sys::HtmlInputElement;
use web_sys::KeyboardEvent;
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct GroupProps {
    pub name: String,
    pub color: String,  // Pass the color from the parent component
}

#[function_component(Group)]
pub fn group(props: &GroupProps) -> Html {
    let tasks = use_state(|| Vec::new()); 
    let group_name = use_state(|| props.name.clone());
    let is_expanded = use_state(|| false);
    let is_editing = use_state(|| false);

    // The random color is now passed as a prop and won't change on re-renders
    let random_color = &props.color;

    let on_add_task = {
        let tasks = tasks.clone();
        Callback::from(move |new_task: String| {
            let mut new_tasks = (*tasks).clone();
            new_tasks.push(new_task);
            tasks.set(new_tasks);
        })
    };

    let toggle_expand = {
        let is_expanded = is_expanded.clone();
        Callback::from(move |_| {
            is_expanded.set(!*is_expanded);
        })
    };

    let on_group_name_change = {
        let group_name = group_name.clone();
        let is_editing = is_editing.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    group_name.set(input.value());
                    is_editing.set(false);
                }
            }
        })
    };

    let on_edit_start = {
        let is_editing = is_editing.clone();
        Callback::from(move |_| {
            is_editing.set(true);
        })
    };

    let on_edit_end = {
        let is_editing = is_editing.clone();
        Callback::from(move |_| {
            is_editing.set(false);
        })
    };

    html! {
        <div class={format!("p-4 min-h-24 border rounded-lg my-4 border-l-8 border-{}-500", random_color)}>
            <div class="flex items-center">
                <button onclick={toggle_expand} class="mr-2">
                    { if *is_expanded { "v" } else { ">" } }
                </button>
                { if *is_editing {
                    html! {
                        <input
                            class={format!("text-xl font-semibold border focus:outline-none font-poppins text-{}-500", random_color)}
                            type="text"
                            value={(*group_name).clone()}
                            onkeydown={on_group_name_change}
                            onblur={on_edit_end.clone()}
                            autofocus=true
                            style="font-family: 'Poppins', 'Roboto', 'Noto Sans Hebrew', 'Noto Kufi Arabic', sans-serif;"
                        />
                    }
                } else {
                    html! {
                        <span
                            class={format!("text-xl font-semibold font-poppins text-{}-500 border border-transparent hover:border-{}-500", random_color, random_color)}
                            onclick={on_edit_start}
                            style="font-family: 'Poppins', 'Roboto', 'Noto Sans Hebrew', 'Noto Kufi Arabic', sans-serif; cursor: pointer; padding: 4px;"
                        >
                            { &*group_name }
                        </span>
                    }
                }}
            </div>
            { if *is_expanded {
                html! {
                    <ul class="mt-4">
                        { for tasks.iter().map(|task| html! { <Task name={task.clone()} /> }) }
                        <AddTaskRow on_add={on_add_task.clone()} />
                    </ul>
                }
            } else {
                html! { }
            }}
        </div>
    }
}


#[derive(Properties, PartialEq)]
pub struct TaskProps {
    pub name: String,
}

#[function_component(Task)]
pub fn task(props: &TaskProps) -> Html {
    let subitems = use_state(Vec::new);
    let task_name = use_state(|| props.name.clone());
    let is_expanded = use_state(|| false);
    let date = use_state(|| "2024-08-11".to_string());
    let area = use_state(|| "Area 1".to_string());
    let owner = use_state(|| "Owner 1".to_string());
    let notes = use_state(|| "Sample Note".to_string());
    let files = use_state(|| "0".to_string());
    let budget = use_state(|| "$0".to_string());
    let progress = use_state(|| "0%".to_string());

    let on_add_subitem = {
        let subitems = subitems.clone();
        Callback::from(move |new_subitem: String| {
            let mut new_subitems = (*subitems).clone();
            new_subitems.push(new_subitem);
            subitems.set(new_subitems);
        })
    };

    let toggle_expand = {
        let is_expanded = is_expanded.clone();
        Callback::from(move |_| {
            is_expanded.set(!*is_expanded);
        })
    };

    let on_task_name_change = {
        let task_name = task_name.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    task_name.set(input.value());
                }
            }
        })
    };

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
        <li class="ml-4 p-4 border rounded-lg shadow-md my-2">
            <div class="flex items-center">
                <button onclick={toggle_expand} class="mr-2">
                    { if *is_expanded { "v" } else { ">" } }
                </button>
                <input
                    class="text-lg font-semibold border-b-2 focus:outline-none"
                    type="text"
                    value={(*task_name).clone()}
                    onkeydown={on_task_name_change}
                />
            </div>
            { if *is_expanded {
                html! {
                    <div>
                        <div class="grid grid-cols-7 gap-4 mt-4">
                            <input class="border rounded p-1" type="text" value={(*date).clone()} onkeydown={on_input_change(date.clone())} />
                            <input class="border rounded p-1" type="text" value={(*area).clone()} onkeydown={on_input_change(area.clone())} />
                            <input class="border rounded p-1" type="text" value={(*owner).clone()} onkeydown={on_input_change(owner.clone())} />
                            <input class="border rounded p-1" type="text" value={(*notes).clone()} onkeydown={on_input_change(notes.clone())} />
                            <input class="border rounded p-1" type="text" value={(*files).clone()} onkeydown={on_input_change(files.clone())} />
                            <input class="border rounded p-1" type="text" value={(*budget).clone()} onkeydown={on_input_change(budget.clone())} />
                            <input class="border rounded p-1" type="text" value={(*progress).clone()} onkeydown={on_input_change(progress.clone())} />
                        </div>
                        <ul class="mt-4">
                            { for subitems.iter().map(|subitem| html! { <Subitem name={subitem.clone()} /> }) }
                            <AddSubitemRow on_add={on_add_subitem.clone()} />
                        </ul>
                    </div>
                }
            } else {
                html! { }
            }}
        </li>
    }
}

#[derive(Properties, PartialEq)]
pub struct AddTaskRowProps {
    pub on_add: Callback<String>,
}

#[function_component(AddTaskRow)]
pub fn add_task_row(props: &AddTaskRowProps) -> Html {
    let task_name = use_state(|| "".to_string());

    let on_keydown = {
        let task_name = task_name.clone();
        let on_add = props.on_add.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    let value = input.value();
                    if !value.is_empty() {
                        on_add.emit(value.clone());
                        task_name.set("".to_string()); // Reset the input field
                    }
                }
            }
        })
    };

    html! {
        <li class="ml-4 p-2 border rounded-lg shadow-sm my-1">
            <input
                class="border rounded p-1 w-full"
                type="text"
                value={(*task_name).clone()}
                placeholder="Add task"
                onkeydown={on_keydown}
            />
        </li>
    }
}

#[derive(Properties, PartialEq)]
pub struct AddSubitemRowProps {
    pub on_add: Callback<String>,
}

#[function_component(AddSubitemRow)]
pub fn add_subitem_row(props: &AddSubitemRowProps) -> Html {
    let subitem_name = use_state(|| "".to_string());

    let on_keydown = {
        let subitem_name = subitem_name.clone();
        let on_add = props.on_add.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    let value = input.value();
                    if !value.is_empty() {
                        on_add.emit(value.clone());
                        subitem_name.set("".to_string()); // Reset the input field
                    }
                }
            }
        })
    };

    html! {
        <li class="ml-4 p-2 border rounded-lg shadow-sm my-1">
            <input
                class="border rounded p-1 w-full"
                type="text"
                value={(*subitem_name).clone()}
                placeholder="Add subitem"
                onkeydown={on_keydown}
            />
        </li>
    }
}
