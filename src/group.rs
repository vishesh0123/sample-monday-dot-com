use crate::subitem::Subitem;
use web_sys::HtmlInputElement;
use web_sys::KeyboardEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GroupProps {
    pub name: String,
    pub color: String, // Pass the color from the parent component
}

#[function_component(Group)]
pub fn group(props: &GroupProps) -> Html {
    let tasks = use_state(|| Vec::new());
    let group_name = use_state(|| props.name.clone());
    let is_expanded = use_state(|| false);
    let is_editing = use_state(|| false);

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
                    <>
                    <div class="grid grid-cols-7 gap-4 mt-2 ml-8 text-left font-semibold text-sm text-gray-800">
                        <span>{"Task"}</span>
                        <span>{"Date"}</span>
                        <span>{"Area"}</span>
                        <span>{"Project Owner"}</span>
                        <span>{"Notes"}</span>
                        <span>{"Files / Image Capture"}</span>
                        <span>{"Progress"}</span>
                    </div>
                    <ul class="mt-4">
                        { for tasks.iter().map(|task| html! { <Task name={task.clone()} color={props.color.clone()} /> }) }

                        <AddTaskRow on_add={on_add_task.clone()} />
                    </ul>
                    </>
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
    pub color: String, // Add color prop
}

#[function_component(Task)]
pub fn task(props: &TaskProps) -> Html {
    let subitems = use_state(|| Vec::new());
    let is_expanded = use_state(|| false);
    let task_name = use_state(|| props.name.clone());
    let date = use_state(|| "2024-08-11".to_string());
    let area = use_state(|| "Area 1".to_string());
    let owner = use_state(|| "Owner 1".to_string());
    let notes = use_state(|| "Sample Note".to_string());
    let files = use_state(|| "0".to_string());
    let budget = use_state(|| "$0".to_string());

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
        <li class={format!("grid grid-cols-7 gap-0 items-center bg-white border border-{}-500 rounded-lg p-3 mb-2 shadow-sm border-l-8",props.color.clone())}>
            <div class="flex items-center space-x-2">
                <button onclick={toggle_expand} class="focus:outline-none">
                    { if *is_expanded { "v" } else { ">" } }
                </button>
                <input
                    class="text-base font-medium border-none focus:ring-0 focus:border-blue-300 rounded-lg"
                    type="text"
                    value={(*task_name).clone()}
                    onkeydown={on_input_change(task_name.clone())}
                />
            </div>
            <input class="text-sm border-none focus:ring-0 focus:border-blue-300" type="text" value={(*date).clone()} onkeydown={on_input_change(date.clone())} />
            <input class="text-sm border-none focus:ring-0 focus:border-blue-300" type="text" value={(*area).clone()} onkeydown={on_input_change(area.clone())} />
            <input class="text-sm border-none focus:ring-0 focus:border-blue-300" type="text" value={(*owner).clone()} onkeydown={on_input_change(owner.clone())} />
            <input class="text-sm border-none focus:ring-0 focus:border-blue-300" type="text" value={(*notes).clone()} onkeydown={on_input_change(notes.clone())} />
            <input class="text-sm border-none focus:ring-0 focus:border-blue-300" type="text" value={(*files).clone()} onkeydown={on_input_change(files.clone())} />
            <input class="text-sm border-none focus:ring-0 focus:border-blue-300" type="text" value={(*budget).clone()} onkeydown={on_input_change(budget.clone())} />
        </li>
        { if *is_expanded {
            html! {
                <>
                <div class="grid grid-cols-7 gap-4 mt-2 ml-8 text-left font-semibold text-sm text-gray-600">
                    <span>{"Subitem"}</span>
                    <span>{"Date"}</span>
                    <span>{"Area"}</span>
                    <span>{"People - Sent/Responded"}</span>
                    <span>{"Notes"}</span>
                    <span>{"Files / Image Capture"}</span>
                    <span>{"Budget/Price"}</span>
                </div>
                <ul class="grid grid-cols-7 gap-0 mt-2 ml-8">
                    { for subitems.iter().map(|subitem| html! { <Subitem name={subitem.clone()} /> }) }
                    <li class="col-span-7">
                        <AddSubitemRow on_add={on_add_subitem.clone()} />
                    </li>
                </ul>
                </>
            }
        } else {
            html! { }
        }}
        </>
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
