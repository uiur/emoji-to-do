use std::{collections::HashMap, env};

pub mod team;
pub mod reaction;

pub struct TeamConfig {
    pub team_id: String,
    pub repo: String,
    pub reaction_patterns: Vec<ReactionPattern>,
}

#[derive(Clone, Debug)]
pub struct ReactionPattern {
    pub name: String,
    pub repo: String,
    pub assignees: Vec<String>,
}

pub struct TeamConfigMap {
    data: HashMap<String, TeamConfig>,
}

impl TeamConfigMap {
    pub fn new() -> TeamConfigMap {
        let mut config_map = HashMap::new();
        config_map.insert(
            "T1NRWJ5QT".to_string(),
            TeamConfig {
                team_id: "T1NRWJ5QT".to_string(),
                repo: "uiur/private-sandbox".to_string(),
                reaction_patterns: vec![ReactionPattern {
                    name: String::from("memo"),
                    repo: "uiur/private-sandbox".to_string(),
                    assignees: vec![],
                }],
            },
        );

        TeamConfigMap { data: config_map }
    }

    pub fn find(&self, team_id: &str, channel: &str, reaction: &str) -> Option<ReactionPattern> {
        let team_config = self.data.get(team_id);
        match team_config {
            Some(c) => c
                .reaction_patterns
                .iter()
                .find(|reaction_pattern| reaction == reaction_pattern.name)
                .and_then(|p| Some(p.clone())),
            None => {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::TeamConfigMap;

    #[test]
    fn it_returns_some_if_found() {
        let team_config_map = TeamConfigMap::new();
        let result = team_config_map.find("T1NRWJ5QT", "", "memo");
        assert_matches!(result, Some(_));
    }

    #[test]
    fn it_returns_none_if_not_found() {
        let team_config_map = TeamConfigMap::new();
        let result = team_config_map.find("foo", "bar", "baz");
        assert_matches!(result, None);
    }
}
