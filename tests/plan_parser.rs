use runa::plan::parser::parse_plan;

#[test]
fn extracts_code_blocks_as_steps() {
    let md = r#"
# Title

Some text

```bash
echo hello
```

More text

```python
print(1)
```
"#;
    let steps = parse_plan(md).expect("parse");
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0].code.trim(), "echo hello");
    assert_eq!(steps[1].code.trim(), "print(1)");
}
