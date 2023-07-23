create table admins (
    id serial primary key,
    username text not null unique,
    password_hash text not null,
    session_version int not null default 0,
    superuser bool not null
);

create table conferences (
    id serial primary key,

    title text not null default 'unamed',
    info text not null default 'no description',
    password text,

    start_ts timestamptz not null default now(),
    end_ts timestamptz not null default now(),

    top_left_lat double precision not null default 28.36075358346873,
    top_left_lon double precision not null default -81.5956429181568,
    width_in_tiles smallint not null default 10,
    height_in_tiles smallint not null default 10,

    admins int[] not null default '{}'
);

create table announcements (
    id serial primary key,
    conference_id int not null references conferences(id),
    posted_by int not null references admins(id),

    title text not null,
    content text not null
);

create table categories (
    id serial primary key,
    conference_id int not null references conferences(id),

    title text not null,
    info text,

    notification_preset smallint default 3
);

create table locations (
    id serial primary key,
    conference_id int not null references conferences(id),

    lat double precision not null,
    lon double precision not null,

    title text not null,
    info text
);

create table events (
    id serial primary key,
    conference_id int not null references conferences(id),
    location_id int not null references locations(id),

    title text not null,
    info text,
    categories int[] not null
);
