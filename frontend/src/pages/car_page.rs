use yew::prelude::*;
use yewdux::prelude::*;
use crate::components::header::Header;
use crate::store::{set_info_car, set_login_user, set_trip_info, Store};
use crate::store::TRIP_INFO;



#[function_component(CarPage)]
pub fn car_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
   // let trip_info=store.trip_info.clone();
   let trip_info = unsafe { &TRIP_INFO };

   let trip_info_html: Html = if let Some(trip_info) = trip_info {
    let mut trip_info_html = html! {};
    let mut trip_count = 1;

    for (i, item) in trip_info.trip_data.iter().enumerate() {
        let trip_html = html! {
            <div style="background-color: #cccccc; margin: 0 auto; padding: 20px; width: 230px;">
                <details>
                    <summary>{format!("Trip {}", i+1)}</summary>
                    <div>
                        <p class="mb-4 mt-4">{format!("Km: {}", item.km)}</p>
                        <p class="mb-4">{format!("Max speed: {}", item.max_speed)}</p>
                        <p class="mb-4">{format!("Average speed: {}", item.speed_average)}</p>
                        <p class="mb-4">{format!("Duration: {}", item.duration)}</p>
                        <p class="mb-4">{format!("Trouble Codes: {}", item.trouble_codes)}</p>
                    </div>
                </details>
            </div>
        };

        if trip_count % 4 == 0 {
            trip_info_html = html! {
                <>
                    {trip_info_html}
                    <div style="overflow: hidden; margin-bottom: 20px; float: left">
                        {trip_html}
                    </div>
                </>
            };
        } else {
            trip_info_html = html! {
                <>
                    {trip_info_html}
                    <div style="float: left; width: 230px; margin-right: 30px; margin-bottom: 20px;">
                        {trip_html}
                    </div>
                </>
            };
        }

        trip_count += 1;
    }

    trip_info_html
} else {
    html! {
        <div class="mt-8">
            <p class="mb-4">{"Aún no se han generado datos"}</p>
        </div>
    }
};



    

   
    html! {
      <>
        <Header />
        <section class="bg-ct-blue-600 min-h-screen pt-20">
          <div class="max-w-6xl mx-auto bg-ct-dark-100 rounded-md h-[70rem] flex justify-left">
            <div style="width: 1500px;">
              <p class="ml-5 mt-8 text-5xl font-semibold">{"Your Trips"}</p>   
              if let Some(trip_info) = trip_info {
                <div class="mt-8 ml-10">
                <p class="text-3xl font-semibold">{"General Information"}</p>
                <div class="mt-4 ml-5">
                <p class="mb-4 text-1xl font-semibold">{format!("License Plate: {}", trip_info.matricula)}</p>
                <p class="mb-4 text-1xl font-semibold">{format!("Total km: {}", trip_info.total_km)}</p>
                <p class="mb-4 text-1xl font-semibold">{format!("Total trips: {}", trip_info.trip)}</p>
                <p class="mb-4 text-1xl font-semibold">{format!("Date: {}", trip_info.date_created)}</p>
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
                
                <div class="mt-8 ml-10">
                {trip_info_html}
                </div>
            </div>
              
          </div>
          
        </section>
      </>
      }
}