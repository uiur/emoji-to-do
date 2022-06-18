pragma foreign_keys = ON;
create table if not exists reactions (
  id integer primary key not null,
  name text not null,
  team_id integer not null,
  created_at text not null default (datetime('now', 'utc')),
  foreign key(team_id) references teams(id) on delete cascade
);
create unique index index_team_id_and_name_on_reactions on reactions(team_id, name);

create table if not exists reaction_assignees (
  id integer primary key not null,
  reaction_id integer not null,
  name text not null,
  created_at text not null default (datetime('now', 'utc')),
  foreign key (reaction_id) references reactions(id) on delete cascade
);

create unique index index_reaction_id_and_name_on_reaction_assignees on reaction_assignees(reaction_id, name);

