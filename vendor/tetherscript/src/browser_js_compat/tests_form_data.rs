use super::super::*;

#[test]
fn form_data_collects_forms_and_mutates_entries() {
    let result = eval_with_dom(
        "<form id='f'><input name='q' value='one'><input name='skip' disabled value='x'><input type='checkbox' name='ok' checked></form>",
        "let d=new FormData(document.getElementById('f')); \
         d.append('q','two'); d.append('upload',File(['x'],'x.txt'),'sent.txt'); \
         let all=d.getAll('q'); let rows=d.entries(); let file=d.get('upload'); \
         let before=d.has('skip'); d.delete('ok'); \
         d.get('q')+'|'+all.length+'|'+all[1]+'|'+before+'|'+d.has('ok')+'|'+rows.length+'|'+file.name+'|'+file.size;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("one|2|two|false|false|4|sent.txt|1".into())
    );
}

#[test]
fn form_data_set_iterators_and_for_each_are_deterministic() {
    let result = eval_with_dom(
        "<main></main>",
        "let d=new FormData(); \
         d.append('a','1'); d.append('a','2'); d.append('b','3'); \
         d.set('a','9'); d.set('c','4'); \
         let rows=d.entries(); let seen=''; \
         d.forEach(function(value,name,self){ seen=seen+name+'='+value+':' + (self===d) + ';'; }); \
         d.keys().join(',')+'|'+d.values().join(',')+'|'+rows.length+'|'+rows[0][0]+'='+rows[0][1]+'|'+d.getAll('a').length+'|'+seen;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("a,b,c|9,3,4|3|a=9|1|a=9:true;b=3:true;c=4:true;".into())
    );
}

#[test]
fn form_data_collects_select_controls_like_browsers() {
    let html = "<form id='f'><select name='single'><option>A</option><option value='b' selected>B</option></select><select name='first'><option>First Text</option><option value='z'>Z</option></select><select name='multi' multiple><option value='x' selected>X</option><option selected>Y Text</option><option value='d' disabled selected>D</option></select><select name='off' disabled><option selected value='bad'>Bad</option></select><input name='q' value='one'><input type='checkbox' name='ok' checked><textarea name='body'>Hi</textarea></form>";
    let script = "let d=new FormData(document.getElementById('f')); d.get('single')+'|'+d.get('first')+'|'+d.getAll('multi').join(',')+'|'+d.has('off')+'|'+d.get('q')+'|'+d.get('ok')+'|'+d.get('body')+'|'+d.entries().length;";
    let result = eval_with_dom(html, script).unwrap();
    assert_eq!(
        result.value,
        JsValue::String("b|First Text|x,Y Text|false|one|on|Hi|7".into())
    );
}
