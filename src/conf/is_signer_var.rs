
pub fn check_is_signer_name(var_name: String) -> bool {

    let auth_names = get_is_signer_vars();
    for auth_name in auth_names {
        if var_name.contains(&auth_name) {
            return true;
        }
    }
    false
}

pub fn get_is_signer_vars() -> Vec<String> {
    return Vec::from([
        // "authority_info".to_string(),
        "authority".to_string(),
        "owner".to_string(),
        "admin".to_string(),
        "manager".to_string(),
        // "admin_acc".to_string(),
        // "owner_acc".to_string(),
    ])
}


pub fn get_blacklist() -> Vec<String>{
    return Vec::from([
        "anchor_lang".to_string(),
        "__idl".to_string(),
        "#[program]".to_string(),
    ])
}

pub fn in_blacklist(var_name: String) -> bool {
    
    let auth_names = get_blacklist();
    for auth_name in auth_names {
        if var_name.contains(&auth_name) {
            return true;
        }
    }
    false
}



pub fn get_signer_filter_vars() -> Vec<String>{
    return Vec::from([
        "::try_accounts".to_string(),
    ])
}

pub fn is_signer_var_filter(var_name: String) -> bool {
    let auth_names = get_signer_filter_vars();
    for auth_name in auth_names {
        if var_name.contains(&auth_name) {
            return true;
        }
    }
    false
}

