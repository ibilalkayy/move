create table if not exists budget (
    id serial primary key,
    category text not null,
    amount BIGINT not null
);