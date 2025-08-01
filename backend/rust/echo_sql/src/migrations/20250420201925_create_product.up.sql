CREATE TABLE
  products (
	id UUID PRIMARY KEY, 
  booth_id UUID REFERENCES booths(id),
	name varchar(255),
  price DOUBLE PRECISION,
  delivery_types delivery_type[],
  shipping_types shipping_type[],
	wtl_delivery_types wtl_delivery_type[],
  quantity int,
	unlimited BOOLEAN DEFAULT FALSE,
	bidable BOOLEAN DEFAULT FALSE,
  visible BOOLEAN,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);


CREATE TABLE product_self_delivery_info (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID REFERENCES products(id) UNIQUE,
    self_delivery_fee DOUBLE PRECISION,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE product_shipping_fee (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID REFERENCES products(id) UNIQUE,
    shipping_fee DOUBLE PRECISION,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE product_info (
	id UUID PRIMARY KEY, 
  product_id UUID REFERENCES products(id) UNIQUE,
	weight int,
	height int,
	length int,
	width int,
	description varchar(255),
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);


CREATE TABLE bids (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID REFERENCES products(id) UNIQUE,
    account_id UUID REFERENCES accounts(id),
    booth_id UUID REFERENCES booths(id),
    price DOUBLE PRECISION,
	  accepted BOOLEAN DEFAULT FALSE,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
