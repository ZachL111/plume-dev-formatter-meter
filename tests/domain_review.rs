use plume_dev_formatter_meter::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 65, slack: 26, drag: 25, confidence: 68 };
    assert_eq!(review_score(case), 149);
    assert_eq!(review_lane(case), "ship");
}
