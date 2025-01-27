create table if not exists totalamount (
    id serial primary key,
    category text not null,
    total_amount text not null,
    spent_amount text not null,
    remaining_amount text not null,
    label text not null,
    statuss text not null
);

create table if not exists budget (
    id serial primary key,
    category text not null,
    amount text not null
);

create table if not exists alert (
    id serial primary key,
    category text not null,
    frequency text not null,
    method text not null,
    dayz text not null,
    hourz text not null,
    minutez text not null,
    secondz text not null,
    weekdays text not null
);

create table if not exists spend (
    id serial primary key,
    category text not null,
    amount text not null
)