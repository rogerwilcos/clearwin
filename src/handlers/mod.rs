use crate::utils::http_get;

pub fn get_user_commits(username: String) {
    println!("git username: {}", username);
    let uri = "https://api.github.com/users/".to_owned() + &username;
    let _ = http_get(uri.as_str());
}

pub fn get_user_info(username: String) {
    //     curl -L \
    //   -H "Accept: application/vnd.github+json" \
    //   -H "Authorization: Bearer <YOUR-TOKEN>" \
    //   -H "X-GitHub-Api-Version: 2022-11-28" \
    //   https://api.github.com/users/USERNAME
}
