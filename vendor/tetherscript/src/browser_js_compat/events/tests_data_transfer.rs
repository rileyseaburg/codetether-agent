use super::*;

#[test]
fn data_transfer_constructor_defaults_and_string_data() {
    let result = eval_with_dom(
        "",
        "let dt=DataTransfer();let initial=typeof DataTransfer+':' +\
         typeof window.DataTransfer+':'+dt.dropEffect+':'+dt.effectAllowed+':' +\
         dt.types.length+':'+dt.files.length+':'+dt.items.length;\
         let r=dt.setData('text/plain','one');dt.setData('text/html','<b>');\
         dt.setData('text/plain',42);\
         let mid=dt.getData('text/plain')+':'+dt.getData('missing')+':' +\
         dt.types.join('|')+':'+(r===undefined);\
         dt.clearData('text/html');\
         let one=dt.types.join('|')+':'+dt.getData('text/html');dt.clearData();\
         initial+';'+mid+';'+one+';'+dt.types.length+':' +\
         dt.getData('text/plain');",
    )
    .unwrap();
    assert_eq!(
        result.value.display(),
        "function:function:none:all:0:0:0;42::text/plain|text/html:true;text/plain:;0:"
    );
}

#[test]
fn data_transfer_items_and_drag_event_init_are_deterministic() {
    let result = eval_with_dom(
        "",
        "let dt=DataTransfer();let a=dt.items.add('payload','text/plain');\
         dt.items.add({name:'upload'});\
         let before=dt.items.length+':'+dt.items[0].type+':'+a.kind+':' +\
         dt.items[1].kind;dt.items.remove(0);\
         let after=dt.items.length+':'+dt.items[0].kind;dt.items.clear();\
         let e=DragEvent('drop',{dataTransfer:dt});\
         before+';'+after+';'+dt.items.length+':'+(e.dataTransfer===dt);",
    )
    .unwrap();
    assert_eq!(
        result.value.display(),
        "2:text/plain:string:file;1:file;0:true"
    );
}
