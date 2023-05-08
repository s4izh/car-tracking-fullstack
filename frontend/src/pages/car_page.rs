use gloo::console;
use yew::prelude::*;
use plotters::prelude::*;
//use yew_router::prelude::use_navigator;
use yewdux::prelude::*;
use crate::api::user_api::api_car;
use crate::components::header::Header;
use crate::store::{set_info_car, set_login_user, set_trip_info, Store};
use crate::store::TRIP_INFO;


#[function_component(CarPage)]
pub fn car_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let info = store.info_car.clone();
    let user = store.login_user.clone();
   // let trip_info=store.trip_info.clone();
   let trip_info = unsafe { &TRIP_INFO };
    use_effect_with_deps(
        move |_| {
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = api_car().await;
                match response {
                    Ok(info) => {
                        set_info_car(Some(info), dispatch);
                        console::log!("Hola");
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
            /*   if let Some(info) = info {
                <div class="mt-8">
                  <p class="mb-4">{format!("Timestap: {}", info.timestamp)}</p>
                  <p class="mb-4">{format!("Velocidad: {}", info.speed)}</p>
                  <p class="mb-4">{format!("RPM: {}", info.rpm)}</p>
          
                </div>
              }else {
                <div class="mt-8">
                <p class="mb-4">{"Aún no se han generado datos"}</p>
                </div>
              }*/
              if let Some(user) = user {
              <div class="mt-8">
              <p class="mb-4">{format!("Matricula: {}", user.matricula)}</p>
              </div>
              }else {
                <div class="mt-8">
                <p class="mb-4">{"Aún no se han generado datos"}</p>
                </div>
              }
              if let Some(trip_info) = trip_info {
                <div class="mt-8">
                <p class="mb-4">{format!("KM: {}", trip_info.total_km)}</p>
                <p class="mb-4">{format!("trip_data: {}", trip_info.trip_data[0].max_speed)}</p>
                <p class="mb-4">{format!("fuel: {}", trip_info.trip_data[0].data[0].fuel_level)}</p>


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