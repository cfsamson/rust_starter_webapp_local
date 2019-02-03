CREATE TABLE IF NOT EXISTS registrations (
    id integer primary key asc,
    date text not null,
    dateint integer not null,
    item_id integer not null,
    quantity numeric not null 
);

CREATE TABLE IF NOT EXISTS items (
    id integer primary key asc,
    name text not null
);

