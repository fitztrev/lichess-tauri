-- Your SQL goes here

create table settings (
  key text primary key not null,
  value text not null
);

create table engines (
  lichess_id text primary key not null,
  binary_location text not null
);
