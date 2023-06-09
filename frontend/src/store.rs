use serde::{Deserialize, Serialize};
use yewdux::prelude::*;
use gloo::console;

use crate::api::types::{User, CarGeneralData, UserData2, Trip, UserTrip};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct AlertInput {
    pub show_alert: bool,
    pub alert_message: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
pub struct Store {
    pub auth_user: Option<User>,
    pub page_loading: bool,
    pub alert_input: AlertInput,
    pub info_car: Option<CarGeneralData>,
    pub login_user: Option<UserData2>,
    pub trip_info: Option<Trip>,
}

pub static mut TRIP_INFO: Option<UserTrip> = None;


pub fn set_trip_info(info: Option<UserTrip>) {
    unsafe {
        TRIP_INFO = info;
    }
}

pub fn set_info_car(info: Option<CarGeneralData>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.info_car = info;
    })
}

pub fn set_page_loading(loading: bool, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.page_loading = loading;
    })
}

pub fn set_login_user(user: Option<UserData2>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.login_user = user;
    })
}

pub fn set_auth_user(user: Option<User>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.auth_user = user;
    })
}

pub fn set_show_alert(message: String, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input = AlertInput {
            alert_message: message,
            show_alert: true,
        };
    })
}

pub fn set_hide_alert(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input.show_alert = false;
    })
}
