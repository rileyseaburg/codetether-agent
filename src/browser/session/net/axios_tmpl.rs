//! JS template for axios replay.

use crate::browser::{BrowserError, request::AxiosRequest};

/// Build the in-page axios replay script from the request fields.
pub(super) fn build_script(request: AxiosRequest) -> Result<String, BrowserError> {
    let body = serde_json::to_string(&request.body.unwrap_or(serde_json::Value::Null))?;
    let headers = serde_json::to_string(&request.headers.unwrap_or_default())?;
    let url = serde_json::to_string(&request.url)?;
    let method = request.method.to_lowercase();
    let body_methods = matches!(method.as_str(), "post" | "put" | "patch");
    let path = serde_json::to_string(
        &request
            .axios_path
            .unwrap_or_else(|| "__autodetect__".to_string()),
    )?;
    let method_json = serde_json::to_string(&method)?;
    Ok(TEMPLATE
        .replace("__PATH__", &path)
        .replace("__URL__", &url)
        .replace("__BODY__", &body)
        .replace("__HEADERS__", &headers)
        .replace("__METHOD__", &method_json)
        .replace(
            "__BODY_METHODS__",
            if body_methods { "true" } else { "false" },
        ))
}

const TEMPLATE: &str = r#"(async () => {
  const needle = __PATH__;
  const findAxios = () => { if (needle !== '__autodetect__') {
    try { const parts = needle.replace(/^window\./,'').split('.'); let obj = window;
      for (const p of parts) obj = obj && obj[p];
      if (obj && typeof obj.request === 'function') return { instance: obj, path: needle }; } catch (_) {} return null; }
    const seen = new Set(), candidates = [];
    const push = (obj, path) => { if (!obj || typeof obj !== 'object' || seen.has(obj)) return; seen.add(obj);
      if (typeof obj.request === 'function' && obj.defaults && typeof obj.defaults === 'object')
        candidates.push({ instance: obj, path, hasBaseURL: !!obj.defaults.baseURL }); };
    try { if (window.axios) push(window.axios, 'window.axios');
      for (const k of Object.keys(window)) { try { push(window[k], 'window.'+k); } catch (_) {} } } catch (_) {}
    candidates.sort((a,b) => Number(b.hasBaseURL) - Number(a.hasBaseURL)); return candidates[0] || null; };
  const found = findAxios();
  if (!found) return { ok: false, status: 0, error: 'no axios instance on window', axios_path: null };
  const ax = found.instance, url = __URL__, body = __BODY__, headers = __HEADERS__, config = { headers };
  try { let resp; const method = __METHOD__;
    if (method==='get') resp = await ax.get(url,config); else if (method==='delete') resp = await ax.delete(url,config);
    else if (method==='head') resp = await ax.head(url,config); else if (__BODY_METHODS__) resp = await ax[method](url,body,config);
    else resp = await ax.request({method,url,data:body,headers});
    return {ok:true,status:resp.status,status_text:resp.statusText,headers:resp.headers||{},data:resp.data,axios_path:found.path};
  } catch (err) { const resp = err && err.response;
    return {ok:false,status:resp?resp.status:0,status_text:resp?resp.statusText:String((err&&err.message)||err),headers:(resp&&resp.headers)||{},data:resp&&resp.data,error:String((err&&err.stack)||(err&&err.message)||err),axios_path:found.path}; }
})()"#;
