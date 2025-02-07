use super::*;

fn not_from_hop1zuo1_candidates(game_state: &PureGameState) -> Vec<PureMove> {
    not_from_hop1zuo1_candidates_(
        &Config {
            allow_kut2tam2: false,
        },
        game_state,
    )
}

fn not_from_hop1zuo1_candidates_with_kut2tam2(game_state: &PureGameState) -> Vec<PureMove> {
    not_from_hop1zuo1_candidates_(
        &Config {
            allow_kut2tam2: true,
        },
        game_state,
    )
}

fn run_test<T, F, F2>(
    fun: F,
    sample: &PureGameState,
    serializer: F2,
    tested_against: &[&'static str],
) where
    F: FnOnce(&PureGameState) -> Vec<T>,
    F2: Fn(T) -> String,
{
    use std::collections::HashSet;
    let set1 = fun(sample)
        .into_iter()
        .map(serializer)
        .collect::<HashSet<String>>();
    let set2 = tested_against
        .iter()
        .map(|s| (*s).to_string())
        .collect::<HashSet<String>>();

    assert_eq!(set1, set2);
}

mod test_not_from_hop1zuo1_candidates_with_kut2tam2;

mod not_from_hop1zuo1_candidates;

mod test_cases;

mod empty_squares {
    use cetkaik_core::relative;
    #[test]
    fn test_initial_board_sample() {
        super::run_test(
            crate::empty_squares,
            &crate::tests::test_cases::INITIAL_BOARD_SAMPLE,
            relative::serialize_coord,
            &[
                "[1,2]", "[1,4]", "[1,6]", "[3,0]", "[3,1]", "[3,2]", "[3,3]", "[3,4]", "[3,5]",
                "[3,6]", "[3,7]", "[3,8]", "[4,0]", "[4,1]", "[4,2]", "[4,3]", "[4,5]", "[4,6]",
                "[4,7]", "[4,8]", "[5,0]", "[5,1]", "[5,2]", "[5,3]", "[5,4]", "[5,5]", "[5,6]",
                "[5,7]", "[5,8]", "[7,2]", "[7,4]", "[7,6]",
            ],
        );
    }
}

mod get_opponent_pieces_rotated {
    use cetkaik_core::relative;

    #[allow(clippy::needless_pass_by_value)]
    fn serialize_rotated(r: crate::Rotated) -> String {
        format!(
            "{} {}",
            relative::serialize_coord(r.rotated_coord),
            relative::serialize_piece(r.rotated_piece.into())
        )
    }
    #[test]
    fn test_initial_board_sample() {
        super::run_test(
            crate::get_opponent_pieces_and_tam_rotated,
            &crate::tests::test_cases::INITIAL_BOARD_SAMPLE,
            serialize_rotated,
            &[
                "[8,8] 黒筆↑",
                "[8,7] 黒馬↑",
                "[8,6] 黒車↑",
                "[8,5] 黒将↑",
                "[8,4] 赤王↑",
                "[8,3] 赤将↑",
                "[8,2] 赤車↑",
                "[8,1] 赤馬↑",
                "[8,0] 赤筆↑",
                "[7,8] 赤巫↑",
                "[7,7] 赤弓↑",
                "[7,5] 赤虎↑",
                "[7,3] 黒虎↑",
                "[7,1] 黒弓↑",
                "[7,0] 黒巫↑",
                "[6,8] 黒兵↑",
                "[6,7] 赤兵↑",
                "[6,6] 黒兵↑",
                "[6,5] 赤兵↑",
                "[6,4] 赤船↑",
                "[6,3] 赤兵↑",
                "[6,2] 黒兵↑",
                "[6,1] 赤兵↑",
                "[6,0] 黒兵↑",
                "[4,4] 皇",
            ],
        );
    }
}

mod from_hop1zuo1_candidates;
