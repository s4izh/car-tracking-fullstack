use gloo::console;
use yew::prelude::*;
//use yew_router::prelude::use_navigator;
use yewdux::prelude::*;
use crate::api::user_api::api_car;
use crate::components::header::Header;
use crate::store::{set_info_car,Store};



#[function_component(CarPage)]
pub fn car_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let info = store.info_car.clone();
    use_effect_with_deps(
        move |_| {
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = api_car().await;
                match response {
                    Ok(info) => {
                        set_info_car(Some(info), dispatch);
                    }
                    Err(e) => {
                        console::log!("Error {}", e);
                    }
                }
            });
        },
        (),
    );

    

    html! {
      <>
        <Header />
        <section class="bg-ct-blue-600 min-h-screen pt-20">
          <div class="max-w-4xl mx-auto bg-ct-dark-100 rounded-md h-[20rem] flex justify-center items-center">
            <div>
              <p class="text-5xl font-semibold">{"Car Engine Info"}</p>
              if let Some(info) = info {
                <div class="mt-8">
                  <p class="mb-4">{format!("Timestap: {}", info.timestamp)}</p>
                  <p class="mb-4">{format!("Velocidad: {}", info.speed)}</p>
                  <p class="mb-4">{format!("Codigos de error: {}", info.trouble_codes)}</p>
                  <p class="mb-4">{format!("RPM: {}", info.rpm)}</p>
          
                </div>
              }else {
                <div class="mt-8">
                <p class="mb-4">{"Aún no se han generado datos"}</p>
                </div>
              }
            </div>
          </div>
        </section>
      </>
      }
}