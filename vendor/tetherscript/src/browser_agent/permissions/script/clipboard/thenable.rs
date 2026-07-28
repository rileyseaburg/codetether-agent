//! Small synchronous thenables for clipboard APIs.

pub(super) const SOURCE: &str = r#"
function __clipOk(v){
let o={};
o.then=function(cb){if(cb){cb(v);}return o;};
o.catch=function(){return o;};
return o;
}
function __clipErr(n,m){
let o={};
let e={name:n,message:m};
o.then=function(){return o;};
o.catch=function(cb){if(cb){cb(e);}return o;};
return o;
}
"#;
