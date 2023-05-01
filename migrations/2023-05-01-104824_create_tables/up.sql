-- Your SQL goes here
CREATE TABLE users (
    id              INT             NOT NULL AUTO_INCREMENT,
    matricula       VARCHAR(255)    NOT NULL UNIQUE,
    hash            VARCHAR(255)    NOT NULL,
    total_km        INT             NOT NULL DEFAULT 0,
    trip	          INT		          NOT NULL DEFAULT 0,
    date_created    TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);

CREATE TABLE trips (
    id              INT             NOT NULL,
    trip            INT             NOT NULL,
    km    	        INT             NOT NULL,
    max_speed       INT             NOT NULL,
    avg_speed       DOUBLE           NOT NULL,
    fuel_percentage DOUBLE           NOT NULL,
    duration        INT             NOT NULL,
    trouble_codes   VARCHAR(255)    NOT NULL,
    date_created           TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id, trip)
);

CREATE TABLE car_data (
  id                  INT             NOT NULL,
  trip		            INT		          NOT NULL,
  timestamp           VARCHAR(255)    NOT NULL,
  speed               INT             NOT NULL,
  rpm                 INT             NOT NULL,
  throttle            DOUBLE           NOT NULL,
  engine_load         DOUBLE           NOT NULL,
  engine_coolant_temp DOUBLE           NOT NULL,
  oil_temp            DOUBLE           NOT NULL,
  fuel_level          DOUBLE           NOT NULL,
  fuel_consumption    DOUBLE           NOT NULL,
  PRIMARY KEY (id, timestamp)
);

