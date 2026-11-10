use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Kasir,
    Pembeli,
}

impl Role {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::Kasir => "kasir",
            Self::Pembeli => "pembeli",
        }
    }

    pub fn from_database(value: &str) -> Option<Self> {
        match value {
            "admin" => Some(Self::Admin),
            "kasir" => Some(Self::Kasir),
            "pembeli" => Some(Self::Pembeli),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Role;

    #[test]
    fn role_database_values_are_strict() {
        assert_eq!(Role::from_database("admin"), Some(Role::Admin));
        assert_eq!(Role::from_database("kasir"), Some(Role::Kasir));
        assert_eq!(Role::from_database("pembeli"), Some(Role::Pembeli));
        assert_eq!(Role::from_database("owner"), None);
    }
}