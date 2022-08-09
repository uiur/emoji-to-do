-- Add down migration script here
alter table teams drop column github_installation_id;
