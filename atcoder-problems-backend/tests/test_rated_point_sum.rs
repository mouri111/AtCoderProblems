use atcoder_problems_backend::sql::models::{Contest, ContestProblem, Submission, UserSum};
use atcoder_problems_backend::sql::schema::{contest_problem, contests, rated_point_sum};
use atcoder_problems_backend::sql::RatedPointSumClient;
use diesel::dsl::*;
use diesel::prelude::*;

mod utils;

const FIRST_AGC_EPOCH_SECOND: i64 = 1_468_670_400;
const UNRATED_STATE: &str = "-";

#[test]
fn test_update_rated_point_sum() {
    let conn = utils::initialize_and_connect_to_test_sql();

    let user_id = "user";
    let rated_contest = "rated_contest";
    let unrated_contest1 = "unrated_contest1";
    let unrated_contest2 = "unrated_contest2";

    let same_contest_unrated = "same_contest_unrated";
    let same_contest_rated = "same_contest_rated";

    insert_into(contests::table)
        .values(vec![
            Contest {
                id: rated_contest.to_string(),
                start_epoch_second: FIRST_AGC_EPOCH_SECOND,
                duration_second: 1000,
                title: "Rated Contest".to_string(),
                rate_change: "All".to_string(),
            },
            Contest {
                id: unrated_contest1.to_string(),
                start_epoch_second: 0,
                duration_second: 1000,
                title: "Unrated Old Contest".to_string(),
                rate_change: "All".to_string(),
            },
            Contest {
                id: unrated_contest2.to_string(),
                start_epoch_second: FIRST_AGC_EPOCH_SECOND,
                duration_second: 1000,
                title: "Unrated New Contest".to_string(),
                rate_change: UNRATED_STATE.to_string(),
            },
            Contest {
                id: same_contest_rated.to_string(),
                start_epoch_second: FIRST_AGC_EPOCH_SECOND,
                duration_second: 1000,
                title: "Unrated New Contest".to_string(),
                rate_change: "All".to_string(),
            },
            Contest {
                id: same_contest_unrated.to_string(),
                start_epoch_second: FIRST_AGC_EPOCH_SECOND,
                duration_second: 1000,
                title: "Unrated New Contest".to_string(),
                rate_change: UNRATED_STATE.to_string(),
            },
        ])
        .execute(&conn)
        .unwrap();

    let submissions = vec![
        Submission {
            id: 0,
            user_id: user_id.to_string(),
            point: 100.0,
            problem_id: "problem1".to_string(),
            contest_id: rated_contest.to_string(),
            ..Default::default()
        },
        Submission {
            id: 1,
            user_id: user_id.to_string(),
            point: 100.0,
            problem_id: "problem1".to_string(),
            contest_id: rated_contest.to_string(),
            ..Default::default()
        },
        Submission {
            id: 2,
            user_id: user_id.to_string(),
            point: 100.0,
            problem_id: "problem2".to_string(),
            contest_id: unrated_contest1.to_string(),
            ..Default::default()
        },
        Submission {
            id: 3,
            user_id: user_id.to_string(),
            point: 100.0,
            problem_id: "problem3".to_string(),
            contest_id: unrated_contest2.to_string(),
            ..Default::default()
        },
        Submission {
            id: 4,
            user_id: user_id.to_string(),
            point: 100.0,
            problem_id: "problem4".to_string(),
            contest_id: rated_contest.to_string(),
            ..Default::default()
        },
        Submission {
            id: 5,
            user_id: user_id.to_string(),
            point: 100.0,
            problem_id: "problem5".to_string(),
            contest_id: same_contest_unrated.to_string(),
            ..Default::default()
        },
    ];

    insert_into(contest_problem::table)
        .values(vec![
            ContestProblem {
                problem_id: "problem1".to_string(),
                contest_id: rated_contest.to_string(),
            },
            ContestProblem {
                problem_id: "problem2".to_string(),
                contest_id: unrated_contest1.to_string(),
            },
            ContestProblem {
                problem_id: "problem3".to_string(),
                contest_id: unrated_contest1.to_string(),
            },
            ContestProblem {
                problem_id: "problem4".to_string(),
                contest_id: rated_contest.to_string(),
            },
            ContestProblem {
                problem_id: "problem5".to_string(),
                contest_id: same_contest_rated.to_string(),
            },
            ContestProblem {
                problem_id: "problem5".to_string(),
                contest_id: same_contest_unrated.to_string(),
            },
        ])
        .execute(&conn)
        .unwrap();

    conn.update_rated_point_sum(&submissions).unwrap();
    let sums = rated_point_sum::table.load::<UserSum>(&conn).unwrap();
    assert_eq!(sums.len(), 1);
    assert_eq!(sums[0].user_id, user_id.to_string());
    assert_eq!(sums[0].point_sum, 300.0);
    assert_eq!(conn.get_users_rated_point_sum(user_id).unwrap(), 300.0);
    assert_eq!(conn.get_rated_point_sum_rank(300.0).unwrap(), 0);

    assert!(conn
        .get_users_rated_point_sum("non_existing_user")
        .is_none());
}
