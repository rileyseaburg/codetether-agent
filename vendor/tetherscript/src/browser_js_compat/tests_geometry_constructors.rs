use super::super::super::*;

#[test]
fn dom_point_constructor_static_and_json_work() {
    assert_eval(
        "let a=DOMPoint();let b=new DOMPoint(1,2,3,4);\
         let c=DOMPoint.fromPoint({x:5,y:6,z:7,w:8});let j=c.toJSON();\
         [typeof DOMPoint,window.DOMPoint===DOMPoint,a.x,a.y,a.z,a.w,\
         b.x,b.y,b.z,b.w,c.x,c.y,c.z,c.w,j.x,j.y,j.z,j.w].join('|');",
        "function|true|0|0|0|1|1|2|3|4|5|6|7|8|5|6|7|8",
    );
}

#[test]
fn dom_rect_constructor_static_and_json_work() {
    assert_eval(
        "let a=DOMRect();let b=new DOMRect(1,2,3,4);\
         let c=DOMRect.fromRect({x:5,y:6,width:7,height:8});let j=c.toJSON();\
         [typeof DOMRect,window.DOMRect===DOMRect,a.x,a.y,a.width,a.height,\
         b.x,b.y,b.width,b.height,b.left,b.top,b.right,b.bottom,\
         c.x,c.y,c.width,c.height,j.left,j.top,j.right,j.bottom].join('|');",
        "function|true|0|0|0|0|1|2|3|4|1|2|4|6|5|6|7|8|5|6|12|14",
    );
}

#[test]
fn dom_matrix_constructor_static_and_json_work() {
    assert_eval(
        "let a=DOMMatrix();let b=new DOMMatrix('matrix(1, 2, 3, 4, 5, 6)');\
         let c=DOMMatrix.fromMatrix({a:7,b:8,c:9,d:10,e:11,f:12});let j=c.toJSON();\
         [typeof DOMMatrix,window.DOMMatrix===DOMMatrix,a.isIdentity,b.a,b.b,b.e,b.f,\
         c.m11,c.m12,c.m41,c.m42,j.a,j.f,b.toString()].join('|');",
        "function|true|true|1|2|5|6|7|8|11|12|7|12|matrix(1, 2, 3, 4, 5, 6)",
    );
}

fn assert_eval(script: &str, expected: &str) {
    let result = eval_with_dom("<main></main>", script).unwrap();
    assert_eq!(result.value, JsValue::String(expected.into()));
}
