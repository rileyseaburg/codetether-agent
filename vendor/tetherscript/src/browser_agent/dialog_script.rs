//! JavaScript bridge snippets for deterministic dialogs.

use crate::browser_agent::dialog::{DialogDecision, DialogKind};

pub(crate) fn install(decisions: &[DialogDecision]) -> String {
    format!(
        "{}window.__agentDialogDecisions={};window.__agentDialogDecisionIndex=0;let alert=window.alert;let confirm=window.confirm;let prompt=window.prompt;",
        BOOT,
        array(decisions)
    )
}

pub(crate) fn drain() -> &'static str {
    "window.__agentDialogDrainValue=[window.__agentDialogDecisionIndex,window.__agentDialogs];window.__agentDialogs=[];window.__agentDialogDrainValue;"
}

pub(crate) fn kind(value: &str) -> Option<DialogKind> {
    match value {
        "alert" => Some(DialogKind::Alert),
        "confirm" => Some(DialogKind::Confirm),
        "prompt" => Some(DialogKind::Prompt),
        _ => None,
    }
}

fn array(decisions: &[DialogDecision]) -> String {
    let items = decisions.iter().map(decision).collect::<Vec<_>>().join(",");
    format!("[{}]", items)
}

fn decision(decision: &DialogDecision) -> String {
    match decision {
        DialogDecision::Accept => "['accept',null]".into(),
        DialogDecision::Dismiss => "['dismiss',null]".into(),
        DialogDecision::Prompt(value) => format!("['prompt',{}]", quote(value)),
    }
}

fn quote(value: &str) -> String {
    let mut out = String::from("'");
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '\'' => out.push_str("\\'"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out.push('\'');
    out
}

const BOOT: &str = "if(!window.__agentDialogInstalled){window.__agentDialogs=[];window.__agentDialogDecisions=[];window.__agentDialogDecisionIndex=0;window.__agentDialogPick=function(){let i=window.__agentDialogDecisionIndex;window.__agentDialogDecisionIndex=i+1;if(i<window.__agentDialogDecisions.length){return window.__agentDialogDecisions[i];}return ['dismiss',null];};window.__agentRecordDialog=function(kind,message,def){let d=window.__agentDialogPick();let action=d[0];let value=d[1];let accepted=action!='dismiss';let response=null;if(kind=='prompt'&&accepted){if(action=='prompt'){response=value;}else{response=def;}}window.__agentDialogs.push([kind,''+message,def,accepted,response]);if(kind=='confirm'){return accepted;}if(kind=='prompt'){return response;}return undefined;};window.alert=function(message){return window.__agentRecordDialog('alert',message,null);};window.confirm=function(message){return window.__agentRecordDialog('confirm',message,null);};window.prompt=function(message,def){return window.__agentRecordDialog('prompt',message,def);};window.__agentDialogInstalled=true;}";
