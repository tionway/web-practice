use crate::errors::TutorError;
use crate::models::tutor::{NewTutor, Tutor, UpdateTutor};
use sqlx::postgres::PgPool;

pub async fn get_all_tutors_db(pool: &PgPool) -> Result<Vec<Tutor>, TutorError> {
    let tutor_rows =
        sqlx::query!("SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile FROM tutor_practice")
            .fetch_all(pool)
            .await?;

    let tutors: Vec<Tutor> = tutor_rows
        .iter()
        .map(|tutor_row| Tutor {
            tutor_id: tutor_row.tutor_id,
            tutor_name: tutor_row.tutor_name.clone(),
            tutor_pic_url: tutor_row.tutor_pic_url.clone(),
            tutor_profile: tutor_row.tutor_profile.clone(),
        })
        .collect();
    match tutors.len() {
        0 => Err(TutorError::NotFound("No tutors found".into())),
        _ => Ok(tutors),
    }
}

pub async fn get_tutor_details_db(pool: &PgPool, tutor_id: i32) -> Result<Tutor, TutorError> {
    let tutor_row = sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile FROM tutor_practice where tutor_id = $1",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map(|tutor_row| Tutor {
        tutor_id: tutor_row.tutor_id,
        tutor_name: tutor_row.tutor_name,
        tutor_pic_url: tutor_row.tutor_pic_url,
        tutor_profile: tutor_row.tutor_profile,
    })
    .map_err(|_err| TutorError::NotFound("Tutor id not found".into()))?;
    Ok(tutor_row)
}

pub async fn post_new_tutor_db(pool: &PgPool, new_tutor: NewTutor) -> Result<Tutor, TutorError> {
    let tutor_row = sqlx::query!(
        "insert into tutor_practice (tutor_name, tutor_pic_url, tutor_profile) values ($1, $2, $3) returning tutor_id, tutor_name, tutor_pic_url, tutor_profile",
        new_tutor.tutor_name,
        new_tutor.tutor_pic_url,
        new_tutor.tutor_profile
    )
    .fetch_one(pool)
    .await?;
    //Retrieve result
    Ok(Tutor {
        tutor_id: tutor_row.tutor_id,
        tutor_name: tutor_row.tutor_name,
        tutor_pic_url: tutor_row.tutor_pic_url,
        tutor_profile: tutor_row.tutor_profile,
    })
}

pub async fn update_tutor_details_db(
    pool: &PgPool,
    tutor_id: i32,
    update_tutor: UpdateTutor,
) -> Result<Tutor, TutorError> {
    let current_tutor_row = sqlx::query_as!(
        Tutor,
        "SELECT * FROM tutor_practice where tutor_id = $1",
        tutor_id,
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| TutorError::NotFound("Course id not found".into()))?;

    let tutor_name = update_tutor.tutor_name.unwrap_or(current_tutor_row.tutor_name);
    let tutor_pic_url = update_tutor.tutor_pic_url.unwrap_or(current_tutor_row.tutor_pic_url);
    let tutor_profile = update_tutor.tutor_profile.unwrap_or(current_tutor_row.tutor_profile);

    let tutor_row = sqlx::query_as!(
        Tutor,
        "UPDATE tutor_practice set tutor_name = $1, tutor_pic_url = $2, tutor_profile = $3 returning tutor_id, tutor_name, tutor_pic_url, tutor_profile", 
        tutor_name, 
        tutor_pic_url, 
        tutor_profile
    )
    .fetch_one(pool)
    .await;

    tutor_row.map_err(|_err| TutorError::NotFound("Course id not found".into()))
}

pub async fn delete_tutor_db(pool: &PgPool, tutor_id: i32) -> Result<String, TutorError> {
    let course_row = sqlx::query!("DELETE FROM tutor_practice where tutor_id = $1", tutor_id,)
        .execute(pool)
        .await?;

    Ok(format!("Deleted {:?} record", course_row))
}
