//! Construct browser and computer grants without expanding requested authority.
use std::rc::Rc;

use tetherscript::browser_cap::BrowserAuthority;
use tetherscript::capability::Authority;
use tetherscript::plugin::PluginHost;

use super::browser::BrowserGrant;
use crate::{ComputerGrant, Host};
use codetether_tetherscript_computer::ComputerAuthority;

pub fn grant_browser(host: &mut PluginHost, browser: BrowserGrant) {
    if let Some(endpoint) = &browser.endpoint {
        host.grant("browser", browser_authority(endpoint, &browser));
    }
}

pub fn grant_computer<H: Host>(host: &mut PluginHost, computer: ComputerGrant) {
    if computer.enabled {
        host.grant("computer", computer_authority::<H>(computer));
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

fn computer_authority<H: Host>(computer: ComputerGrant) -> Rc<dyn Authority> {
    let scopes = if computer.scopes.is_empty() {
        ComputerAuthority::<H>::all_scopes()
    } else {
        computer.scopes
    };
    ComputerAuthority::<H>::new(computer.origins, scopes)
}
