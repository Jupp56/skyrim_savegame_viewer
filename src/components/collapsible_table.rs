use yew::{html, Html};

use super::form_id_resolver::FormIdResolver;

pub enum RenderFunctionType<'a, T> {
    SingleArgument(fn(&T) -> Html),
    WithFormIds(
        fn(&T, &Vec<u32>, &FormIdResolver) -> Html,
        &'a Vec<u32>,
        &'a FormIdResolver,
    ),
}

pub fn render_collapsible_table<T>(
    header: String,
    unique_name: String,
    table_headers: Vec<String>,
    items: &Vec<T>,
    render_item_function: RenderFunctionType<T>,
) -> Html {
    let target = format!("#{}", unique_name);

    html! {
        <>
        <div class="row">
            <div class="col">
                <h3>
                { header }
                </h3>
            </div>
            <div class="col text-right">
                <button class="btn btn-primary" type="button" data-toggle="collapse" data-target={target} aria-expanded="false" aria-controls={unique_name.clone()}>{ "Expand" }</button>
            </div>
        </div>
        <div id={unique_name} class="collapse">
        { render_table(table_headers, items, render_item_function) }
        </div>
        </>
    }
}

pub fn render_table<T>(
    table_headers: Vec<String>,
    items: &Vec<T>,
    render_item_function: RenderFunctionType<T>,
) -> Html {
    html!(
        <table class="table table-striped">
            <thead>
                <tr>
                { table_headers.iter().map(render_th).collect::<Html>() }
                </tr>
            </thead>
            <tbody>
                { render_table_items(items, render_item_function) }
            </tbody>
        </table>
    )
}

fn render_table_items<T>(items: &Vec<T>, arguments: RenderFunctionType<T>) -> Html {
    match arguments {
        RenderFunctionType::SingleArgument(render_function) => {
            items.iter().map(render_function).collect::<Html>()
        }
        RenderFunctionType::WithFormIds(render_function, form_id_array, form_id_map) => items
            .iter()
            .map(|x| render_function(x, form_id_array, form_id_map))
            .collect::<Html>(),
    }
}

fn render_th(th: &String) -> Html {
    html!(
        <th>
        { th }
        </th>
    )
}
