use tetherscript::js;

#[test]
fn optional_chaining_short_circuits_nullish_values() {
    let value = js::eval(
        "let a=null; let b={x:{y:3}, f:function(){return 4;}};\
        (a?.x) + ':' + b?.x?.y + ':' + b.f?.();",
    )
    .unwrap();

    assert_eq!(value, js::JsValue::String("undefined:3:4".into()));
}

#[test]
fn syntax_probe_ignores_string_literals() {
    assert_eq!(
        js::eval("let value='=> class ?. ?? ... import('; value;").unwrap(),
        js::JsValue::String("=> class ?. ?? ... import(".into())
    );
}

#[test]
fn unicode_identifier_property_keys_parse() {
    let key = '\u{0192}';
    let source = format!("let o={{{key}:'x'}}; o.{key};");

    assert_eq!(js::eval(&source).unwrap(), js::JsValue::String("x".into()));
}

#[test]
fn template_literals_parse_as_strings() {
    assert_eq!(
        js::eval("let value=`hello\\nrealm`; value;").unwrap(),
        js::JsValue::String("hello\nrealm".into())
    );
}

#[test]
fn template_literals_interpolate_expressions() {
    assert_eq!(
        js::eval("let name='realm'; let n=2; `hello ${name} ${n + 1}`;").unwrap(),
        js::JsValue::String("hello realm 3".into())
    );
}

#[test]
fn template_literal_expressions_rewrite_nested_regex() {
    assert_eq!(
        js::eval("`${'a///b'.replace(/\\/+/g,'/')}`;").unwrap(),
        js::JsValue::String("a/b".into())
    );
}

#[test]
fn template_literals_keep_later_strings_aligned() {
    let source = r#"let cls=`a ${['x'].join('-')} b`;
        const xs=["-","[","]","/","{","}","(",")","*","+","?",".","\\","^","$","|"];
        let r=RegExp("["+xs.join("\\")+"]","g");
        cls + ':' + typeof r;"#;

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("a x b:object".into())
    );
}

#[test]
fn template_expression_regex_literals_do_not_close_expression_early() {
    let source = r#"let name='x"y';
        let html=`<a ${Array().map.call([{nodeName:'href',value:name}], h=>`${h.nodeName}="${h.value.replace(/"/g,'&quot;')}"`).join(' ')}>`;
        html;"#;

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("<a href=\"x&quot;y\">".into())
    );
}

#[test]
fn nullish_coalescing_uses_only_null_and_undefined() {
    assert_eq!(
        js::eval("let a=null ?? 4; let b=0 ?? 9; let c=undefined ?? 3; a + ':' + b + ':' + c;")
            .unwrap(),
        js::JsValue::String("4:0:3".into())
    );
}

#[test]
fn spread_and_rest_parameters_work() {
    let source = "function sum(a,...rest){return a+rest[0]+rest[1];}\
        let xs=[1,...[2,3]]; let obj={a:1,...{b:2}};\
        sum(...xs) + ':' + obj.b;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::String("6:2".into()));
}

#[test]
fn class_constructor_methods_and_static_methods_work() {
    let source = "class Box { constructor(v){ this.v=v; } value(){ return this.v; } static make(v){ return new Box(v); } }\
        Box.make(5).value();";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(5.0));
}

#[test]
fn class_super_constructor_and_methods_use_parent_class() {
    let source = "class Base { constructor(v){ this.base=v; } value(){ return this.base + 1; } }\
        class Child extends Base { constructor(v){ super(v); this.own=2; } value(){ return super.value() + this.own; } }\
        new Child(4).value();";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(7.0));
}

#[test]
fn class_values_accept_static_property_assignment() {
    assert_eq!(
        js::eval("class Box {} Box[Symbol('internals')]=9; Box[Symbol('internals')];").unwrap(),
        js::JsValue::Number(9.0)
    );
}

#[test]
fn class_prototype_mutations_persist_for_client_factories() {
    let source = "class Client { request(config){ return config.method + ':' + config.url; } }\
        Client.prototype.get=function(url){ return this.request({method:'get',url:url}); };\
        function bind(fn,thisArg){ return function(){ return fn.apply(thisArg,arguments); }; }\
        function create(){ let ctx=new Client(); let inst=bind(Client.prototype.request,ctx);\
        Object.getOwnPropertyNames(Client.prototype).forEach(function(k){ inst[k]=bind(Client.prototype[k],ctx); });\
        return inst; }\
        let F=create(); typeof F.get + ':' + F.get('/users/my-profile');";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("function:get:/users/my-profile".into())
    );
}

#[test]
fn static_methods_can_define_prototype_accessors() {
    let source = "class Headers { set(n,v){this[n]=v;} get(n){return this[n];}\
        static accessor(n){Object.defineProperty(this.prototype,'getContentType',\
        {value:function(){return this.get(n);}});}}\
        Headers.accessor('Content-Type'); let h=new Headers();\
        h.set('Content-Type','application/json'); typeof h.getContentType + ':' + h.getContentType();";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("function:application/json".into())
    );
}

#[test]
fn function_call_preserves_later_arguments_after_this_arg() {
    let source = "let h={getContentType:function(){return 'json'},normalize:function(){return this}};\
        function transform(data,headers){return typeof headers.getContentType + ':' + headers.getContentType();}\
        transform.call({scope:true},'body',h.normalize());";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("function:json".into())
    );
}

