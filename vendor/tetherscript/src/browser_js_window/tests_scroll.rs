use super::super::eval_with_dom;
use crate::js::JsValue;

#[test]
fn scroll_methods_sync_aliases_and_dispatch_scroll() {
    let result = eval_with_dom(
        "<main></main>",
        "let seen='';window.addEventListener('scroll',function(e){seen=seen+e.type+':' +(e.target===window)+':' +this.scrollX+','+this.scrollY+';';});window.onscroll=function(){seen=seen+'H'+pageYOffset;};scrollTo(10,15);window.scrollBy(5,-3);window.scroll(2,4);[scrollX,scrollY,pageXOffset,pageYOffset,window.scrollX,window.pageYOffset,seen].join('|');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "2|4|2|4|2|4|scroll:true:10,15;H15scroll:true:15,12;H12scroll:true:2,4;H4".into()
        )
    );
}
