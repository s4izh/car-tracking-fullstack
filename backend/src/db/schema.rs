// @generated automatically by Diesel CLI.

diesel::table! {
    car_data (id, timestamp) {
        id -> Integer,
        trip -> Integer,
        timestamp -> Varchar,
        speed -> Integer,
        rpm -> Integer,
        throttle -> Float,
        engine_load -> Float,
        engine_coolant_temp -> Float,
        oil_temp -> Float,
        fuel_level -> Float,
        fuel_consumption -> Float,
    }
}

diesel::table! {
    trips (id, trip) {
        id -> Integer,
        trip -> Integer,
        km -> Integer,
        avg_speed -> Float,
        fuel_percentage -> Float,
        duration -> Integer,
        trouble_codes -> Varchar,
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
