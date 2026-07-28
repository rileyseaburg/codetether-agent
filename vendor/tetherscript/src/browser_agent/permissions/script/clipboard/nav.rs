//! Navigator clipboard read/write object methods.

pub(super) const SOURCE: &str = r#"
function __clipDenied(p){return __clipErr('NotAllowedError',p+' denied');}
function __clipRead(){
if(__states['clipboard-read']!='granted'){return __clipDenied('clipboard-read');}
return __clipOk(__clipEnsureItems().slice(0));
}
function __clipReadText(){
if(__states['clipboard-read']!='granted'){return __clipDenied('clipboard-read');}
return __clipOk(__clipPlainText());
}
function __clipWrite(items){
if(__states['clipboard-write']!='granted'){return __clipDenied('clipboard-write');}
let out=[];
let source=items;
if(source&&source.length!=undefined){
let i=0;
while(i<source.length){out.push(__clipItem(source[i]));i=i+1;}
}else if(source){out.push(__clipItem(source));}
window.__agentClipboardItems=out;
window.__agentClipboardText=__clipItemsText(out);
if(window.__agentClipboardText==undefined){window.__agentClipboardText='';}
window.__agentClipboardItemText=window.__agentClipboardText;
return __clipOk(undefined);
}
function __clipWriteText(v){
if(__states['clipboard-write']!='granted'){return __clipDenied('clipboard-write');}
let t=''+v;
window.__agentClipboardText=t;
window.__agentClipboardItems=[new ClipboardItem({'text/plain':t})];
window.__agentClipboardItemText=t;
return __clipOk(undefined);
}
navigator.clipboard={
read:__clipRead,
readText:__clipReadText,
write:__clipWrite,
writeText:__clipWriteText
};
"#;
