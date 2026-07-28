use super::super::*;

#[test]
fn blob_file_bytes_returns_byte_array_promise() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let out=''; Blob(['A',Uint8Array([66])]).bytes().then(function(v){\
         out=v.length+':'+v[0]+':'+v[1];document.getElementById('out').textContent=out; }); out;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("".into()));
    assert!(result.html.contains("2:65:66"));
}

#[test]
fn blob_file_sliced_blob_bytes_use_sliced_content() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let out=''; Blob(['abcd']).slice(1,3).bytes().then(function(v){\
         out=v.length+':'+v[0]+':'+v[1];document.getElementById('out').textContent=out; }); out;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("".into()));
    assert!(result.html.contains("2:98:99"));
}

#[test]
fn blob_file_webkit_relative_path_defaults_empty() {
    let result = eval_with_dom(
        "<main></main>",
        "let f=File(['ok'],'a.txt',{type:'text/custom',lastModified:42});\
         f.name+'|'+f.lastModified+'|'+f.type+'|'+f.size+'|'+f.webkitRelativePath;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("a.txt|42|text/custom|2|".into())
    );
}
