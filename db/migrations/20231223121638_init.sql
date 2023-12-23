-- migrate:up
create extension pg_stat_statements;
create function update_updated_at()
returns trigger as $$
begin
   NEW.updated_at = date_trunc('second', current_timestamp); 
   return NEW;
end;
$$ language 'plpgsql';

create table "user" (
    "id" uuid not null default gen_random_uuid(),
    "created_at" timestamptz not null default date_trunc('second', current_timestamp),
    "updated_at" timestamptz not null default date_trunc('second', current_timestamp),

    "email" text not null,
    "password" text not null,
    
    primary key ("id"),
    unique ("email")
);
create trigger "update_updated_at_on_user" before update on "user" for each row execute procedure update_updated_at();

-- migrate:down
drop trigger "update_updated_at_on_user" on "user" restrict;
drop table "user";

drop function update_updated_at();
drop extension pg_stat_statements;

