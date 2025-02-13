#[test]
fn test_tam_corner_sample() {
    super::run_test(
        super::not_from_hop1zuo1_candidates_with_kut2tam2,
        &crate::tests::test_cases::tam_corner_sample(),
        crate::pure_move::PureMove::serialize,
        &[
            "CAI片XAI",
            "CAI片CY",
            "MAI片MAUMAI",
            "PAI片PAUPAI",
            "MAU片MIAMAU",
            "MAU片PAUMAU",
            "MAU片MAIMAU",
            "PAU片MAUPAU",
            "PAU片PAIPAU",
            "MIA片MAUMIA",
            /* 撃皇 */
            "PAU片PIAPAU",
            "MIA片PIAMIA",
        ],
    );
}

#[test]
fn test_tam_itself_is_not_tam_hue_sample() {
    super::run_test(
        super::not_from_hop1zuo1_candidates_with_kut2tam2,
        &crate::tests::test_cases::TAM_ITSELF_IS_NOT_TAM_HUE_SAMPLE,
        crate::pure_move::PureMove::serialize,
        &[
            &crate::tests::test_cases::INITIAL_MOVES_NO_KUT_TAM[..],
            &vec![
                "ZI片ZOXO",
                "ZI片ZOTO",
                "ZI片ZOCO",
                "ZI片ZONO",
                "ZI片ZO心ZY",
                "ZI片ZO心ZAI",
                "ZI片ZO心ZU",
                "ZI片ZO心ZI",
                "ZI片ZO心ZE",
            ][..],
        ]
        .concat(),
    );
}

#[test]
fn test_initial_board_sample() {
    super::run_test(
        super::not_from_hop1zuo1_candidates_with_kut2tam2,
        &crate::tests::test_cases::INITIAL_BOARD_SAMPLE,
        crate::pure_move::PureMove::serialize,
        &[
            &crate::tests::test_cases::INITIAL_MOVES_NO_KUT_TAM[..],
            &[
                "ZI片ZOXO",
                "ZI片ZOTO",
                "ZI片ZOCO",
                "ZI片ZONO",
                "ZI片ZO心ZY",
                "ZI片ZO心ZAI",
                "ZI片ZO心ZU",
                "ZI片ZO心ZI",
                "ZI片ZO心ZE",
            ],
        ]
        .concat(),
    );
}
