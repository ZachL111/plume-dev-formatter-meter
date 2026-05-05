use plume_dev_formatter_meter::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 81, capacity: 106, latency: 9, risk: 16, weight: 4 };
    assert_eq!(score(signal), 178);
    assert_eq!(classify(signal), "accept");
    let signal = Signal { demand: 87, capacity: 89, latency: 12, risk: 13, weight: 8 };
    assert_eq!(score(signal), 209);
    assert_eq!(classify(signal), "accept");
    let signal = Signal { demand: 91, capacity: 84, latency: 13, risk: 20, weight: 13 };
    assert_eq!(score(signal), 198);
    assert_eq!(classify(signal), "accept");
}
