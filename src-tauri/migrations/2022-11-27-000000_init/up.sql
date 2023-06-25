create table engines (
  engine_id text primary key not null,
  binary_location text not null,
  uci_options text not null
);

create table settings (
  key text primary key not null,
  value text not null
);

-- Add default settings
insert into settings (key, value) values ("lichess_host", "https://lichess.org");
insert into settings (key, value) values ("engine_host", "https://engine.lichess.ovh");
insert into settings (key, value) values ("provider_secret", lower(hex(randomblob(32))));
