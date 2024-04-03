create table notes (
    id uuid not null primary key,
    title varchar not null,
    content varchar,
    time_added varchar not null,
    details json
)

create table todos (
    id uuid not null primary key,
    title varchar not null,
    time_added varchar not null,
    details json
)