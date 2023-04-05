use crate::types::Routes;

// Receives the stream of the request and with the method and route filter the created routes
pub fn get_filtered_routes<'a>(routes: Vec<Routes<'a>>, parts_of_req: Vec<&str>) -> Vec<Routes<'a>> {
    let route = parts_of_req[1];
    let method = parts_of_req[0];

    // route 
    let filtered_routes: Vec<Routes<'_>> = routes
        .into_iter()
        .filter(|voc| voc.method == method.to_string())
        .filter(|voc| voc.route == route.to_string())
        .collect();

    return filtered_routes;
}
