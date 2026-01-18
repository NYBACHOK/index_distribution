create table bundle_kinds as (
    kind text not null
);

insert into bundle_kinds (kind) values ('static', 'nodejs');

create table bundles (
    id uuid default uuid_generate_v4(),
    is_uploaded bool not null default(false),
    is_deployed bool not null default(false),
    bundle_version integer not null default(0),
    sha text null,
    owner text not null, -- TODO: better way for ownership check?
    kind text not null references bundle_kinds(kind),

    -- TODO: preferences for nodes

    primary key (id)
);