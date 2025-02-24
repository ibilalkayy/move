create table if not exists blockchain (
    id serial primary key,
    private_key text not null,
    alchemy_url text not null
);

create table if not exists gmail (
    id serial primary key,
    username text not null,
    gmail_address text not null,
    app_password text not null
);

create table if not exists totalamount (
    id serial primary key,
    total_amount int not null,
    spent_amount int not null,
    remaining_amount int not null
);

create table if not exists statuss (
    id serial primary key,
    statuss text not null
);

create table if not exists totalcategories (
    id serial primary key,
    category text not null,
    label text not null
);

create table if not exists budget (
    id serial primary key,
    category text not null,
    amount int not null,
    spent_amount int not null,
    remaining_amount int not null
);

create table if not exists spend (
    id serial primary key,
    category text not null,
    amount int not null
)