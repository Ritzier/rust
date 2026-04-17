use super::*;

#[rstest]
// Config
#[case(Some(FileEvent::Modify), None, Some(Event::ConfigModify))]
#[case(Some(FileEvent::Remove), None, Some(Event::ConfigRemove))]
#[case(Some(FileEvent::Create), None, Some(Event::ConfigCreate))]
// File
#[case(None, Some(FileEvent::Modify), Some(Event::FileModify))]
#[case(None, Some(FileEvent::Remove), Some(Event::FileRemove))]
#[case(None, Some(FileEvent::Create), Some(Event::FileCreate))]
// `ConfigFileModify`
#[case(
    Some(FileEvent::Modify),
    Some(FileEvent::Modify),
    Some(Event::ConfigFileModify)
)]
#[case(
    Some(FileEvent::Modify),
    Some(FileEvent::Create),
    Some(Event::ConfigFileModify)
)]
#[case(
    Some(FileEvent::Modify),
    Some(FileEvent::Remove),
    Some(Event::ConfigFileModify)
)]
// ConfigRemove
#[case(
    Some(FileEvent::Remove),
    Some(FileEvent::Modify),
    Some(Event::ConfigRemove)
)]
#[case(
    Some(FileEvent::Remove),
    Some(FileEvent::Remove),
    Some(Event::ConfigRemove)
)]
#[case(
    Some(FileEvent::Remove),
    Some(FileEvent::Create),
    Some(Event::ConfigRemove)
)]
#[tokio::test]
async fn merge_events_test(
    #[case] config: Option<FileEvent>,
    #[case] file: Option<FileEvent>,
    #[case] expected: Option<Event>,
) {
    let mut seen = HashMap::new();

    if let Some(c) = config {
        seen.insert(FileType::Config, c);
    }

    if let Some(f) = file {
        seen.insert(FileType::File, f);
    }

    let result = Watcher::merge_events(seen);

    assert_eq!(result, expected);
}
