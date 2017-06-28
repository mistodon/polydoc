extern crate polydoc;
extern crate polydoc_core;
extern crate polydoc_js;


#[test]
fn test_fake_js_api()
{
    use polydoc_core::Doc;

    let api_source = include_str!("apis/js_api.js");

    let docs = polydoc::parse_from_source(api_source, &polydoc_js::extract_declarations);

    let expected_docs = vec![
        Doc::Function
        {
            name: "report_version".to_owned(),
            description: "Reports API version".to_owned()
        },
        Doc::Function
        {
            name: "report_commit_hash".to_owned(),
            description: "Reports commit hash\nNot valid in debug builds".to_owned()
        },
        Doc::Function
        {
            name: "report_release_date".to_owned(),
            description: "Reports release date\nNot valid in debug builds".to_owned()
        }
    ];

    assert_eq!(docs, expected_docs);
}
