use super::super::*;

#[test]
fn blob_file_text_array_buffer_and_slice_work() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let b=Blob(['hi ',Uint8Array([65,66])],{type:'Text/Plain'}); \
         let text=''; let bytes=''; let sliced=''; \
         b.text().then(function(v){ text=v; }); \
         b.arrayBuffer().then(function(v){ bytes=v.length+':'+v[3]; }); \
         b.slice(3,5,'x/y').text().then(function(v){ sliced=v; }); \
         let f=File(['ok'],'a.txt',{type:'text/custom',lastModified:42}); \
         setTimeout(function(){document.getElementById('out').textContent=\
         text+'|'+bytes+'|'+sliced+'|'+b.size+'|'+b.type+'|'+f.name+'|'+f.lastModified+'|'+f.type;},0);\
         b.size+'|'+b.type+'|'+f.name+'|'+f.lastModified+'|'+f.type;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("5|text/plain|a.txt|42|text/custom".into())
    );
    assert!(result
        .html
        .contains("hi AB|5:65|AB|5|text/plain|a.txt|42|text/custom"));
}
