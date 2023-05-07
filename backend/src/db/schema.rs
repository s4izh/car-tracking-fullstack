// @generated automatically by Diesel CLI.

diesel::table! {
    car_data (id, timestamp) {
        id -> Integer,
        trip -> Integer,
        timestamp -> Varchar,
        speed -> Integer,
        rpm -> Integer,
        throttle -> Integer,
        engine_load -> Integer,
        engine_coolant_temp -> Integer,
        oil_temp -> Integer,
        fuel_level -> Integer,
        fuel_consumption -> Integer,
    }
}

diesel::table! {
    trips (id, trip) {
        id -> Integer,
        trip -> Integer,
        km -> Integer,
        max_speed -> Integer,
        avg_speed -> Integer,
        fuel_percentage -> Integer,
        duration -> Integer,
        trouble_codes -> Varchar,
        date_created -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        matricula -> Varchar,
        hash -> Varchar,
        total_km -> Integer,
        trip -> Integer,
        date_created -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    car_data,
    trips,
    users,
);
