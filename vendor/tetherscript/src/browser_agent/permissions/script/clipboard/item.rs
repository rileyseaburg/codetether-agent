//! ClipboardItem constructor and item helpers.

pub(super) const SOURCE: &str = r#"
function __clipBlob(t,v){
if(v&&typeof v.text=='function'){return v;}
let s=''+v;
let o={type:t,size:s.length,__text:s};
o.text=function(){return __clipOk(this.__text);};
return o;
}
function __clipAddType(item,t,v){
item.__items[t]=__clipText(v);
item.types.push(t);
return item;
}
function ClipboardItem(data){
let d=data;
if(!d){d={};}
this.types=[];
this.__items={};
if(d['text/plain']!=undefined){__clipAddType(this,'text/plain',d['text/plain']);}
this.getType=function(t){
let k=''+t;
let v=this.__items[k];
if(v==undefined){return __clipErr('NotFoundError','clipboard type not found');}
return __clipOk(__clipBlob(k,v));
};
undefined;
}
window.ClipboardItem=ClipboardItem;
"#;
