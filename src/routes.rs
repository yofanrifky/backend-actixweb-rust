use actix_web::middleware::from_fn;
use actix_web::web;

use crate::{
    auth_middleware::{
        require_admin, require_login, require_menu_access, require_order_access,
        require_payment_access,
    },
    handlers::{auth, menus, orders, payments},
};

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .service(
                web::scope("/menus")
                    .wrap(from_fn(require_menu_access))
                    .route("", web::get().to(menus::get_menus))
                    .route("", web::post().to(menus::create_menu))
                    .route("/{id}", web::get().to(menus::get_menu))
                    .route("/{id}", web::put().to(menus::update_menu))
                    .route("/{id}", web::patch().to(menus::patch_menu))
                    .route("/{id}", web::delete().to(menus::delete_menu)),
            )
            .service(
                web::scope("/orders")
                    .wrap(from_fn(require_order_access))
                    .service(
                        web::resource("/{id}/payment")
                            .wrap(from_fn(require_payment_access))
                            .route(web::post().to(payments::create_payment)),
                    )
                    .service(
                        web::resource("/{id}/receipt")
                            .wrap(from_fn(require_payment_access))
                            .route(web::get().to(payments::get_receipt)),
                    )
                    .route("", web::get().to(orders::get_orders))
                    .route("", web::post().to(orders::create_order))
                    .route("/{id}", web::get().to(orders::get_order))
                    .route("/{id}", web::put().to(orders::update_order))
                    .route("/{id}", web::patch().to(orders::patch_order))
                    .route("/{id}", web::delete().to(orders::delete_order)),
            )
            .service(
                web::scope("/payments")
                    .wrap(from_fn(require_admin))
                    .route("", web::get().to(payments::list_payments))
                    .route("/{id}/confirm", web::post().to(payments::confirm_payment)),
            )
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(auth::register))
                    .route("/login", web::post().to(auth::login))
                    .route("/logout", web::post().to(auth::logout))
                    .service(
                        web::resource("/users/{id}/role")
                            .wrap(from_fn(require_admin))
                            .route(web::put().to(auth::update_user_role)),
                    )
                    .service(
                        web::resource("/me")
                            .wrap(from_fn(require_login))
                            .route(web::get().to(auth::me)),
                    ),
            ),
    );
}
