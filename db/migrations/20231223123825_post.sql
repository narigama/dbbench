-- migrate:up
create table "post" (
    "id" uuid not null default gen_random_uuid(),
    "created_at" timestamptz not null default date_trunc('second', current_timestamp),
    "updated_at" timestamptz not null default date_trunc('second', current_timestamp),

    "user_id" uuid not null references "user" on delete cascade,
    "title" text not null,
    "body" text not null,
    
    primary key ("id")
);
create trigger "update_updated_at_on_post" before update on "post" for each row execute procedure update_updated_at();

-- migrate:down
drop trigger "update_updated_at_on_post" on "post" restrict;
drop table "post";

