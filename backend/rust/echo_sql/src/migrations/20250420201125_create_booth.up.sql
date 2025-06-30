CREATE TYPE payment_type AS ENUM ('CASH', 'BTC', 'XRP', 'XMR');
CREATE TYPE delivery_type AS ENUM ('SELF', 'PICKUP', 'WTL');
CREATE TYPE shipping_type AS ENUM ('FEDEX', 'UPS', 'USPS');

CREATE TABLE booths (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID REFERENCES accounts(id),
		name varchar(255) UNIQUE NOT NULL,
		location varchar(255) UNIQUE NOT NULL,
    payment_types payment_type[],
    delivery_types delivery_type[],
    shipping_types shipping_type[],
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE booth_self_delivery_fee (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    booth_id UUID REFERENCES booths(id) UNIQUE,
    min_self_delivery_fee DOUBLE PRECISION,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE booth_shipping_fee (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    booth_id UUID REFERENCES booths(id) UNIQUE,
    min_shipping_fee DOUBLE PRECISION,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);


CREATE TABLE booth_to_address (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    booth_id UUID REFERENCES booths(id) UNIQUE,
    address_id UUID REFERENCES address(id) UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);


CREATE TABLE booth_to_crypto_info (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    booth_id UUID REFERENCES booths(id) UNIQUE,
    crypto_info_id UUID REFERENCES crypto_info(id) UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

