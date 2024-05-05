CREATE TABLE notes (
    id uuid not null primary key,
    title varchar not null,
    content varchar,
    time_added varchar not null,
    details json
);

CREATE TABLE todos (
    id uuid not null primary key,
    title varchar not null,
    time_added varchar not null,
    details json
)
