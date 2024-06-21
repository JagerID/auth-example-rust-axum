use super::{dto::FilteredUser, model::User};

pub fn filter_user(user: &User) -> FilteredUser {
    FilteredUser {
        id: user.id.to_owned(),
        name: user.name.to_owned(),
        email: user.email.to_owned(),
        role: user.role.to_owned(),
        photo: user.photo.to_owned(),
        created_at: user.created_at.to_owned(),
        updated_at: user.updated_at.to_owned(),
    }
}

pub fn filter_users(users: &Vec<User>) -> Vec<FilteredUser> {
    users.iter().map(filter_user).collect()
}
