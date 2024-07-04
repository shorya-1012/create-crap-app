use actix_web::{post, web, HttpResponse, Responder};

use crate::{
    schemas::{clerk_webhook_schema::ClerkWebHookEvent, error_json::ErrorJson},
    AppState,
};

#[post("/webhooks/clerk")]
pub async fn clerk_webhook(
    webhook_event: web::Json<ClerkWebHookEvent>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let user_data = &webhook_event.data;
    let email = &webhook_event.data.email_addresses[0].email_address;

    if webhook_event.webhook_type == "user.created" {
        let query = "INSERT INTO user_info VALUES ($1 , $2 , $3, $4)";
        match sqlx::query(query)
            .bind(&user_data.id)
            .bind(format!("{} {}", user_data.first_name, user_data.last_name))
            .bind(email)
            .bind(&user_data.image_url)
            .execute(&app_state.db_pool)
            .await
        {
            Ok(_) => println!("User Created Successfully"),
            Err(err) => {
                println!("{:#?}", err);
                return HttpResponse::InternalServerError().json(ErrorJson {
                    error: err.to_string(),
                });
            }
        };
        return HttpResponse::Created().json("User created successfully");
    } else if webhook_event.webhook_type == "user.deleted" {
        return HttpResponse::Created().json("User created successfully");
    }

    HttpResponse::BadRequest().json(ErrorJson {
        error: "Unrecoginsed webhook event".to_string(),
    })
}
