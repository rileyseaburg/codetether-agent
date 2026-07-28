//! Shared route-table storage for page runtimes.

use std::cell::RefCell;
use std::rc::Rc;

use super::RouteTable;

pub(crate) type SharedRouteTable = Rc<RefCell<RouteTable>>;

pub(crate) fn shared_route_table() -> SharedRouteTable {
    Rc::new(RefCell::new(RouteTable::default()))
}
