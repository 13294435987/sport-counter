create table users (
    id integer primary key not null,
    openid text not null,
    session_key text not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create unique index users_openid_index on users(openid);