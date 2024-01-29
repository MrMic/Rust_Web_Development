## Get questions
GET http://localhost:3030/questions

# ______________________________________________________________________
### REGISTRATION 
POST http://localhost:3030/registration
content-type: application/json

{
  "email": "mic@email.com",
  "password": "pepere"
}


POST http://localhost:3030/registration
Content-Type: application/json
{
  "email": "test@email.com",
  "password": "cleartext"
}

# ______________________________________________________________________
## AUTHENTICATION
POST http://localhost:3030/login
Content-Type: application/json
{
  "email": "test@email.com",
  "password": "cleartext"
}


## POST the authentication token
POST http://localhost:3030/login
Content-Type: application/json
{
  "email": "mic@email.com",
  "password": "pepere"
}

# ______________________________________________________________________
## CREATE FIRST QUESTION (N°6)
POST http://localhost:3030/questions
Authorization: v2.local.hgHpiAmHrMGR-KLpqZnzT1xJM8nyEb9P9aCFcP6RpjFyFeZ7qtzIMICQjxBcTHRjGj2PKEsVDMVJOpI5w7TVvU4w8ov0FBpCaJtBwwmW53ScT_k2ii4HMAEIqUKjN1RvO5rJMkl8jxE-Cb1oRW-LIanDC6tFZHSzelkjoOhiGOL8gvgstys
Content-Type: application/json
{
  "title": "How can I code better?",
  "content": "Any tips for a junior developer?",
  "tags": ["general", "programming"]
}


# ______________________________________________________________________
## UPDATE QUESTION
PUT http://localhost:3030/questions/6
Authorization: v2.local.hgHpiAmHrMGR-KLpqZnzT1xJM8nyEb9P9aCFcP6RpjFyFeZ7qtzIMICQjxBcTHRjGj2PKEsVDMVJOpI5w7TVvU4w8ov0FBpCaJtBwwmW53ScT_k2ii4HMAEIqUKjN1RvO5rJMkl8jxE-Cb1oRW-LIanDC6tFZHSzelkjoOhiGOL8gvgstys
Content-Type: application/json
{
  "id": 6,
  "title": "New Title for question 6",
  "content": "Any tips for a junior developer?"
}
