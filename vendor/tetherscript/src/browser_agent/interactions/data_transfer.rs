//! DataTransfer-like object script assembly.

pub(crate) fn create(seed: &str) -> String {
    format!(
        "let __store={{}};let __types=[];function __add(t){{let f=false;\
         for(let i=0;i<__types.length;i=i+1){{if(__types[i]==t){{f=true;}}}}\
         if(!f){{__types.push(t);}}}}let dt={{types:__types,dropEffect:'none',\
         effectAllowed:'all',setData:function(t,v){{__store[t]=''+v;__add(t);}},\
         getData:function(t){{let v=__store[t];if(typeof v=='undefined'){{return '';}}return v;}},\
         clearData:function(t){{if(typeof t=='undefined'){{__store={{}};\
         __types=[];this.types=__types;}}else{{__store[t]='';}}}}}};\
         dt.setData('text/plain',{});",
        seed
    )
}
