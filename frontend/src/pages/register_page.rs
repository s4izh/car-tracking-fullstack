use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::api::user_api::api_register_user;
use crate::components::{form_input::FormInput, loading_button::LoadingButton};
use crate::router::{self, Route};
use crate::store::{set_page_loading, set_show_alert, Store};

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]

struct RegisterUserSchema {
    #[validate(length(min = 1, message = "License plate is required"))]
    matricula: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 5, message = "Password must be at least 5 characters")
    )]
    hash: String,
    #[validate(
        length(min = 1, message = "Please confirm your password"),
        must_match(other = "hash", message = "Passwords do not match")
    )]
    hash_confirm: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<RegisterUserSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "name" => data.matricula = value,
            "password" => data.hash = value,
            "password_confirm" => data.hash_confirm= value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let form = use_state(|| RegisterUserSchema::default());
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let navigator = use_navigator().unwrap();

    let name_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();
    let password_confirm_input_ref = NodeRef::default();

    let validate_input_on_blur = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut data = cloned_form.deref().clone();
            match name.as_str() {
                "password" => data.hash = value,
                _ => (),
            }
            cloned_form.set(data);

            match cloned_form.validate() {
                Ok(_) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .remove(name.as_str());
                }
                Err(errors) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .retain(|key, _| key != &name);
                    for (field_name, error) in errors.errors() {
                        if field_name == &name {
                            cloned_validation_errors
                                .borrow_mut()
                                .errors_mut()
                                .insert(field_name.clone(), error.clone());
                        }
                    }
                }
            }
        })
    };

    let handle_name_input = get_input_callback("name", form.clone());
    let handle_password_input = get_input_callback("password", form.clone());
    let handle_password_confirm_input = get_input_callback("password_confirm", form.clone());

    let on_submit = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();
        let cloned_navigator = navigator.clone();
        let cloned_dispatch = dispatch.clone();

        let cloned_name_input_ref = name_input_ref.clone();
        let cloned_password_input_ref = password_input_ref.clone();
        let cloned_password_confirm_input_ref = password_confirm_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            let form = cloned_form.clone();
            let validation_errors = cloned_validation_errors.clone();
            let navigator = cloned_navigator.clone();
            let dispatch = cloned_dispatch.clone();

            let name_input_ref = cloned_name_input_ref.clone();
            let password_input_ref = cloned_password_input_ref.clone();
            let password_confirm_input_ref = cloned_password_confirm_input_ref.clone();

            event.prevent_default();
            spawn_local(async move {
                match form.validate() {
                    Ok(_) => {
                        let form_data = form.deref().clone();
                        let form_json = serde_json::to_string(&form_data).unwrap();
                        set_page_loading(true, dispatch.clone());

                        let name_input = name_input_ref.cast::<HtmlInputElement>().unwrap();
                        let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();
                        let password_confirm_input = password_confirm_input_ref
                            .cast::<HtmlInputElement>()
                            .unwrap();

                        name_input.set_value("");
                        password_input.set_value("");
                        password_confirm_input.set_value("");

                        let res = api_register_user(&form_json).await;
                        match res {
                            Ok(_) => {
                                set_page_loading(false, dispatch.clone());
                                set_show_alert(
                                    "Account registered successfully".to_string(),
                                    dispatch,
                                );
                                navigator.push(&router::Route::LoginPage);
                            }
                            Err(e) => {
                                if e.contains("400") {
                                    set_page_loading(false, dispatch.clone());
                                    set_show_alert("Account Already Exists".to_string(), dispatch);
                                } else {
                                    set_page_loading(false, dispatch.clone());
                                    set_show_alert(e.to_string(), dispatch);
                                }
                            }
                        };
                    }
                    Err(e) => {
                        validation_errors.set(Rc::new(RefCell::new(e)));
                    }
                }
            });
        })
    };

    html! {
    <section class="py-8 bg-ct-blue-600 min-h-screen grid place-items-center">
      <div class="w-full">
        <h1 class="text-4xl xl:text-6xl text-center font-[600] text-ct-yellow-600 mb-4">
         {" Welcome to Car Tracking!"}
        </h1>
        <h2 class="text-lg text-center mb-4 text-ct-dark-200">
          {"Sign Up To Get Started!"}
        </h2>
          <form
            onsubmit={on_submit}
            class="max-w-md w-full mx-auto overflow-hidden shadow-lg bg-ct-dark-200 rounded-2xl p-8 space-y-5"
          >
            <FormInput label="License plate" name="matricula" input_ref={name_input_ref} handle_onchange={handle_name_input}  errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
            <FormInput label="Password" name="hash" input_type="password" input_ref={password_input_ref} handle_onchange={handle_password_input}  errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
            <FormInput
              label="Confirm Password"
              name="hash_confirm"
              input_type="password"
              input_ref={password_confirm_input_ref}
              handle_onchange={handle_password_confirm_input}
              errors={&*validation_errors}
              handle_on_input_blur={validate_input_on_blur.clone()}
            />
            <span class="block">
              {"Already have an account?"} {" "}
            <Link<Route> to={Route::LoginPage} classes="text-ct-blue-600">{"Login Here"}</Link<Route>>
            </span>
            <LoadingButton
              loading={store.page_loading}
              text_color={Some("text-ct-blue-600".to_string())}
            >
             {" Sign Up"}
            </LoadingButton>
          </form>
      </div>
    </section>
    }
}
