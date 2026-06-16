create table bundle_kinds (
    kind text not null primary key
);

insert into bundle_kinds (kind) values ('static'), ('nodejs');

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";  

create table bundles (
    id uuid default uuid_generate_v4(),
    is_uploaded bool not null default(false),
    is_deployed bool not null default(false),
    owner text not null,
    kind text not null references bundle_kinds(kind),

    -- TODO: preferences for nodes and other options?

    primary key (id)
);