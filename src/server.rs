use rouille::{Response, router};

pub fn start(loc: &str) -> () {
    rouille::start_server(loc, move |request| {
        router!(request,
            (GET) (/all-cocktails) => {
                Response::text("Yay")
            },

            _ => Response::empty_404()
        )
    });
}
