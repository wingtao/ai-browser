use ai_browser::engine::{dom::parser::parse_html, net::url::parse_url};

#[test]
fn navigation_pipeline_without_network() {
    let url = parse_url("https://example.com/docs").unwrap();
    assert_eq!(url.host, "example.com");

    let html = r#"
      <html>
        <head><title>Doc</title></head>
        <body><h1>Parser</h1><p>from scratch</p></body>
      </html>
    "#;
    let doc = parse_html(html).unwrap();
    assert_eq!(doc.find_title().as_deref(), Some("Doc"));
    assert!(doc.visible_text(100).contains("Parser"));
}
