create table if not exists teams (
  id integer primary key not null,
  name text not null,
  slack_team_id text not null,
  created_at text not null default (datetime('now', 'utc'))
);
