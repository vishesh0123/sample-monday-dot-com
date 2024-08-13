use yew::prelude::*;
mod group;
mod subitem;
use group::Group;
use rand::Rng;
use web_sys::console;

#[function_component(App)]
fn app() -> Html {
    let groups = use_state(|| Vec::<(String, String)>::new());

    let on_add_group = {
        let groups = groups.clone();
        Callback::from(move |_| {
            let mut new_groups = (*groups).clone();
            let colors = vec!["red", "blue", "green", "purple", "orange"];
            let mut rng = rand::thread_rng();
            let color = colors[rng.gen_range(0..colors.len())].to_string();
            new_groups.push((format!("This is a Sample Group {}", new_groups.len()), color));
            groups.set(new_groups);
        })
    };

    let on_update_group_name = {
        let groups = groups.clone();
        Callback::from(move |(index, new_name): (usize, String)| {
            let mut new_groups = (*groups).clone();
            if let Some(group) = new_groups.get_mut(index) {
                group.0 = new_name;
            }
            groups.set(new_groups);
        })
    };

    html! {
        <div class="p-4">
            <h1 class="text-2xl font-bold">{"Sample Monday.com Functionality"}</h1>
            <button onclick={on_add_group} class="mt-4 p-2 bg-blue-500 text-white rounded">{"Add Group"}</button>
            <div class="mt-4 pb-4">
                { for groups.iter().enumerate().map(|(index, (name, color))| html! {
                    <Group
                        name={name.clone()}
                        color={color.clone()}
                        on_update_name={on_update_group_name.clone()}
                        index={index}
                    />
                }) }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
