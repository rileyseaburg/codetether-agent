//! Page-local clipboard item state helpers.

pub(super) const SOURCE: &str = r#"
function __clipText(v){
if(v==undefined){return '';}
if(v&&v.__blobBytes){
if(typeof TextDecoder=='function'){return TextDecoder().decode(v.__blobBytes);}
return '';
}
if(v&&typeof v.text=='function'){
let out='';
v.text().then(function(t){out=''+t;});
return out;
}
return ''+v;
}
function __clipItem(v){
if(v&&v.getType){return v;}
return new ClipboardItem(v);
}
function __clipSeedText(){
let t=window.__agentClipboardText;
if(t==undefined){t='';}
window.__agentClipboardItems=[new ClipboardItem({'text/plain':''+t})];
window.__agentClipboardItemText=''+t;
}
function __clipEnsureItems(){
if(!window.__agentClipboardItems){__clipSeedText();}
let t=window.__agentClipboardText;
if(t==undefined){t='';}
if(window.__agentClipboardItemText!=''+t){__clipSeedText();}
return window.__agentClipboardItems;
}
function __clipItemsText(items){
let i=0;
while(i<items.length){
let item=items[i];
if(item&&item.__items&&item.__items['text/plain']!=undefined){
return __clipText(item.__items['text/plain']);
}
i=i+1;
}
return undefined;
}
function __clipPlainText(){
let text=__clipItemsText(__clipEnsureItems());
if(text!=undefined){return text;}
let t=window.__agentClipboardText;
if(t==undefined){return '';}
return ''+t;
}
"#;
