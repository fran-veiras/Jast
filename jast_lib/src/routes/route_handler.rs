use crate::types::Routes;

/// Filter the route created with Jast that is equal to the request, if not it exists 404.
///
/// # Arguments
///
/// * `parts_of_req` - Method[0] and route[1] of the request.
/// * `routes` - routes created with Jast.
///
/// # Returns
///
/// Struct of Routes (method, route, response)

pub fn get_filtered_routes<'a>(routes: &'a Vec<Routes<'a>>, parts_of_req: Vec<&str>) -> Vec<&'a Routes<'a>> {
    let route = parts_of_req[1];
    let method = parts_of_req[0];

    let filtered_routes: Vec<&Routes<'a>> = routes
        .iter()
        .filter(|voc| voc.method == method.to_string())
        .filter(|voc| voc.route == route.to_string())
        .collect();

    // mandar el 404 desde acá
    // if filtered_routes.len() == 0 {
    //     println!("hola, {:?}", filtered_routes.len());
    // }
    
    filtered_routes
}