#[test]
fn accessor_returned_methods_still_bind_receiver_this() {
    let source = "class Headers { normalize(){return this;} }\
        let original=Headers.prototype.normalize;\
        Object.defineProperty(Headers.prototype,'normalize',{get:()=>original});\
        let h=new Headers(); h.mark='ok'; h.normalize().mark;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::String("ok".into()));
}

#[test]
fn class_instances_do_not_enumerate_internal_prototype_link() {
    let source = "class Headers { constructor(){this.accept='json';} method(){return 1;} }\
        let h=new Headers(); Object.keys(h).join(',') + ':' + h.method();";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("accept:1".into())
    );
}

#[test]
fn super_constructor_inherits_function_prototype_methods() {
    let source = "function Base(){ this.subscribe=this.subscribe.bind(this); }\
        Base.prototype.subscribe=function(){ return this.name; };\
        class Child extends Base { constructor(){ super(); this.name='ok'; } }\
        new Child().subscribe();";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::String("ok".into()));
}

#[test]
fn class_accessors_work_as_properties() {
    let source = "class Box { constructor(v){ this.v=v; } get value(){ return this.v; } }\
        let box=new Box(6); box.value;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(6.0));
}

#[test]
fn class_async_methods_parse_as_methods() {
    let source = "class Box { async fetch(v){ return v + 1; } }\
        let box=new Box(); box.fetch(2);";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(3.0));
}

#[test]
fn class_keyword_method_names_parse() {
    let source = "class Box { continue(){ return 4; } } let box=new Box(); box.continue();";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(4.0));
}

#[test]
fn computed_symbol_iterator_methods_parse() {
    let source = "class Box { [Symbol.iterator](){ return 'class'; } }\
        let obj={ [Symbol.iterator](){ return 'object'; } };\
        let box=new Box(); box[Symbol.iterator]() + ':' + obj[Symbol.iterator]();";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("class:object".into())
    );
}

#[test]
fn dynamic_import_returns_resolved_module_promise() {
    assert_eq!(
        js::eval("let seen=''; import('/chunk.js').then(function(m){ seen=typeof m; }); seen;")
            .unwrap(),
        js::JsValue::String("object".into())
    );
}

#[test]
fn module_export_lists_are_ignored_in_browser_execution() {
    let source = "let zgt=1,YRt=2; export{zgt as C,YRt as F}; zgt+YRt;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(3.0));
}

#[test]
fn regex_literals_and_callback_replace_work() {
    let source = "let m={'=':'=0',':':'=2'};\
        let escaped='a=b:c'.replace(/[=:]/g,function(s){return m[s];});\
        let squashed='a///b'.replace(/\\/+/g,'/');\
        function hasName(t){return /^is[A-Z]/.test(t);}\
        let fallback=null || /\\B|\\b/;\
        let made=RegExp('x','g');\
        escaped + ':' + squashed + ':' + hasName('isReady') + ':' + typeof fallback + ':' + typeof made;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("a=0b=2c:a/b:true:object:object".into())
    );
}

#[test]
fn regex_exec_returns_match_array_with_metadata() {
    let source = "let m=/[^.]+$/.exec('a.b'); m[0] + ':' + m.index + ':' + m.input;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("b:2:a.b".into())
    );
}

#[test]
fn regex_replacement_handles_escaped_chars_and_match_marker() {
    let source = r#"let slash='a\\b'.replace(/\\/g,'/');
        let xs=["-","[","]","/","{","}","(",")","*","+","?",".","\\","^","$","|"];
        let escaped='a.b'.replace(RegExp("["+xs.join("\\")+"]","g"),"\\$&");
        slash + ':' + escaped;"#;

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("a/b:a\\.b".into())
    );
}

