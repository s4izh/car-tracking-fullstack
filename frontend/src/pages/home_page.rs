use crate::components::header::Header;
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
      <>
      <Header />
      <section class="bg-ct-blue-600 min-h-screen pt-20">
      <div class="max-w-6xl mx-auto bg-ct-dark-100 rounded-md h-[30rem] flex justify-center items-center">
        <div >
          <p class="text-5xl font-semibold">{"Welcome To Car Tracking"}</p>   
                <div class="mt-5 flex justify-center items-center">
                <a href="http://nattech.fib.upc.edu:40402/" target="_blank" style="color: blue;" >{"View Your Certified KM"}</a> 
                </div>
            </div>
          </div>
            

        </section>
      </>
    }
}
