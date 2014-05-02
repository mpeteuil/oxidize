//! A very simplistic router that matches over a url and method and calls the handler
//! Very useful for applications that don't need variables or reverse routing or wildcards
//! (or anything useful for that matter) One potential use case is for web framework benchmarks :)
use std::vec::Vec;
use router::RouteInfo;

struct MatchRouter {
    pub routes : Arc<RWLock<Vec<(&'static str, &'static str, &'static str)>>>,
}

impl Router for MatchRouter {
    fn add_route(&self, method: &'static str, name: &'static str, url: &'static str){
        self.routes.write().push((method, name, url));
    }
    fn route<'a>(&self, method: &'a str, url: &'a str) -> RouteInfo<'a> {
        let (match_method, match_url) = (method, url);
        for r in routes.read() {
            let (route_method, route_url, route_name) = r;
            if route_method == match_method && route_url == match_url {
                return RouteInfo<'a> {
                    name: route_name,
                    method: route_method,
                    params: None,
                }
            }
        }
        // 404 if not found. (TODO)
        unreachable!();
    }
    
    fn copy(&self) -> ~Router:Send+Share {
        ~MatchRouter {
            routes: self.routes.clone(),
        } as ~Router:Send+Share
    }
}