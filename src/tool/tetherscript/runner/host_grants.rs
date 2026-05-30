use std::rc::Rc;

use tetherscript::browser_cap::BrowserAuthority;
use tetherscript::capability::Authority;
use tetherscript::plugin::PluginHost;

use super::browser::BrowserGrant;
use super::computer::{ComputerAuthority, ComputerGrant};

pub fn grant_browser(host: &mut PluginHost, browser: BrowserGrant) {
    if let Some(endpoint) = &browser.endpoint {
        host.grant("browser", browser_authority(endpoint, &browser));
    }
}

pub fn grant_computer(host: &mut PluginHost, computer: ComputerGrant) {
    if computer.enabled {
        host.grant("computer", computer_authority(computer));
    }
}

fn browser_authority(endpoint: &str, browser: &BrowserGrant) -> Rc<dyn Authority> {
    let scopes = if browser.scopes.is_empty() {
        BrowserAuthority::all_scopes()
    } else {
        browser.scopes.clone()
    };
    BrowserAuthority::new(endpoint, browser.origins.clone(), scopes)
}

fn computer_authority(computer: ComputerGrant) -> Rc<dyn Authority> {
    let scopes = if computer.scopes.is_empty() {
        ComputerAuthority::all_scopes()
    } else {
        computer.scopes
    };
    ComputerAuthority::new(computer.origins, scopes)
}
