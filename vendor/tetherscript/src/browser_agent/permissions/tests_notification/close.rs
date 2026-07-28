use super::*;

#[test]
fn notification_close_dispatches_handler_and_listener() {
    let script = "let n=new Notification('Hi');let log='';n.onclose=function(e){log=log+'h:'+e.type+':'+(this===n)+';';};n.addEventListener('close',function(e){log=log+'l:'+e.target.title+':'+this.closed;});n.close();log;";
    assert_eq!(
        value(page(PermissionState::Granted), script),
        "h:close:true;l:Hi:true"
    );
}
