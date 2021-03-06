import ProblemModel from "../../../interfaces/ProblemModel";
import Submission from "../../../interfaces/Submission";

export interface Trophy {
  title: string;
  reason: string;
  achieved: boolean;
  sortId: string;
  group: TrophyGroup;
}

export const TrophyGroups = [
  "AC Count",
  "Contests",
  "Problems",
  "Streak",
] as const;
export type TrophyGroup = typeof TrophyGroups[number];

export interface TrophySubmission {
  submission: Submission;
  problemModel: ProblemModel | undefined;
}
