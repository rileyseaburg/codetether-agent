use super::*;

#[test]
fn webgl_context_records_metadata_commands_and_parameters() {
    let result = eval_with_dom(
        "<canvas id='c' width='8' height='4'></canvas>",
        "let gl=document.getElementById('c').getContext('webgl'); \
         gl.viewport(1,2,3,4); gl.clearColor(0.25,0.5,0.75,1); gl.clear(gl.COLOR_BUFFER_BIT); \
         gl.getParameter(gl.VENDOR)+':'+gl.getParameter(gl.VIEWPORT).join(',')+':'+\
         gl.getSupportedExtensions().join('|');",
    )
    .unwrap();
    assert_eq!(
        result.value.display(),
        concat!(
            "tetherscript deterministic WebGL:1,2,3,4:",
            "ANGLE_instanced_arrays|OES_element_index_uint|",
            "OES_standard_derivatives|OES_texture_float|WEBGL_debug_renderer_info"
        )
    );
    let attrs = match &result.document.children[0] {
        Node::Element(el) => &el.attrs,
        _ => panic!("expected canvas"),
    };
    assert_eq!(
        attrs.get("data-agent-webgl-commands").map(String::as_str),
        Some("viewport|1|2|3|4;clearColor|0.25|0.5|0.75|1;clear|16384")
    );
}
