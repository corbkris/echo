CREATE TYPE wtl_delivery_type AS ENUM ('CAR', 'TRUCK', 'TRUCK-TRAILER', 'CAR-HAULER');

CREATE TABLE
  drivers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID REFERENCES accounts(id) UNIQUE,
		name varchar(255) UNIQUE NOT NULL,
    wtl_delivery_types wtl_delivery_type[],
    start_day int NOT NULL,
		end_day int NULL,
    start_time TIMESTAMPTZ NOT NULL,
		end_time TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
  );

CREATE TABLE
  driver_vehicle (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    driver_id UUID REFERENCES drivers(id),
    make varchar(255) NOT NULL,
    wtl_delivery_types wtl_delivery_type,
		gross_capacity int NULL,
    picture varchar(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
  );

CREATE TABLE driver_to_address (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    driver_id UUID REFERENCES accounts(id) UNIQUE,
    address_id UUID REFERENCES address(id) UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE driver_to_crypto_info (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    driver_id UUID REFERENCES drivers(id) UNIQUE,
    crypto_info_id UUID REFERENCES crypto_info(id) UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
