create table if not exists users (
  id integer primary key not null,
  slack_team_id text not null,
  slack_user_id text not null,
  slack_token text not null,
  created_at text not null default (datetime('now', 'utc'))
);

create unique index if not exists index_users_on_slack_user_id on users(slack_user_id);
