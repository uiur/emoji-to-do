import { ReactionAssignee } from "./ReactionAssignee"

export interface Reaction {
  id: number
  name: string
  repo: string
  reaction_assignees: ReactionAssignee[]
}
