use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn file_input_files_empty_without_upload() {
    let result = eval_with_dom(
        "<input id='u' type='file'>",
        "let f=document.getElementById('u').files; f.length + ':' + f.item(0) + ':' + (f[0] === undefined);",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("0:null:true".into()));
}

#[test]
fn file_input_files_expose_single_file_fields() {
    let html = "<input id='u' type='file' data-agent-files='[{\"name\":\"note.txt\",\"type\":\"text/plain\",\"size\":3}]'>";
    let result = eval_with_dom(html, "let f=document.getElementById('u').files; let a=f[0]; f.length+':'+f.item(0).name+':'+a.name+':'+a.type+':'+a.size+':'+a.lastModified").unwrap();
    assert_eq!(
        result.value,
        JsValue::String("1:note.txt:note.txt:text/plain:3:0".into())
    );
}

#[test]
fn file_input_files_expose_multiple_indices_and_item() {
    let html = "<input id='u' type='file' data-agent-files='[{\"name\":\"a.txt\",\"type\":\"text/plain\",\"size\":1},{\"name\":\"b.bin\",\"type\":\"application/octet-stream\",\"size\":2}]'>";
    let result = eval_with_dom(
        html,
        "let f=document.getElementById('u').files; f.length+':'+f[0].name+':'+f.item(1).name+':'+f.item(2)",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("2:a.txt:b.bin:null".into()));
}

#[test]
fn file_input_files_expose_rows_and_for_each() {
    let html = "<input id='u' type='file' data-agent-files='[{\"name\":\"a.txt\",\"type\":\"text/plain\",\"size\":1},{\"name\":\"b.bin\",\"type\":\"application/octet-stream\",\"size\":2}]'>";
    let script = "let f=document.getElementById('u').files;let keys=f.keys();let vals=f.values();let rows=f.entries();let ctx={tag:'T'};let seen='';keys[0]=9;f.forEach(function(file,index,self){seen=seen+this.tag+':'+index+':'+file.name+':' +(self===f)+';';},ctx);f.keys()[0]+'|'+keys.join(',')+'|'+vals.length+':'+vals[0].name+','+vals[1].name+'|'+rows.length+':'+rows[0][0]+':'+rows[0][1].name+':'+rows[1][0]+':'+rows[1][1].name+'|'+seen";
    let result = eval_with_dom(html, script).unwrap();
    assert_eq!(
        result.value,
        JsValue::String(
            "0|9,1|2:a.txt,b.bin|2:0:a.txt:1:b.bin|T:0:a.txt:true;T:1:b.bin:true;".into()
        )
    );
}
