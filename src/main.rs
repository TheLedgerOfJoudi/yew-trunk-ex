use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Cell {
    id: usize,
    state: u8,
    color: String
}

#[derive(Properties, PartialEq)]
struct GridProps {
    cells: Vec<Cell>,
    on_click: Callback<Cell>,
}

#[function_component(Grid)]
fn grid(GridProps { cells, on_click }: &GridProps) -> Html {
    cells
        .iter()
        .map(|cell: &Cell| {
            let on_cell_select: Callback<_> = {
                let on_click: Callback<Cell> = on_click.clone();
                let cell: Cell = cell.clone();
                Callback::from(move |_| on_click.emit(cell.clone()))
            };
            html! {
                <div class={cell.color.clone()} key={cell.id} onclick = {on_cell_select}>{cell.state}</div>
            }
        })
        .collect()
}

#[function_component]
fn App() -> Html {
    let mut cells: Vec<Cell> = vec![];
    for i in 0..100 {
        cells.push(Cell { id: i, state: 0,color:"".to_string() })
    }

    let selected_cell = use_state(|| None);

    let on_cell_select = {
        let selected_cell = selected_cell.clone();
        Callback::from(move |cell: Cell| selected_cell.set(Some(cell)))
    };

    let details = selected_cell.as_ref().map(|cell| {
        
        cells[cell.id] = Cell {
            id: cell.id,
            state: cell.state + 1,
            color: "colored-div".to_string(),
        };
        html! {
        <div></div>
        }
    });

    html! {
        <div class="grid">  <Grid cells={cells} on_click={on_cell_select.clone()}  />
        {for details}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
