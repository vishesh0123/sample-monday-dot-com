use crate::subitem::Subitem;
use web_sys::console;
use web_sys::HtmlInputElement;
use web_sys::KeyboardEvent;
use yew::prelude::*;
use yew::use_effect_with;

#[derive(Properties, PartialEq)]
pub struct GroupProps {
    pub name: String,
    pub color: String,
    pub on_update_name: Callback<(usize, String)>,
    pub index: usize,
}

#[derive(Clone, PartialEq)]
pub struct TaskData {
    name: String,
    subitems: Vec<SubitemData>,
}

#[derive(Clone, PartialEq)]
pub struct SubitemData {
    name: String,
}

#[function_component(Group)]
pub fn group(props: &GroupProps) -> Html {
    console::log_1(&format!("Props: {:?}", props.name.clone()).into());
    let tasks = use_state(|| Vec::<TaskData>::new());
    let group_name = use_state(|| props.name.clone());
    let is_expanded = use_state(|| false);
    let is_editing = use_state(|| false);

    let random_color = &props.color;

    {
        let group_name = group_name.clone();
        use_effect_with(props.name.clone(), move |name| {
            console::log_1(&format!("Updating group_name to: {}", name).into());
            group_name.set(name.clone());
            || ()
        });
    }

    let task_count = tasks.len();
    let subitem_count: usize = tasks.iter().map(|task| task.subitems.len()).sum();

    console::log_1(&format!("Group Component Rendered: {}", *group_name).into());
    console::log_1(&format!("Random Color: {}", random_color).into());
    console::log_1(&format!("Is Expanded: {}", *is_expanded).into());
    console::log_1(&format!("Is Editing: {}", *is_editing).into());

    let on_add_task = {
        let tasks = tasks.clone();
        Callback::from(move |new_task_name: String| {
            let mut new_tasks = (*tasks).clone();
            new_tasks.push(TaskData {
                name: new_task_name,
                subitems: Vec::new(),
            });
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
        let on_update_name = props.on_update_name.clone();
        let index = props.index;
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    let new_name = input.value();
                    group_name.set(new_name.clone());
                    on_update_name.emit((index, new_name));
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
            <div class="flex items-center justify-between">
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
                <span class="text-sm text-gray-500">
                    { format!("{} Tasks / {} Subitems", task_count, subitem_count) }
                </span>
            </div>
            { if *is_expanded {
                html! {
                    <>
                    <div class="grid grid-cols-7 gap-4 mt-2 ml-8 text-left font-semibold text-sm text-gray-1000">
                        <span>{"Task"}</span>
                        <span>{"Date"}</span>
                        <span>{"Area"}</span>
                        <span>{"Project Owner"}</span>
                        <span>{"Notes"}</span>
                        <span>{"Files / Image Capture"}</span>
                        <span>{"Budget/Price"}</span>
                    </div>
                    <ul class="mt-4">
                        { for (*tasks).iter().enumerate().map(|(index, task)| {
                            let cloned_tasks = tasks.clone();
                            html! {
                                <Task
                                    name={task.name.clone()}
                                    color={props.color.clone()}
                                    subitems={task.subitems.clone()}
                                    on_add_subitem={Callback::from(move |subitem_name: String| {
                                        let mut new_tasks = (*cloned_tasks).clone();
                                        if let Some(task) = new_tasks.get_mut(index) {
                                            task.subitems.push(SubitemData { name: subitem_name });
                                        }
                                        cloned_tasks.set(new_tasks);
                                    })}
                                />
                            }
                        })}

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
    pub color: String,
    pub subitems: Vec<SubitemData>,
    pub on_add_subitem: Callback<String>,
}

#[function_component(Task)]
pub fn task(props: &TaskProps) -> Html {
    let subitems = use_state(|| props.subitems.clone());
    let task_name = use_state(|| props.name.clone());
    let is_expanded = use_state(|| false);
    let date = use_state(|| "2024-08-11".to_string());
    let area = use_state(|| "Area 1".to_string());
    let owner = use_state(|| "Owner 1".to_string());
    let notes = use_state(|| "Sample Note".to_string());
    let files = use_state(|| "0".to_string());
    let budget = use_state(|| "$0".to_string());

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

    let on_add_subitem = {
        let subitems = subitems.clone();
        let on_add_subitem = props.on_add_subitem.clone();
        Callback::from(move |subitem_name: String| {
            let mut new_subitems = (*subitems).clone();
            new_subitems.push(SubitemData {
                name: subitem_name.clone(),
            });
            subitems.set(new_subitems);
            on_add_subitem.emit(subitem_name);
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
                <div class="grid grid-cols-7 gap-4 mt-6 ml-12 text-left font-semibold text-sm text-gray-1000">
                    <span>{"Subitem"}</span>
                    <span>{"Date"}</span>
                    <span>{"Area"}</span>
                    <span>{"People - Sent/Responded"}</span>
                    <span>{"Notes"}</span>
                    <span>{"Files / Image Capture"}</span>
                    <span>{"Budget/Price"}</span>
                </div>
                <ul class="ml-8 mt-4">
                    { for subitems.iter().enumerate().map(|(index, subitem)| html! {
                        <Subitem
                            name={subitem.name.clone()}
                            color={props.color.clone()}
                        />
                    })}

                    <li class="grid grid-cols-7 gap-4 mt-2 col-span-7">
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
        <li class="ml-4 mt-4 p-2 border rounded-lg shadow-sm my-1">
            <input
                class=" rounded p-1 w-full"
                type="text"
                value={(*task_name).clone()}
                placeholder=" + Add task"
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
                class=" rounded p-1 w-full"
                type="text"
                value={(*subitem_name).clone()}
                placeholder=" + Add subitem"
                onkeydown={on_keydown}
            />
        </li>
    }
}
