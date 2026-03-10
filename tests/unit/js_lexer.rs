use ai_browser::engine::js::lexer::{lex, TokenKind};

#[test]
fn unit_lexer_should_tokenize_keywords() {
    let tokens = lex("for (let i=0; i<1; i=i+1) { break; }").unwrap();
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::For)));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Break)));
}
