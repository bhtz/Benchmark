package models

type User struct {
	Id        string `json:"id"`
	Firstname string `json:"firstname"`
	Lastname  string `json:"lastname"`
	Email     string `json:"email"`
}

func NewUser(id string, firstname string, lastname string, email string) *User {
	user := new(User)
	user.Id = id
	user.Firstname = firstname
	user.Lastname = lastname
	user.Email = email
	return user
}