#[test]
fn string_match_returns_regex_arrays_for_router_paths() {
    let source = r#"let re=new RegExp("^/login\\/*$","i");
        let m="/login".match(re);
        m[0] + ':' + m.index + ':' + m.input + ':' + (/^\/login\/*$/i.test('/LOGIN/'));"#;

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("/login:0:/login:true".into())
    );
}

#[test]
fn string_match_exposes_route_param_captures() {
    let source = r#"let re=new RegExp("^/([^\\/]+)/exports\\/*$","i");
        let m="/Acme/exports".match(re);
        m[0] + ':' + m[1] + ':' + m.index;"#;

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("/Acme/exports:Acme:0".into())
    );
}

#[test]
fn regex_callback_replace_supports_camel_case_captures() {
    let source = "' content-type'.replace(/[-_\\s]([a-z\\d])(\\w*)/g,\
        function(s,r,i){return r.toUpperCase()+i;});";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("ContentType".into())
    );
}

#[test]
fn regex_literals_after_logical_and_are_rewritten() {
    assert_eq!(
        js::eval("let s=' x'; s.length && /\\s/.test(s);").unwrap(),
        js::JsValue::Bool(true)
    );
}

#[test]
fn regex_literals_after_return_keyword_keep_a_separator() {
    assert_eq!(
        js::eval("function ok(t){return/^is[A-Z]/.test(t)} ok('isReady');").unwrap(),
        js::JsValue::Bool(true)
    );
}

#[test]
fn regex_literals_and_spread_arrays_cover_highlight_patterns() {
    let source = "let p=[1], m={A:2};\
        let o={contains:[m.A,...p,{begin:/:/,end:/[;}{}]/},{begin:/(url|data-uri)\\(/,end:/\\)/}]};\
        o.contains.length;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(4.0));
}

#[test]
fn bitwise_and_shift_operators_work() {
    assert_eq!(
        js::eval("(8>>>1) + ':' + (1<<3) + ':' + (7&3) + ':' + (4|1) + ':' + (6^3) + ':' + ~0;")
            .unwrap(),
        js::JsValue::String("4:8:3:5:5:-1".into())
    );
}

#[test]
fn object_accessors_and_methods_work() {
    let source = "let v=0; let o={set _(x){v=x}, get _(){return v}, inc(){this._=this._+1;}};\
        o._=4; o.inc(); o._;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(5.0));
}

#[test]
fn object_async_method_marker_parses() {
    let source = "let o={async pull(x){return x+1}, cancel(){return 3}};\
        o.pull(2)+':'+o.cancel();";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::String("3:3".into()));
}

#[test]
fn increment_and_decrement_update_bindings() {
    assert_eq!(
        js::eval("let i=0; let a=i++; let b=++i; let c=i--; a + ':' + b + ':' + c + ':' + i;")
            .unwrap(),
        js::JsValue::String("0:2:2:1".into())
    );
}

#[test]
fn arrow_helpers_support_minified_bundle_patterns() {
    let source = "var set=Object.defineProperty;\
        var put=(e,t,s)=>t in e?set(e,t,{value:s}):e[t]=s;\
        var pair=(e,t)=>(put(e,'x',t),e.x);\
        pair({},7);";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(7.0));
}

#[test]
fn for_in_iterates_object_keys() {
    let source = "let o={a:1,b:2}; let total=0;\
        for (const k in o) total=total+o[k]; total;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(3.0));
}

#[test]
fn for_of_iterates_arrays_and_strings() {
    let source = "let out=''; for (const v of [1,2]) out=out+v;\
        for (const ch of 'ab') out=out+ch; out;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("12ab".into())
    );
}

#[test]
fn for_of_accepts_destructuring_bindings() {
    let source = "let out=''; for (const [k,v] of [['a',1],['b',2]]) out+=k+v;\
        for (const {x} of [{x:'c'}]) out+=x; out;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("a1b2c".into())
    );
}

#[test]
fn for_await_of_parses_as_for_of() {
    assert_eq!(
        js::eval("let out=''; for await (const x of ['a','b']) out+=x; out;").unwrap(),
        js::JsValue::String("ab".into())
    );
}

#[test]
fn for_of_iterates_array_like_objects_by_index() {
    let source = "let list={0:'a',1:'b',length:2,extra:'x'}; let out='';\
        for (const value of list) out=out+value; out;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::String("ab".into()));
}

#[test]
fn object_symbol_iterator_drives_array_from() {
    let source = "let it={i:0,[Symbol.iterator](){return {next(){\
        this.i=(this.i||0)+1; return this.i<3?{value:this.i,done:false}:{done:true};}}}};\
        Array.from(it).join(',') + ':' + Array.from({a:1,b:2}).length;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("1,2:0".into())
    );
}

#[test]
fn return_level_comma_sequences_work() {
    let source = "function cfg(){let a={};\
        return a.x=1,a.y=2,a.x+a.y;} cfg();";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(3.0));
}

#[test]
fn named_function_expressions_bind_their_local_name() {
    let source = "let f=function again(n){return n?again(n-1)+1:0;}; f(3);";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(3.0));
}

#[test]
fn keyword_property_names_and_symbol_for_work() {
    assert_eq!(
        js::eval("typeof Symbol.for('react.element')+':'+Symbol.for('react.element').valueOf();")
            .unwrap(),
        js::JsValue::String("symbol:Symbol.for(react.element)".into())
    );
}

#[test]
fn symbol_constructor_accepts_empty_description() {
    assert_eq!(
        js::eval("typeof Symbol()+':'+Symbol().valueOf();").unwrap(),
        js::JsValue::String("symbol:Symbol()".into())
    );
}

#[test]
fn symbols_do_not_take_string_host_type_paths() {
    let source = "let mode=Symbol.for('react.strict_mode');let out='';\
        if(typeof mode==='string') out='host';\
        else switch(mode){case Symbol.for('react.strict_mode'): out='strict';} out;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("strict".into())
    );
}

#[test]
fn native_error_constructors_cover_browser_error_types() {
    assert_eq!(
        js::eval("RangeError('bad') + '|' + TypeError();").unwrap(),
        js::JsValue::String("RangeError: bad|TypeError".into())
    );
}

#[test]
fn native_error_constructors_return_mutable_error_objects() {
    let source = "function E(m){let value=Error.call(this,m)||this;\
        value.message=m+'!'; return value;}\
        let err=new E('bad'); err.extra=1; err + '|' + err.message + '|' + err.extra;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("Error: bad!|bad!|1".into())
    );
}

#[test]
fn functions_without_explicit_return_produce_undefined() {
    let source = "function Parent(a,b,o){this.attrName=a;this.keyName=b;\
        o.whitelist&&(this.whitelist=o.whitelist);}\
        function Child(){return Parent.apply(this,arguments)||this;}\
        let item=new Child('font','ql-font',{whitelist:['serif','monospace']});\
        item.attrName + ':' + item.keyName + ':' + item.whitelist.join(',') + ':' + Array.isArray(item);";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("font:ql-font:serif,monospace:false".into())
    );
}

#[test]
fn object_define_properties_installs_descriptor_values() {
    assert_eq!(
        js::eval("let o={}; Object.defineProperties(o,{a:{value:1}, b:{value:2}}); o.a + o.b;")
            .unwrap(),
        js::JsValue::Number(3.0)
    );
}

#[test]
fn object_get_own_property_names_returns_keys() {
    let value = js::eval("let o={a:1,b:2}; Object.getOwnPropertyNames(o).join(',');").unwrap();
    assert!(
        matches!(&value, js::JsValue::String(s) if s == "a,b" || s == "b,a"),
        "{value:?}"
    );
}

#[test]
fn object_entries_and_values_iterate_own_properties() {
    let value =
        js::eval("let o={a:1,b:2}; Object.entries(o).length + Object.values(o).length;").unwrap();

    assert_eq!(value, js::JsValue::Number(4.0));
}

#[test]
fn object_from_entries_builds_object_from_rows() {
    let source = "let o=Object.fromEntries([['a',1],['b',2]]); o.a+':'+o.b;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::String("1:2".into()));
}

#[test]
fn object_define_property_accessors_are_live_descriptors() {
    let source = "let source={value:1}; let target={};\
        Object.defineProperty(target,'x',{enumerable:true,get:function(){return source.value;}});\
        let descriptor=Object.getOwnPropertyDescriptor(target,'x');\
        source.value=4;\
        target.x + ':' + typeof descriptor.get + ':' + Object.keys(target).join(',');";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("4:function:x".into())
    );
}

#[test]
fn object_define_property_accessors_work_on_functions() {
    let source = "let source={value:3}; function target(){};\
        Object.defineProperty(target,'x',{enumerable:true,get:function(){return source.value;}});\
        source.value=8;\
        target.x + ':' + Object.prototype.hasOwnProperty.call(target,'x') + ':' + Object.keys(target).includes('x');";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("8:true:true".into())
    );
}

#[test]
fn object_state_helpers_support_freeze_walkers() {
    let source = "let o={a:1}; Object.freeze(o)===o && \
        Object.seal(o)===o && Object.preventExtensions(o)===o && \
        !Object.isFrozen(o) && !Object.isSealed(o) && Object.isExtensible(o) && \
        Object.isFrozen(1);";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Bool(true));
}

#[test]
fn set_and_map_constructor_inputs_and_for_each_work() {
    let source = "let seen=''; let s=new Set(['a','b']);\
        s.add('c').forEach(v=>seen+=v);\
        let m=new Map([['x',1]]); m.set('y',2);\
        seen + ':' + s.size + ':' + s.has('b') + ':' + m.get('x') + ':' + m.size;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("abc:3:true:1:2".into())
    );
}

#[test]
fn object_get_own_property_descriptors_returns_descriptor_map() {
    assert_eq!(
        js::eval(
            "const {getOwnPropertyDescriptors:g}=Object;\
            let d=g({a:2}); d.a.value;"
        )
        .unwrap(),
        js::JsValue::Number(2.0)
    );
}

#[test]
fn object_get_prototype_of_can_be_destructured_and_called() {
    assert_eq!(
        js::eval("const {getPrototypeOf:zye}=Object; typeof zye({});").unwrap(),
        js::JsValue::String("object".into())
    );
}

#[test]
fn object_define_property_without_value_preserves_existing_property() {
    let source = "function Child(){}\
        Child.prototype={ok:1};\
        Object.defineProperty(Child,'prototype',{writable:false});\
        Child.prototype.ok;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(1.0));
}

#[test]
fn object_set_prototype_of_updates_function_proto_for_babel_helpers() {
    let source = "function Base(){} function Child(){}\
        Object.setPrototypeOf(Child,Base);\
        let parent=Child.__proto__||Object.getPrototypeOf(Child);\
        (typeof parent)+':'+(parent===Base)+':'+(Object.getPrototypeOf(function(){})===Function.prototype);";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("function:true:true".into())
    );
}

#[test]
fn object_create_uses_prototype_chain_for_inherited_properties() {
    let source = "let parent={answer:42}; let child=Object.create(parent);\
        child.own=1; child.answer+':'+('answer' in child)+':'+child.hasOwnProperty('answer');";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("42:true:false".into())
    );
}

#[test]
fn webpack_module_exports_preserve_function_named_exports() {
    let source = "let r={}; Object.defineProperty(r,'__esModule',{value:true});\
        r.default=r.BaseTooltip=void 0;\
        function Base(){}\
        function R(P,G){P.prototype=Object.create(G&&G.prototype,{constructor:{value:P}});Object.setPrototypeOf(P,G);}\
        let K=function(P){R(G,P);function G(){} return G;}(Base);\
        let H=function(P){R(G,P);function G(){} return G;}(K);\
        r.BaseTooltip=H; r.default=K;\
        let h=r&&r.__esModule?r:{default:r}; typeof h.BaseTooltip+':'+typeof h.default;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("function:function".into())
    );
}

#[test]
fn circular_module_reads_partial_named_exports_as_undefined() {
    let source = "let s=[\
        function(s,r,i){Object.defineProperty(r,'__esModule',{value:true});\
            r.default=r.BaseTooltip=void 0; i(1); r.BaseTooltip=function H(){};},\
        function(s,r,i){let m=i(0); let h=m&&m.__esModule?m:{default:m};\
            r.type=typeof h.BaseTooltip;}];\
        let c={}; function i(a){if(c[a]) return c[a].exports;\
            let o=c[a]={exports:{}}; s[a].call(o.exports,o,o.exports,i); return o.exports;}\
        i(0); i(1).type;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("undefined".into())
    );
}

#[test]
fn string_relational_comparison_supports_typeof_guards() {
    assert_eq!(
        js::eval("typeof {exports:{}} < 'u';").unwrap(),
        js::JsValue::Bool(true)
    );
}

#[test]
fn loose_equality_matches_nullish_and_scalar_browser_coercions() {
    let source = "[
        undefined == null,
        undefined != null,
        '1' == 1,
        false == 0,
        '' == 0,
        null === undefined
    ].join(':');";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("true:false:true:true:true:false".into())
    );
}

#[test]
fn commonjs_typeof_guard_can_replace_module_exports() {
    let source = "let module={exports:{}};\
        function EventEmitter(){};\
        typeof module<'u'&&(module.exports=EventEmitter);\
        typeof module.exports;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("function".into())
    );
}

#[test]
fn nested_webpack_module_reads_named_export_into_default_object() {
    let source = "let s=[\
        function(s,r,i){Object.defineProperty(r,'__esModule',{value:true});\
            var v=i(1), _={register:v.register}; r.default=_;},\
        function(s,r,i){Object.defineProperty(r,'__esModule',{value:true});\
            function b(){} r.register=b;}];\
        let c={}; function i(a){if(c[a]) return c[a].exports;\
            let o=c[a]={exports:{}}; s[a].call(o.exports,o,o.exports,i); return o.exports;}\
        let p=i(0); let v=p&&p.__esModule?p:{default:p}; typeof v.default.register;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("function".into())
    );
}

#[test]
fn hoisted_function_declaration_keeps_pre_declaration_static_properties() {
    let source = "function decorate(F){F.register=function(){return 'ok';}}\
        let C=function(){decorate(H); function H(){} return H;}();\
        typeof C.register + ':' + C.register();";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("function:ok".into())
    );
}

#[test]
fn instanceof_walks_user_function_prototype_chains() {
    let source = "function F(){} let value=new F();\
        (value instanceof F)+':'+(F instanceof Function)+':'+({} instanceof Object);";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("true:true:true".into())
    );
}

#[test]
fn babel_constructor_guard_accepts_new_function_instances() {
    let source = "function C(){if(!(this instanceof C)) throw new TypeError('bad'); this.ok=1;}\
        let c=new C(); c.ok;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(1.0));
}

#[test]
fn transpiled_super_constructor_apply_resolves_from_function_proto() {
    let source = "function Base(){this.ok=2;}\
        function Child(){return (Child.__proto__||Object.getPrototypeOf(Child)).apply(this,arguments)||this;}\
        Object.setPrototypeOf(Child,Base);\
        Child.prototype=Object.create(Base.prototype,{constructor:{value:Child}});\
        let child=new Child(); child.ok;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(2.0));
}

#[test]
fn transpiled_subclass_constructor_keeps_instanceof_guard() {
    let source = "function Base(){this.base=1;}\
        function inherits(C,P){C.prototype=Object.create(P&&P.prototype,{constructor:{value:C}});Object.setPrototypeOf(C,P);}\
        let Child=function(P){inherits(C,P); function C(){\
            if(!(this instanceof C)) throw new TypeError('bad');\
            return (C.__proto__||Object.getPrototypeOf(C)).apply(this,arguments)||this;} return C;}(Base);\
        let child=new Child(); child.base + ':' + (child instanceof Child);";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("1:true".into())
    );
}

#[test]
fn function_prototype_is_callable_and_exposes_apply() {
    let source = "let proto=Object.getPrototypeOf(function(){});\
        typeof Function.prototype+':'+typeof proto.apply+':'+(proto===Function.prototype);";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("function:function:true".into())
    );
}

#[test]
fn function_prototype_to_string_supports_lodash_native_checks() {
    let source = "let text=Function.prototype.toString.call(Object.prototype.hasOwnProperty);\
        text.includes('hasOwnProperty')+':'+text.includes('[native code]');";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("true:true".into())
    );
}

#[test]
fn new_without_parentheses_constructs_with_no_arguments() {
    let source = "class Box { constructor(){ this.v=4; } }\
        let box=new Box; box.v;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(4.0));
}

#[test]
fn for_in_sources_accept_comma_sequences_and_void() {
    let source = "let source={a:1}; let seen=0;\
        for (k in void 0, source) seen=source[k]; seen;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(1.0));
}

#[test]
fn for_initializers_accept_multiple_var_bindings() {
    let source = "let out=0; for (var i=0,j=2; i<2; i++) out=out+j; out;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(4.0));
}

#[test]
fn for_var_bindings_remain_visible_after_the_loop() {
    let source = "(function(){\
        for (var t=[], s=1; s<arguments.length; s++) t[s-1]=arguments[s];\
        for (var r=0, i=t.length, a; r<i; r++) {}\
        return t[0] + ':' + a;\
    })(0,7);";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("7:undefined".into())
    );
}

#[test]
fn var_declarations_hoist_out_of_unexecuted_control_flow() {
    let source = "(function(flag){\
        if (flag) for (var a=1; false;) {}\
        return a;\
    })(false);";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Undefined);
}

#[test]
fn var_initializers_inside_blocks_assign_the_hoisted_binding() {
    let source = "(function(){ if (true) { var s=function(){return 7;} }\
        return typeof s + ':' + s(); })();";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("function:7".into())
    );
}

#[test]
fn switch_cases_support_fallthrough_and_break() {
    let source = "let out=''; switch('a'){case 'a': out=out+'a';\
        case 'b': out=out+'b'; break; default: out='x';} out;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::String("ab".into()));
}

#[test]
fn control_flow_conditions_accept_comma_sequences() {
    let source = "let out=0; if (out=1, true) out=out+1;\
        for (var i=0; i<1, i<2; i++, out=out+1) {} out;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(4.0));
}

#[test]
fn arithmetic_compound_assignments_work() {
    let source = "let n=1; n+=4; n-=1; n*=3; n/=2; n%=5; n;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(1.0));
}

#[test]
fn bitwise_compound_assignments_work() {
    let source = "let n=8; n>>>=1; n<<=1; n&=14; n|=1; n^=3; n;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(10.0));
}

#[test]
fn expression_for_initializers_accept_comma_sequences() {
    let source = "let i=0; let j=0; let out=0;\
        for (i=1,j=1; i<3; i++) out=out+j; out;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(2.0));
}

#[test]
fn throw_accepts_comma_sequences() {
    let err = js::eval("let t=''; throw t='x', Error('bad '+t);").unwrap_err();

    assert!(err.contains("bad x"));
}

#[test]
fn labels_and_labeled_breaks_parse() {
    let source = "let n=0; outer: for (; n<3;) { n++; break outer; } n;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(1.0));
}

#[test]
fn labeled_break_escapes_switch_blocks() {
    let source = "let out='start'; exit: switch ('outer') { default:\
        switch ('provider') { case 'provider': out='matched'; break exit; }\
        out='bad'; } out;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("matched".into())
    );
}

#[test]
fn exponent_number_literals_parse() {
    assert_eq!(
        js::eval("1e3 / 4 + 2.5e1;").unwrap(),
        js::JsValue::Number(275.0)
    );
}

#[test]
fn leading_dot_number_literals_parse() {
    assert_eq!(
        js::eval("let o={relevance:.2}; o.relevance + .3;").unwrap(),
        js::JsValue::Number(0.5)
    );
}

#[test]
fn ternary_leading_dot_number_does_not_parse_as_optional_chain() {
    assert_eq!(
        js::eval("let m=2; m=m>=1?.5:m; m;").unwrap(),
        js::JsValue::Number(0.5)
    );
}

#[test]
fn array_elisions_parse_as_undefined_slots() {
    assert_eq!(
        js::eval("let a=[,'x',,]; a.length + ':' + a[0] + ':' + a[1] + ':' + a[2];").unwrap(),
        js::JsValue::String("3:undefined:x:undefined".into())
    );
}

#[test]
fn of_is_contextual_and_can_be_bound_as_an_identifier() {
    assert_eq!(
        js::eval("let of=1; for (let x of [2]) { of = of + x; } of;").unwrap(),
        js::JsValue::Number(3.0)
    );
}

#[test]
fn var_declarations_are_available_during_their_own_initializer() {
    assert_eq!(
        js::eval("var OQe=function(e){e.UseBlocker='x'; return e}(OQe||{}); OQe.UseBlocker;")
            .unwrap(),
        js::JsValue::String("x".into())
    );
}

#[test]
fn try_catch_finally_and_do_while_work() {
    let source = "let out=''; try { throw Error('x'); }\
        catch (e) { out='caught'; } finally { out=out+'!'; }\
        let i=0; do i++; while(i<2); out + i;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("caught!2".into())
    );
}

#[test]
fn catch_binding_is_optional() {
    assert_eq!(
        js::eval("let ok=0; try { throw Error('x'); } catch { ok=1; } ok;").unwrap(),
        js::JsValue::Number(1.0)
    );
}

#[test]
fn keyword_property_names_include_return() {
    assert_eq!(
        js::eval("let node={continue:2}; node.return={tag:3}; node.continue + node.return.tag;")
            .unwrap(),
        js::JsValue::Number(5.0)
    );
}

#[test]
fn numeric_object_literal_keys_work() {
    assert_eq!(
        js::eval("let m={8:'x'}; m[8];").unwrap(),
        js::JsValue::String("x".into())
    );
}

#[test]
fn instanceof_parses_as_comparison_operator() {
    assert_eq!(
        js::eval("let value={}; value instanceof Object;").unwrap(),
        js::JsValue::Bool(true)
    );
}

#[test]
fn delete_removes_object_properties() {
    assert_eq!(
        js::eval("let value={x:1}; delete value.x; 'x' in value;").unwrap(),
        js::JsValue::Bool(false)
    );
}

#[test]
fn function_declarations_are_hoisted_in_blocks() {
    let source = "(function(){ let value=call(); function call(){ return 7; } return value; })();";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(7.0));
}

#[test]
fn functions_have_mutable_properties() {
    let source = "function F(){} F.prototype.x=1; F.extra=2; F.prototype.x + F.extra;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(3.0));
}

#[test]
fn object_and_function_helpers_cover_bundle_patterns() {
    let source = "let o={a:1}; let h=Object.prototype.hasOwnProperty;\
        let keys=Object.keys(Object.assign({b:2}, o));\
        let applied=Object.assign.apply(null,[{}, {c:3}]);\
        h.call(o,'a') + ':' + Array.isArray(keys) + ':' + Object.freeze(o).a + ':' + applied.c;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("true:true:1:3".into())
    );
}

#[test]
fn object_prototype_to_string_reports_browser_type_tags() {
    let source = "let object={}; object[Symbol.toStringTag]='Module';\
        Object.prototype.toString.call([]) + ':' +\
        Object.prototype.toString.call(object) + ':' +\
        Object.prototype.propertyIsEnumerable.call({a:1}, 'a');";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("[object Array]:[object Module]:true".into())
    );
}

#[test]
fn error_stack_and_string_helpers_cover_react_probe() {
    let source = "let out=''; try { throw Error('x'); } catch (e) {\
        out=e.stack.trim().split('x')[0].includes('Error') + ':' + (e.stack.match(/x/)===null);\
    } out;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("true:true".into())
    );
}

#[test]
fn string_case_methods_work() {
    assert_eq!(
        js::eval("'Ab'.toLowerCase()+':'+ 'Ab'.toUpperCase();").unwrap(),
        js::JsValue::String("ab:AB".into())
    );
}

#[test]
fn string_helpers_cover_bundle_tokenizers() {
    let source = "'['.concat('a',']') + ':' + \
        'foobar'.startsWith('foo') + ':' + \
        'foobar'.endsWith('bar') + ':' + \
        'banana'.indexOf('na') + ':' + \
        'banana'.lastIndexOf('na') + ':' + \
        'abcdef'.substr(2,3);";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("[a]:true:true:2:4:cde".into())
    );
}

#[test]
fn string_index_helpers_work() {
    assert_eq!(
        js::eval(
            "'Az'[0]+':'+ 'Az'.charAt(1)+':'+ 'Az'.charCodeAt(0)+':'+ 'Realm'.substring(1,4);"
        )
        .unwrap(),
        js::JsValue::String("A:z:65:eal".into())
    );
}

#[test]
fn math_global_covers_bundle_helpers() {
    let source = "Math.max(1,4,2)+':'+Math.min(1,4,2)+':'+Math.floor(1.8)\
        +':'+Math.ceil(1.2)+':'+Math.pow(2,3)+':'+(Math.random()>=0&&Math.random()<1);";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("4:1:1:2:8:true".into())
    );
}

#[test]
fn string_constructor_exposes_static_character_helpers() {
    assert_eq!(
        js::eval("String.fromCharCode(82,101)+':'+String.fromCodePoint(97);").unwrap(),
        js::JsValue::String("Re:a".into())
    );
}

#[test]
fn object_instances_expose_has_own_property() {
    assert_eq!(
        js::eval("let o={a:1}; o.hasOwnProperty('a')+':'+o.hasOwnProperty('b');").unwrap(),
        js::JsValue::String("true:false".into())
    );
}

#[test]
fn array_concat_flattens_array_arguments_once() {
    assert_eq!(
        js::eval("['a'].concat(['b','c'],'d').join('');").unwrap(),
        js::JsValue::String("abcd".into())
    );
}

#[test]
fn array_sort_mutates_with_default_and_comparator_order() {
    let source = "let a=['width','any-hover','height'];\
        let first=a.sort().join(',');\
        let b=[3,1,2];\
        first + '|' + b.sort((x,y)=>x-y).reverse().join(',');";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("any-hover,height,width|3,2,1".into())
    );
}

#[test]
fn array_iteration_and_mutation_helpers_cover_bundle_methods() {
    let source = "let a=[1,2,3,4];\
        let reduced=a.reduce(function(acc,x){return acc+x;},0);\
        let filtered=a.filter(function(x){return x%2===0;}).join(',');\
        let found=a.find(function(x){return x>2;});\
        let checks=a.some(function(x){return x===3;})+':'+a.every(function(x){return x>0;});\
        a.unshift(0); let shifted=a.shift(); let spliced=a.splice(1,2,'x','y').join(',');\
        reduced+'|'+filtered+'|'+found+'|'+checks+'|'+shifted+'|'+spliced+'|'+a.join(',');";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("10|2,4|3|true:true|0|2,3|1,x,y,4".into())
    );
}

#[test]
fn array_iterator_methods_support_from_and_spread() {
    let source = "let keys=Array.from(Array(4).keys()).join(',');\
        let values=Array.from(['a','b'].values()).join(',');\
        let entry=Array.from(['x','y'].entries())[1].join(':');\
        let spread=[...Array(3).keys()].join(',');\
        keys + '|' + values + '|' + entry + '|' + spread;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("0,1,2,3|a,b|1:y|0,1,2".into())
    );
}

#[test]
fn arrays_accept_non_index_own_properties() {
    let source = "let a=[]; Object.defineProperty(a,'raw',{value:['x']});\
        a.named=3; a[0]='z';\
        a.raw[0] + ':' + a.named + ':' + ['raw'].includes('raw') + ':' + a.hasOwnProperty('0');";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("x:3:true:true".into())
    );
}

#[test]
fn array_prototype_slice_call_handles_array_like_receivers() {
    let source = "(function(){\
        return Array.prototype.slice.call(arguments,1).join(',');\
    })(0,'a','b');";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::String("a,b".into()));
}

#[test]
fn native_constructor_prototypes_accept_bundle_polyfills() {
    let source = "String.prototype.left=function(n){return this.substr(0,n);};\
        Object.defineProperty(Array.prototype,'find',{value:function(cb){\
            for(var i=0;i<this.length;i++){if(cb(this[i])) return this[i];}\
        }});\
        'abcd'.left(2)+':'+[1,2,3].find(function(x){return x>1;});";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("ab:2".into())
    );
}

#[test]
fn number_primitives_expose_to_string() {
    assert_eq!(
        js::eval("Math.floor(12.8).toString(36)+':'+(4).valueOf();").unwrap(),
        js::JsValue::String("c:4".into())
    );
}

#[test]
fn string_padding_covers_hex_bundle_helpers() {
    assert_eq!(
        js::eval("(15).toString(16).padStart(4,'0') + ':' + 'x'.padEnd(3,'.');").unwrap(),
        js::JsValue::String("000f:x..".into())
    );
}

#[test]
fn date_global_covers_timestamp_bundle_helpers() {
    let source = "let fixed=new Date(1234);\
        (Date.now() > 0) + ':' + fixed.getTime() + ':' + fixed.valueOf() + ':' +\
        fixed.getTimezoneOffset() + ':' + typeof fixed.toISOString();";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("true:1234:1234:0:string".into())
    );
}

#[test]
fn functions_receive_arguments_binding() {
    let source = "function f(){return arguments.length+':'+arguments[0];}\
        let o={m:function(){return arguments[1];}}; f('x',2)+':'+o.m(1,'y');";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("2:x:y".into())
    );
}

#[test]
fn object_destructuring_declarations_bind_aliases() {
    let source = "let src={location:{pathname:'/login', search:'?x', hash:'#h'}};\
        let {pathname:a, search:o, missing:m='fallback'}=src.location,\
        {hash:h}=src.location, tail='!'; a+o+h+m+tail;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("/login?x#hfallback!".into())
    );
}

#[test]
fn nested_object_destructuring_binds_leaf_aliases() {
    let source = "const {callback:i,options:{bubbles:a=false,args:o=[],checker:l=null}}=\
        {callback:'cb',options:{args:[1]}}; i+':'+a+':'+o.length+':'+l;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("cb:false:1:null".into())
    );
}

#[test]
fn object_destructuring_accepts_nested_array_bindings() {
    let source = "const {div:r,parentDimensions:[i,a]}={div:'d',parentDimensions:[2,3]};\
        r+':'+i+':'+a;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("d:2:3".into())
    );
}

#[test]
fn array_destructuring_declarations_bind_rest() {
    let source = "let [head,,third='x',...tail]=['a','b'], suffix=tail.length;\
        head+third+suffix;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::String("ax0".into()));
}

#[test]
fn array_destructuring_accepts_nested_object_bindings() {
    let source = "let [{x:i,y:a,pageIndex:o},l]=[{x:1,y:2,pageIndex:3},4];\
        i+a+o+l;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(10.0));
}

#[test]
fn mixed_const_declarations_accept_array_and_object_destructuring() {
    let source =
        "const M=1,K=2,[Q,D]=[3,4],{data:ee,isLoading:W,error:U}={data:5,isLoading:6,error:7};\
        M+K+Q+D+ee+W+U;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(28.0));
}

#[test]
fn array_destructuring_parameters_bind_items() {
    assert_eq!(
        js::eval("[['a',1]].map(([name,value])=>name+':'+value).join(',');").unwrap(),
        js::JsValue::String("a:1".into())
    );
}

#[test]
fn function_default_parameters_apply_for_undefined_args() {
    let source = "function f(a=2,b=3){return a+b;} f(undefined,4)+':'+f(5);";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::String("6:8".into()));
}

#[test]
fn function_constructor_builds_callable_functions() {
    let source = "let f=Function('a','b','return a+b'); typeof Function + ':' + f(2,3);";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("function:5".into())
    );
}

#[test]
fn object_rest_parameters_apply_default_values() {
    let source = "function f({...t}={a:1}){ return t.a; } f()+':'+f({a:2});";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::String("1:2".into()));
}

#[test]
fn exponentiation_operator_is_right_associative() {
    assert_eq!(js::eval("2**3**2;").unwrap(), js::JsValue::Number(512.0));
}

#[test]
fn async_arrow_functions_parse_as_functions() {
    let source = "let f=async()=>{ return await 3; }; let g=async(x)=>x+1;\
        let h=({value:v})=>v+1; f()+g(4)+h({value:2});";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(11.0));
}

#[test]
fn await_unwraps_fulfilled_promise_like_values() {
    let source = "let f=async()=>{ let r=await Promise.resolve({data:5}); return r.data; }; f();";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(5.0));
}

#[test]
fn async_function_declarations_are_hoisted() {
    let source = "let before=typeof W5; async function W5(e,t='text'){ return t; }\
        before + ':' + typeof W5;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("function:function".into())
    );
}

#[test]
fn generator_function_markers_parse_as_functions() {
    let source = "let a=function*(){return 1}; let b=async function*(){return 2};\
        typeof a + ':' + typeof b;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("function:function".into())
    );
}

#[test]
fn generator_method_markers_parse_as_methods() {
    let source = "class Box { *items(){return 1} } let o={*items(){return 2}};\
        typeof (new Box()).items + ':' + typeof o.items;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("function:function".into())
    );
}

#[test]
fn yield_expressions_parse_inside_generator_bodies() {
    let source = "let a=function*(x){ yield x; }; class Box { *items(x){ return (yield x); } }\
        typeof a + ':' + typeof (new Box()).items;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("function:function".into())
    );
}
