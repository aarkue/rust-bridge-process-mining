use chrono::DateTime;

use crate::event_log::{import_xes::import_xes_slice, AttributeValue};

#[test]
fn test_xes_gz_import() {
    let x = include_bytes!("test_data/Sepsis Cases - Event Log.xes.gz");
    let log = import_xes_slice(x, true, None);

    // Log has 1050 cases total
    assert_eq!(log.traces.len(), 1050);

    // Case with concept:name "A" has 22 events
    let case_a = log.traces.iter().find(|t| {
        t.attributes
            .get("concept:name")
            .is_some_and(|c| c.value == AttributeValue::String("A".to_string()))
    });
    assert!(case_a.is_some());
    assert_eq!(case_a.unwrap().events.len(), 22);

    // Test if activity trace of case "A" is correct
    let case_a_act_trace: Vec<AttributeValue> = case_a
        .unwrap()
        .events
        .iter()
        .map(|ev| {
            ev.attributes
                .get(&"concept:name".to_string())
                .unwrap()
                .value
                .clone()
        })
        .collect();
    let case_a_correct_act_trace: Vec<AttributeValue> = vec![
        "ER Registration",
        "Leucocytes",
        "CRP",
        "LacticAcid",
        "ER Triage",
        "ER Sepsis Triage",
        "IV Liquid",
        "IV Antibiotics",
        "Admission NC",
        "CRP",
        "Leucocytes",
        "Leucocytes",
        "CRP",
        "Leucocytes",
        "CRP",
        "CRP",
        "Leucocytes",
        "Leucocytes",
        "CRP",
        "CRP",
        "Leucocytes",
        "Release A",
    ]
    .into_iter()
    .map(|act| AttributeValue::String(act.into()))
    .collect();
    assert_eq!(case_a_act_trace, case_a_correct_act_trace);

    // Test if first event of case "A" has correct timestamp
    assert_eq!(
        case_a.unwrap().events[0]
            .attributes
            .get("time:timestamp")
            .unwrap()
            .value,
        AttributeValue::Date(
            DateTime::parse_from_rfc3339("2014-10-22 09:15:41+00:00")
                .unwrap()
                .into()
        )
    );

    // Test if log name is correct
    let log_name = match &log.attributes.get("concept:name").unwrap().value {
        AttributeValue::String(s) => Some(s.as_str()),
        _ => None,
    };
    assert_eq!(log_name, Some("Sepsis Cases - Event Log"));
}