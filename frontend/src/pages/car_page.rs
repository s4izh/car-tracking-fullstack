use gloo::console;
use plotters::coord::Shift;
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
          <div class="max-w-6xl mx-auto bg-ct-dark-100 rounded-md h-[50rem] flex justify-left">
            <div>
              <p class="ml-5 mt-8 text-5xl font-semibold">{"Car Engine Info"}</p>
            /*   if let Some(info) = info {
                <div class="mt-8">
                <p class="text-3xl font-semibold">{"General Info"}</p>
                <div class="mt-4 ml-10">
                  <p class="mb-4">{format!("Timestap: {}", info.timestamp)}</p>
                  <p class="mb-4">{format!("Velocidad: {}", info.speed)}</p>
                  <details position="absolute">
                  <summary>{"Trip 1 Info"}</summary>
                  <p class="mb-4">{format!("RPM: {}", info.rpm)}</p>
                  <p class="mb-4">{format!("RPM: {}", info.rpm)}</p>
                  <p class="mb-4">{format!("RPM: {}", info.rpm)}</p>
                  <p class="mb-4">{format!("RPM: {}", info.rpm)}</p>
                  <p class="mb-4">{format!("RPM: {}", info.rpm)}</p>
                  <p class="mb-4">{format!("RPM: {}", info.rpm)}</p>
                  <p class="mb-4">{format!("RPM: {}", info.rpm)}</p>
                  <p class="mb-4">{format!("RPM: {}", info.rpm)}</p>
                  <p class="mb-4">{format!("RPM: {}", info.rpm)}</p>
                  <p class="mb-4">{format!("RPM: {}", info.rpm)}</p>
                  <p class="mb-4">{format!("RPM: {}", info.rpm)}</p>
                  </details>
                  </div>
          
                </div>
              }else {
                <div class="mt-8">
                <p class="mb-4">{"Aún no se han generado datos"}</p>
                </div>
              } */
             
              if let Some(trip_info) = trip_info {
                <div class="mt-8 ml-10">
                <p class="text-3xl font-semibold">{"General Info"}</p>
                <div class="mt-4 ml-5">
                <p class="mb-4">{format!("License Plate: {}", trip_info.matricula)}</p>
                <p class="mb-4">{format!("Total km: {}", trip_info.total_km)}</p>
                <p class="mb-4">{format!("Total trips: {}", trip_info.trip)}</p>
                <p class="mb-4">{format!("Date: {}", trip_info.date_created)}</p>
                <div style="background-color: #cccccc; width: 100%; max-width: 800px; margin: 0 auto; padding: 20px;">
                <details position="absolute">
                  <summary>{"Trip 1 Info"}</summary>
                  <div class="mt-4">
                  <p class="mb-4">{format!("Km: {}", trip_info.trip_data[0].km)}</p>
                  <p class="mb-4">{format!("Max speed: {}", trip_info.trip_data[0].max_speed)}</p>
                  <p class="mb-4">{format!("Average speed: {}", trip_info.trip_data[0].speed_average)}</p>
                  <p class="mb-4">{format!("Duration: {}", trip_info.trip_data[0].duration)}</p>
                  <p class="mb-4">{format!("Trouble Codes: {}", trip_info.trip_data[0].trouble_codes)}</p>
                  </div>
                  <details position="absolute">
                  <summary>{"Trip 1 Data traces"}</summary>
                  <div class="mt-4 ml-5">
                  <p class="mb-4">{format!("RPM: {}", trip_info.trip_data[0].data[0].rpm)}</p>
                  <p class="mb-4">{format!("Speed: {}", trip_info.trip_data[0].data[0].speed)}</p>
                  <p class="mb-4">{format!("Throttle position: {}", trip_info.trip_data[0].data[0].throttle)}</p>
                  </div>
                  </details>
                  </details>
                  </div>
                  

                </div>
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