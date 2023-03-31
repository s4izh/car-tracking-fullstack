use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let onclick = {
        move |_| {
            web_sys::window()
                .unwrap()
                .alert_with_message("Hola")
                .unwrap();
        }
    };

    html! {
        <main>

        <div style="text-align: center">
        <h1>{ "Futuro Nombre del Proyecto de PTI" }</h1>
        <p>{ "Introduce el usuario y la contraseña." }</p>
        </div>
        <div style = "margin-top: 10px; text-align: center">
        <input type="text" placeholder="Usuario" />
        </div>
        <div style = "margin-top: 10px; text-align: center">
        <input type="password" placeholder="Contraseña" />
        </div>
        <div style = "margin-top: 10px; text-align: center">
            <button {onclick}>{ "login" }</button>
        </div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
