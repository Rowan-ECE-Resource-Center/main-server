table! {
    departments (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        abbreviation -> Varchar,
    }
}
table!{
	user_departments (id) {
		id -> Unsigned<Bigint>,
		user_id -> Unsigned<Bigint>,
		department_id -> Unsigned<Bigint>,
	}
}
